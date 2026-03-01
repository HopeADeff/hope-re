use image::{DynamicImage, Rgba};

fn seeded_rand(seed: u32) -> f32 {
    let mut state = seed;
    state = state.wrapping_mul(1103515245).wrapping_add(12345);
    ((state >> 16) & 0x7fff) as f32 / 32768.0
}

fn seeded_rand_pair(seed: u32) -> (f32, f32) {
    let a = seeded_rand(seed);
    let b = seeded_rand(seed.wrapping_mul(2654435761));
    (a, b)
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
        let step_size = epsilon / (1.0 + k as f32 * 0.3);
        let iter_seed = seed.wrapping_add(k.wrapping_mul(7919));

        let noise_scales: [u32; 3] = [4 + (k % 6) * 2, 8 + (k % 5) * 4, 16 + (k % 4) * 8];

        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize;
                let mut dr = 0.0f32;
                let mut dg = 0.0f32;
                let mut db = 0.0f32;

                for (si, &scale) in noise_scales.iter().enumerate() {
                    let bx = x / scale;
                    let by = y / scale;
                    let block_seed = iter_seed
                        .wrapping_add(bx.wrapping_mul(31))
                        .wrapping_add(by.wrapping_mul(17))
                        .wrapping_add(si as u32 * 9973);

                    let weight = 1.0 / (si as f32 + 1.0);
                    dr += (seeded_rand(block_seed.wrapping_add(1)) - 0.5) * weight;
                    dg += (seeded_rand(block_seed.wrapping_add(2)) - 0.5) * weight;
                    db += (seeded_rand(block_seed.wrapping_add(3)) - 0.5) * weight;
                }

                let (phase_x, phase_y) = seeded_rand_pair(iter_seed.wrapping_add(k));
                let freq = (2.0 + (k % 7) as f32) * std::f32::consts::PI * 2.0;
                let wave = ((x as f32 * freq / width as f32 + phase_x * std::f32::consts::TAU)
                    .sin()
                    * (y as f32 * freq / height as f32 + phase_y * std::f32::consts::TAU).cos())
                    * 0.15;

                dr = (dr + wave) * step_size;
                dg = (dg + wave * 0.8) * step_size;
                db = (db + wave * 1.2) * step_size;

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
