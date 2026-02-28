use image::{DynamicImage, Rgba};

fn seeded_rand(seed: u32) -> f32 {
    let mut state = seed;
    state = state.wrapping_mul(1103515245).wrapping_add(12345);
    ((state >> 16) & 0x7fff) as f32 / 32768.0
}

pub fn apply_fallback_noise(
    img: &DynamicImage,
    intensity: f32,
    seed: u32,
    iterations: u32,
) -> DynamicImage {
    let mut rgba = img.to_rgba8();
    let width = rgba.width();
    let height = rgba.height();
    let num_pixels = (width * height) as usize;
    let iterations = iterations.max(1);
    let epsilon = intensity * 255.0;

    let mut perturbation_r = vec![0.0f32; num_pixels];
    let mut perturbation_g = vec![0.0f32; num_pixels];
    let mut perturbation_b = vec![0.0f32; num_pixels];

    for k in 0..iterations {
        let step_size = epsilon / (1.0 + k as f32 * 0.5);
        let noise_scale = 4u32 + (k % 8) * 3;
        let iter_seed = seed.wrapping_add(k.wrapping_mul(7919));

        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;

                let bx = x / noise_scale;
                let by = y / noise_scale;
                let block_seed = iter_seed
                    .wrapping_add(bx.wrapping_mul(31))
                    .wrapping_add(by.wrapping_mul(17));

                let dr = (seeded_rand(block_seed.wrapping_add(1)) - 0.5) * step_size;
                let dg = (seeded_rand(block_seed.wrapping_add(2)) - 0.5) * step_size;
                let db = (seeded_rand(block_seed.wrapping_add(3)) - 0.5) * step_size;

                perturbation_r[idx] = (perturbation_r[idx] + dr).clamp(-epsilon, epsilon);
                perturbation_g[idx] = (perturbation_g[idx] + dg).clamp(-epsilon, epsilon);
                perturbation_b[idx] = (perturbation_b[idx] + db).clamp(-epsilon, epsilon);
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let pixel = rgba.get_pixel_mut(x, y);
            let Rgba([r, g, b, a]) = *pixel;

            *pixel = Rgba([
                (r as f32 + perturbation_r[idx]).clamp(0.0, 255.0) as u8,
                (g as f32 + perturbation_g[idx]).clamp(0.0, 255.0) as u8,
                (b as f32 + perturbation_b[idx]).clamp(0.0, 255.0) as u8,
                a,
            ]);
        }
    }

    DynamicImage::ImageRgba8(rgba)
}

pub fn encode_image_to_base64(img: &DynamicImage, quality: u8) -> Result<String, String> {
    let pixel_count = (img.width() * img.height()) as usize;
    let estimated_size = if quality < 100 {
        pixel_count / 4
    } else {
        pixel_count * 3
    };
    let mut output = std::io::Cursor::new(Vec::with_capacity(estimated_size));

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
