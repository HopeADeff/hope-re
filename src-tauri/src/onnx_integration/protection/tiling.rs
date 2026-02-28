use image::DynamicImage;
use ort::session::Session;
use tauri::Emitter;

use super::preprocessing::{compute_edge_weight_map, preprocess_tile, tile_to_pixels};
use super::spsa::spsa_pgd_on_tile;
use super::types::{
    AlgorithmParams, ModelRunFn, ProtectionProgress, TileProgress, TILE_OVERLAP, TILE_SIZE,
};

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

            let base_tile = preprocess_tile(img, tile_x, tile_y, tile_w, tile_h);
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

            let pixels = tile_to_pixels(&protected_tile, TILE_SIZE, TILE_SIZE);

            let scale_x = TILE_SIZE as f32 / tile_w as f32;
            let scale_y = TILE_SIZE as f32 / tile_h as f32;

            for py in 0..tile_h {
                for px in 0..tile_w {
                    let src_fx = px as f32 * scale_x;
                    let src_fy = py as f32 * scale_y;

                    let x0 = (src_fx as u32).min(TILE_SIZE - 1);
                    let y0 = (src_fy as u32).min(TILE_SIZE - 1);
                    let x1 = (x0 + 1).min(TILE_SIZE - 1);
                    let y1 = (y0 + 1).min(TILE_SIZE - 1);
                    let fx = src_fx - x0 as f32;
                    let fy = src_fy - y0 as f32;

                    let idx00 = ((y0 * TILE_SIZE + x0) * 4) as usize;
                    let idx10 = ((y0 * TILE_SIZE + x1) * 4) as usize;
                    let idx01 = ((y1 * TILE_SIZE + x0) * 4) as usize;
                    let idx11 = ((y1 * TILE_SIZE + x1) * 4) as usize;

                    let dst_x = tile_x + px;
                    let dst_y = tile_y + py;
                    let dst_pixel = (dst_y * width + dst_x) as usize;
                    let dst_idx = dst_pixel * 3;

                    let edge_x =
                        px.min(tile_w - 1 - px).min(TILE_OVERLAP) as f32 / TILE_OVERLAP as f32;
                    let edge_y =
                        py.min(tile_h - 1 - py).min(TILE_OVERLAP) as f32 / TILE_OVERLAP as f32;
                    let weight = edge_x.min(edge_y).max(0.01);

                    let max_idx = pixels.len();
                    if dst_idx + 2 < result_accum.len()
                        && idx00 + 2 < max_idx
                        && idx10 + 2 < max_idx
                        && idx01 + 2 < max_idx
                        && idx11 + 2 < max_idx
                    {
                        for c in 0..3 {
                            let v00 = pixels[idx00 + c] as f32;
                            let v10 = pixels[idx10 + c] as f32;
                            let v01 = pixels[idx01 + c] as f32;
                            let v11 = pixels[idx11 + c] as f32;
                            let v = v00 * (1.0 - fx) * (1.0 - fy)
                                + v10 * fx * (1.0 - fy)
                                + v01 * (1.0 - fx) * fy
                                + v11 * fx * fy;
                            result_accum[dst_idx + c] += v * weight;
                        }
                        weight_accum[dst_pixel] += weight;
                    }
                }
            }

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
