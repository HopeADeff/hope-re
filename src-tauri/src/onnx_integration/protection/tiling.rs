use image::DynamicImage;
use ort::session::Session;
use tauri::Emitter;

use super::preprocessing::{compute_edge_weight_map, preprocess_tile};
use super::spsa::spsa_pgd_on_tile;
use super::types::{
    AlgorithmParams, ModelRunFn, ProtectionProgress, TileProgress, TILE_OVERLAP, TILE_SIZE,
};

struct TileRegion {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

fn blend_tile_direct(
    protected_tile: &ndarray::Array4<f32>,
    region: &TileRegion,
    width: u32,
    result_accum: &mut [f32],
    weight_accum: &mut [f32],
) {
    let ts = TILE_SIZE as usize;
    let no_scale = region.w == TILE_SIZE && region.h == TILE_SIZE;

    for py in 0..region.h {
        for px in 0..region.w {
            let dst_x = region.x + px;
            let dst_y = region.y + py;
            let dst_pixel = (dst_y * width + dst_x) as usize;
            let dst_idx = dst_pixel * 3;

            let edge_x = px.min(region.w - 1 - px).min(TILE_OVERLAP) as f32 / TILE_OVERLAP as f32;
            let edge_y = py.min(region.h - 1 - py).min(TILE_OVERLAP) as f32 / TILE_OVERLAP as f32;
            let weight = edge_x.min(edge_y).max(0.01);

            if dst_idx + 2 >= result_accum.len() {
                continue;
            }

            if no_scale {
                let sx = px as usize;
                let sy = py as usize;
                for c in 0..3 {
                    let v = protected_tile[[0, sy, sx, c]] * 255.0;
                    result_accum[dst_idx + c] += v * weight;
                }
            } else {
                let scale_x = TILE_SIZE as f32 / region.w as f32;
                let scale_y = TILE_SIZE as f32 / region.h as f32;
                let src_fx = px as f32 * scale_x;
                let src_fy = py as f32 * scale_y;

                let x0 = (src_fx as usize).min(ts - 1);
                let y0 = (src_fy as usize).min(ts - 1);
                let x1 = (x0 + 1).min(ts - 1);
                let y1 = (y0 + 1).min(ts - 1);
                let fx = src_fx - x0 as f32;
                let fy = src_fy - y0 as f32;

                for c in 0..3 {
                    let v00 = protected_tile[[0, y0, x0, c]];
                    let v10 = protected_tile[[0, y0, x1, c]];
                    let v01 = protected_tile[[0, y1, x0, c]];
                    let v11 = protected_tile[[0, y1, x1, c]];
                    let v = (v00 * (1.0 - fx) * (1.0 - fy)
                        + v10 * fx * (1.0 - fy)
                        + v01 * (1.0 - fx) * fy
                        + v11 * fx * fy)
                        * 255.0;
                    result_accum[dst_idx + c] += v * weight;
                }
            }

            weight_accum[dst_pixel] += weight;
        }
    }
}

pub fn apply_model_protection(
    img: &DynamicImage,
    session: &mut Session,
    params: &AlgorithmParams,
    iterations: u32,
    run_model: &mut ModelRunFn,
    app: &tauri::AppHandle,
) -> Result<DynamicImage, String> {
    let width = img.width();
    let height = img.height();
    let stride = TILE_SIZE - TILE_OVERLAP;

    let mut result_accum = vec![0.0f32; (width * height * 3) as usize];
    let mut weight_accum = vec![0.0f32; (width * height) as usize];

    let tiles_x = if width <= TILE_SIZE {
        1
    } else {
        ((width - TILE_OVERLAP) as f32 / stride as f32).ceil() as u32
    };
    let tiles_y = if height <= TILE_SIZE {
        1
    } else {
        ((height - TILE_OVERLAP) as f32 / stride as f32).ceil() as u32
    };

    let total_tiles = tiles_x * tiles_y;
    let mut tile_count = 0u32;

    for ty in 0..tiles_y {
        for tx in 0..tiles_x {
            let tile_x = (tx * stride).min(width.saturating_sub(TILE_SIZE));
            let tile_y = (ty * stride).min(height.saturating_sub(TILE_SIZE));
            let tile_w = TILE_SIZE.min(width - tile_x);
            let tile_h = TILE_SIZE.min(height - tile_y);

            let region = TileRegion {
                x: tile_x,
                y: tile_y,
                w: tile_w,
                h: tile_h,
            };

            let base_tile = preprocess_tile(img, region.x, region.y, region.w, region.h);
            let edge_weights = compute_edge_weight_map(&base_tile);

            let protected_tile = spsa_pgd_on_tile(
                session,
                &base_tile,
                params,
                iterations,
                run_model,
                &edge_weights,
                &TileProgress {
                    app: app.clone(),
                    tile_current: tile_count + 1,
                    tile_total: total_tiles,
                },
            )?;

            blend_tile_direct(
                &protected_tile,
                &region,
                width,
                &mut result_accum,
                &mut weight_accum,
            );

            tile_count += 1;
            if cfg!(debug_assertions) {
                log::info!("Processed tile {}/{}", tile_count, total_tiles);
            }
            let percent = (tile_count as f64 / total_tiles as f64 * 95.0).min(95.0);
            let _ = app.emit(
                "protection-progress",
                ProtectionProgress {
                    stage: "processing".to_string(),
                    tile_current: tile_count,
                    tile_total: total_tiles,
                    iteration_current: iterations,
                    iteration_total: iterations,
                    percent,
                },
            );
        }
    }

    let mut result_pixels = vec![0u8; (width * height * 4) as usize];
    let original_rgba = img.to_rgba8();

    for y in 0..height {
        for x in 0..width {
            let pixel_idx = (y * width + x) as usize;
            let rgb_idx = pixel_idx * 3;
            let rgba_idx = pixel_idx * 4;
            let original = original_rgba.get_pixel(x, y);

            if weight_accum[pixel_idx] > 0.0 {
                let w = weight_accum[pixel_idx];
                result_pixels[rgba_idx] = (result_accum[rgb_idx] / w).clamp(0.0, 255.0) as u8;
                result_pixels[rgba_idx + 1] =
                    (result_accum[rgb_idx + 1] / w).clamp(0.0, 255.0) as u8;
                result_pixels[rgba_idx + 2] =
                    (result_accum[rgb_idx + 2] / w).clamp(0.0, 255.0) as u8;
            } else {
                result_pixels[rgba_idx] = original[0];
                result_pixels[rgba_idx + 1] = original[1];
                result_pixels[rgba_idx + 2] = original[2];
            }
            result_pixels[rgba_idx + 3] = original[3];
        }
    }

    image::RgbaImage::from_raw(width, height, result_pixels)
        .map(DynamicImage::ImageRgba8)
        .ok_or_else(|| "Failed to construct result image from pixel data".to_string())
}
