use image::{DynamicImage, Rgba};

use super::spsa::seeded_rand;

pub fn apply_fallback_noise(img: &DynamicImage, intensity: f32, seed: u32) -> DynamicImage {
    let mut rgba = img.to_rgba8();
    let width = rgba.width();
    let height = rgba.height();
    let noise_scale = 10u32;

    for y in 0..height {
        for x in 0..width {
            let pixel = rgba.get_pixel_mut(x, y);
            let Rgba([r, g, b, a]) = *pixel;

            let nx = x / noise_scale;
            let ny = y / noise_scale;
            let block_seed = seed.wrapping_add(nx.wrapping_mul(31).wrapping_add(ny));

            let noise = seeded_rand(block_seed);
            let noise_strength = intensity * (0.5 + noise * 0.5);

            let n = (
                (seeded_rand(block_seed.wrapping_add(1)) - 0.5) * noise_strength * 255.0,
                (seeded_rand(block_seed.wrapping_add(2)) - 0.5) * noise_strength * 255.0,
                (seeded_rand(block_seed.wrapping_add(3)) - 0.5) * noise_strength * 255.0,
            );

            *pixel = Rgba([
                (r as f32 + n.0).clamp(0.0, 255.0) as u8,
                (g as f32 + n.1).clamp(0.0, 255.0) as u8,
                (b as f32 + n.2).clamp(0.0, 255.0) as u8,
                a,
            ]);
        }
    }

    DynamicImage::ImageRgba8(rgba)
}

pub fn encode_image_to_base64(img: &DynamicImage, quality: u8) -> Result<String, String> {
    let mut output = std::io::Cursor::new(Vec::new());

    if quality < 100 {
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, quality);
        img.to_rgb8()
            .write_with_encoder(encoder)
            .map_err(|e| format!("Failed to encode image as JPEG: {}", e))?;
        let encoded = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            output.into_inner(),
        );
        Ok(format!("data:image/jpeg;base64,{}", encoded))
    } else {
        img.write_to(&mut output, image::ImageFormat::Png)
            .map_err(|e| format!("Failed to encode image as PNG: {}", e))?;
        let encoded = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            output.into_inner(),
        );
        Ok(format!("data:image/png;base64,{}", encoded))
    }
}
