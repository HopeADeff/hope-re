use ndarray::{Array, Array4};
use ort::session::Session;
use tauri::Emitter;

use super::types::{
    AlgorithmParams, ModelRunFn, ProtectionProgress, TileProgress, SPSA_DIRECTIONS_PER_ITER,
    TILE_SIZE,
};

struct Xoshiro128 {
    s: [u32; 4],
}

impl Xoshiro128 {
    fn new(seed: u64) -> Self {
        let mut s = [0u32; 4];
        let mut z = seed;
        for slot in &mut s {
            z = z.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(1);
            *slot = (z >> 32) as u32 ^ z as u32;
            if *slot == 0 {
                *slot = 1;
            }
        }
        Self { s }
    }

    fn next_u32(&mut self) -> u32 {
        let result = self.s[0]
            .wrapping_add(self.s[3])
            .rotate_left(7)
            .wrapping_add(self.s[0]);
        let t = self.s[1] << 9;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(11);
        result
    }

    fn next_bool(&mut self) -> bool {
        self.next_u32() & 1 == 0
    }
}

fn generate_spsa_direction(num_elements: usize, rng: &mut Xoshiro128) -> Vec<f32> {
    (0..num_elements)
        .map(|_| if rng.next_bool() { 1.0 } else { -1.0 })
        .collect()
}

pub fn seeded_rand(seed: u32) -> f32 {
    let mut state = seed;
    state = state.wrapping_mul(1103515245).wrapping_add(12345);
    ((state >> 16) & 0x7fff) as f32 / 32768.0
}

