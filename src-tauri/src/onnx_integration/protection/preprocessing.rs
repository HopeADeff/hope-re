use image::{DynamicImage, Rgba};
use ndarray::{Array, Array4};

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
    for (idx, g) in gray.iter_mut().enumerate() {
        let y = idx / w;
        let x = idx % w;
        *g = (tile[[0, y, x, 0]] + tile[[0, y, x, 1]] + tile[[0, y, x, 2]]) * (1.0 / 3.0);
    }

    let mut edges = vec![0.0f32; num_pixels];
    for y in 0..h {
        let row = y * w;
        let next_row = if y + 1 < h { (y + 1) * w } else { (y - 1) * w };
        for x in 0..w {
            let gx = (gray[next_row + x] - gray[row + x]).abs();
            let next_x = if x + 1 < w { x + 1 } else { x - 1 };
            let gy = (gray[row + next_x] - gray[row + x]).abs();
            edges[row + x] = (gx * gx + gy * gy).sqrt();
        }
    }

    let (min_e, max_e) = edges
        .iter()
        .fold((f32::MAX, f32::MIN), |(mn, mx), &e| (mn.min(e), mx.max(e)));
    let inv_range = 1.0 / (max_e - min_e + 1e-8);

    for e in &mut edges {
        *e = 0.3 + 0.7 * (*e - min_e) * inv_range;
    }

    edges
}
