use image::DynamicImage;
use ort::session::Session;

use super::preprocessing::{preprocess_tile, tile_to_pixels};
use super::spsa::spsa_pgd_on_tile;
use super::types::{AlgorithmParams, ModelRunFn, TILE_OVERLAP, TILE_SIZE};

pub fn apply_model_protection(
    img: &DynamicImage,
    session: &mut Session,
    params: &AlgorithmParams,
    iterations: u32,
    run_model: &mut ModelRunFn,
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

            let protected_tile =
                spsa_pgd_on_tile(session, &base_tile, params, iterations, run_model)?;

            let pixels = tile_to_pixels(&protected_tile, TILE_SIZE, TILE_SIZE);

            let scale_x = tile_w as f32 / TILE_SIZE as f32;
            let scale_y = tile_h as f32 / TILE_SIZE as f32;

            for py in 0..tile_h {
                for px in 0..tile_w {
                    let src_x = ((px as f32 / scale_x) as u32).min(TILE_SIZE - 1);
                    let src_y = ((py as f32 / scale_y) as u32).min(TILE_SIZE - 1);
                    let src_idx = ((src_y * TILE_SIZE + src_x) * 4) as usize;

                    let dst_x = tile_x + px;
                    let dst_y = tile_y + py;
                    let dst_pixel = (dst_y * width + dst_x) as usize;
                    let dst_idx = dst_pixel * 3;

                    let edge_x =
                        px.min(tile_w - 1 - px).min(TILE_OVERLAP) as f32 / TILE_OVERLAP as f32;
                    let edge_y =
                        py.min(tile_h - 1 - py).min(TILE_OVERLAP) as f32 / TILE_OVERLAP as f32;
                    let weight = edge_x.min(edge_y).max(0.01);

                    if dst_idx + 2 < result_accum.len() && src_idx + 2 < pixels.len() {
                        result_accum[dst_idx] += pixels[src_idx] as f32 * weight;
                        result_accum[dst_idx + 1] += pixels[src_idx + 1] as f32 * weight;
                        result_accum[dst_idx + 2] += pixels[src_idx + 2] as f32 * weight;
                        weight_accum[dst_pixel] += weight;
                    }
                }
            }

            tile_count += 1;
            if cfg!(debug_assertions) {
                log::info!("Processed tile {}/{}", tile_count, total_tiles);
            }
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