pub fn spsa_pgd_on_tile(
    session: &mut Session,
    base_tile: &Array4<f32>,
    params: &AlgorithmParams,
    iterations: u32,
    run_model: &mut ModelRunFn,
    edge_weights: &[f32],
    progress: &TileProgress,
) -> Result<Array4<f32>, String> {
    let shape = base_tile.shape();
    let num_elements = shape.iter().product::<usize>();
    let mut perturbation = vec![0.0f32; num_elements];
    let epsilon = params.epsilon;
    let alpha = epsilon * params.alpha_multiplier / iterations.max(1) as f32;
    let ck_initial = epsilon * 0.1;
    let perceptual_weight = params.perceptual_weight;

    let tile_pixels = (TILE_SIZE * TILE_SIZE) as usize;
    let per_pixel_edge: Vec<[f32; 3]> = (0..tile_pixels)
        .map(|i| {
            let ew = if i < edge_weights.len() {
                edge_weights[i]
            } else {
                1.0
            };
            [ew, ew, ew]
        })
        .collect();
    let edge_flat: Vec<f32> = per_pixel_edge
        .iter()
        .flat_map(|e| e.iter().copied())
        .collect();

    let base_flat: Vec<f32> = base_tile.iter().copied().collect();

    let global_seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
        .as_nanos() as u64;
    let mut rng = Xoshiro128::new(global_seed);

    let mut consecutive_failures = 0u32;
    let max_consecutive_failures = 5u32;

    for k in 0..iterations {
        let ck = ck_initial / ((k + 1) as f32).powf(0.101);
        let ak = alpha / ((k + 1) as f32).powf(0.602);

        let mut grad_estimate = vec![0.0f32; num_elements];
        let mut valid_directions = 0u32;

        for _d in 0..SPSA_DIRECTIONS_PER_ITER {
            let direction = generate_spsa_direction(num_elements, &mut rng);

            let plus_data: Vec<f32> = (0..num_elements)
                .map(|i| {
                    let base_val = base_flat[i] + perturbation[i];
                    let delta = ck * direction[i];
                    (base_val + delta).clamp(0.0, 1.0)
                })
                .collect();

            let minus_data: Vec<f32> = (0..num_elements)
                .map(|i| {
                    let base_val = base_flat[i] + perturbation[i];
                    let delta = ck * direction[i];
                    (base_val - delta).clamp(0.0, 1.0)
                })
                .collect();

            let p_loss_diff = if perceptual_weight > 0.0 {
                let mut p_loss_plus = 0.0f32;
                let mut p_loss_minus = 0.0f32;
                for i in 0..num_elements {
                    let diff_plus = plus_data[i] - base_flat[i];
                    let diff_minus = minus_data[i] - base_flat[i];
                    let inv_edge = 1.5 - edge_flat[i];
                    p_loss_plus += diff_plus * diff_plus * inv_edge;
                    p_loss_minus += diff_minus * diff_minus * inv_edge;
                }
                p_loss_plus /= num_elements as f32;
                p_loss_minus /= num_elements as f32;
                perceptual_weight * (p_loss_plus - p_loss_minus) * 100.0
            } else {
                0.0
            };

            let plus_tile =
                Array::from_shape_vec((shape[0], shape[1], shape[2], shape[3]), plus_data)
                    .unwrap_or_else(|_| base_tile.clone());

            let minus_tile =
                Array::from_shape_vec((shape[0], shape[1], shape[2], shape[3]), minus_data)
                    .unwrap_or_else(|_| base_tile.clone());

            let loss_plus = match run_model(session, &plus_tile) {
                Ok(v) => v,
                Err(e) => {
                    consecutive_failures += 1;
                    if consecutive_failures >= max_consecutive_failures {
                        return Err(format!(
                            "ONNX model inference failed {} times consecutively at iteration {}: {}",
                            max_consecutive_failures, k, e
                        ));
                    }
                    log::warn!("Model inference failed (plus) at iter {}: {}", k, e);
                    continue;
                }
            };
            let loss_minus = match run_model(session, &minus_tile) {
                Ok(v) => v,
                Err(e) => {
                    consecutive_failures += 1;
                    if consecutive_failures >= max_consecutive_failures {
                        return Err(format!(
                            "ONNX model inference failed {} times consecutively at iteration {}: {}",
                            max_consecutive_failures, k, e
                        ));
                    }
                    log::warn!("Model inference failed (minus) at iter {}: {}", k, e);
                    continue;
                }
            };

            consecutive_failures = 0;
            valid_directions += 1;

            let diff = (loss_plus - loss_minus) + p_loss_diff;
            for i in 0..num_elements {
                grad_estimate[i] += diff / (2.0 * ck * direction[i]);
            }
        }

        if valid_directions > 0 {
            let scale = 1.0 / valid_directions as f32;
            for i in 0..num_elements {
                let weighted_grad = grad_estimate[i] * scale * edge_flat[i];
                let sign = if weighted_grad > 0.0 {
                    1.0
                } else if weighted_grad < 0.0 {
                    -1.0
                } else {
                    0.0
                };
                perturbation[i] -= ak * sign;
                perturbation[i] = perturbation[i].clamp(-epsilon, epsilon);
            }
        }

        if cfg!(debug_assertions) && k % 50 == 0 {
            log::info!("PGD iteration {}/{}", k, iterations);
        }

        if iterations > 0 && k % 10 == 0 {
            let tile_frac = if progress.tile_total > 0 {
                (progress.tile_current - 1) as f64 / progress.tile_total as f64
            } else {
                0.0
            };
            let iter_frac = (k + 1) as f64 / iterations as f64;
            let per_tile = 1.0 / progress.tile_total.max(1) as f64;
            let percent = ((tile_frac + iter_frac * per_tile) * 95.0).min(95.0);
            let _ = progress.app.emit(
                "protection-progress",
                ProtectionProgress {
                    stage: "processing".to_string(),
                    tile_current: progress.tile_current,
                    tile_total: progress.tile_total,
                    iteration_current: k + 1,
                    iteration_total: iterations,
                    percent,
                },
            );
        }
    }

    let result_data: Vec<f32> = base_flat
        .iter()
        .zip(perturbation.iter())
        .map(|(b, p)| (*b + *p).clamp(0.0, 1.0))
        .collect();

    Array::from_shape_vec((shape[0], shape[1], shape[2], shape[3]), result_data)
        .map_err(|e| format!("Failed to reshape result tile: {}", e))
}
