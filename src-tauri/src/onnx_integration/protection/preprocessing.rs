use image::{DynamicImage, Rgba};
use ndarray::{Array, Array4};
use ort::value::TensorRef;

use super::types::TILE_SIZE;

pub fn preprocess_tile(img: &DynamicImage, x: u32, y: u32, w: u32, h: u32) -> Array4<f32> {
    let cropped = img.crop_imm(x, y, w, h);
    let resized = cropped.resize_exact(TILE_SIZE, TILE_SIZE, image::imageops::FilterType::Triangle);
    let rgba = resized.to_rgba8();

    let data: Vec<f32> = rgba
        .pixels()
        .flat_map(|p| {
            let Rgba([r, g, b, _]) = *p;
            [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
        })
        .collect();

    Array::from_shape_vec((1, TILE_SIZE as usize, TILE_SIZE as usize, 3), data)
        .unwrap_or_else(|_| Array4::zeros((1, TILE_SIZE as usize, TILE_SIZE as usize, 3)))
}

pub fn compute_edge_weight_map(tile: &Array4<f32>) -> Vec<f32> {
    let h = TILE_SIZE as usize;
    let w = TILE_SIZE as usize;
    let num_pixels = h * w;

    let mut gray = vec![0.0f32; num_pixels];
    for y in 0..h {
        for x in 0..w {
            gray[y * w + x] = (tile[[0, y, x, 0]] + tile[[0, y, x, 1]] + tile[[0, y, x, 2]]) / 3.0;
        }
    }

    let mut edges = vec![0.0f32; num_pixels];
    for y in 0..h {
        for x in 0..w {
            let gx = if y + 1 < h {
                (gray[(y + 1) * w + x] - gray[y * w + x]).abs()
            } else {
                (gray[y * w + x] - gray[(y - 1) * w + x]).abs()
            };

            let gy = if x + 1 < w {
                (gray[y * w + x + 1] - gray[y * w + x]).abs()
            } else {
                (gray[y * w + x] - gray[y * w + x - 1]).abs()
            };

            edges[y * w + x] = (gx * gx + gy * gy).sqrt();
        }
    }

    let mut min_e = f32::MAX;
    let mut max_e = f32::MIN;
    for &e in &edges {
        if e < min_e {
            min_e = e;
        }
        if e > max_e {
            max_e = e;
        }
    }
    let range = max_e - min_e + 1e-8;

    for e in &mut edges {
        let normalized = (*e - min_e) / range;
        *e = 0.3 + 0.7 * normalized;
    }

    edges
}

pub fn array4_to_tensor_ref(input: &Array4<f32>) -> Result<TensorRef<'_, f32>, String> {
    let shape = input.shape();
    let data = input
        .as_slice()
        .ok_or_else(|| "Array is not contiguous in memory".to_string())?;
    TensorRef::from_array_view(([shape[0], shape[1], shape[2], shape[3]], data))
        .map_err(|e| format!("Failed to create tensor ref: {}", e))
}

pub fn tile_to_pixels(tile: &Array4<f32>, width: u32, height: u32) -> Vec<u8> {
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height as usize {
        for x in 0..width as usize {
            let r = (tile[[0, y, x, 0]] * 255.0).clamp(0.0, 255.0) as u8;
            let g = (tile[[0, y, x, 1]] * 255.0).clamp(0.0, 255.0) as u8;
            let b = (tile[[0, y, x, 2]] * 255.0).clamp(0.0, 255.0) as u8;
            pixels.extend_from_slice(&[r, g, b, 255]);
        }
    }
    pixels
}
