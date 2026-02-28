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

fn fill_spsa_direction(buf: &mut [f32], rng: &mut Xoshiro128) {
    for v in buf.iter_mut() {
        *v = if rng.next_bool() { 1.0 } else { -1.0 };
    }
}

fn expand_edge_weights(edge_weights: &[f32], num_elements: usize) -> Vec<f32> {
    let tile_pixels = (TILE_SIZE * TILE_SIZE) as usize;
    let mut edge_flat = vec![1.0f32; num_elements];
    for i in 0..tile_pixels {
        let ew = if i < edge_weights.len() {
            edge_weights[i]
        } else {
            1.0
        };
        let base = i * 3;
        if base + 2 < num_elements {
            edge_flat[base] = ew;
            edge_flat[base + 1] = ew;
            edge_flat[base + 2] = ew;
        }
    }
    edge_flat
}

const MOMENTUM_BETA: f32 = 0.8;

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
    let ck_initial = epsilon * 0.15;
    let perceptual_weight = params.perceptual_weight;

    let edge_flat = expand_edge_weights(edge_weights, num_elements);
    let base_flat: Vec<f32> = base_tile.iter().copied().collect();

    let global_seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
        .as_nanos() as u64;
    let mut rng = Xoshiro128::new(global_seed);

    let mut direction = vec![0.0f32; num_elements];
    let mut plus_data = vec![0.0f32; num_elements];
    let mut minus_data = vec![0.0f32; num_elements];
    let mut grad_estimate = vec![0.0f32; num_elements];
    let mut momentum = vec![0.0f32; num_elements];

    let mut consecutive_failures = 0u32;
    let max_consecutive_failures = 5u32;

    for k in 0..iterations {
        let ck = ck_initial / ((k + 1) as f32).powf(0.05);
        let ak = alpha / ((k + 1) as f32).powf(0.3);

        for v in grad_estimate.iter_mut() {
            *v = 0.0;
        }
        let mut valid_directions = 0u32;

        for _d in 0..SPSA_DIRECTIONS_PER_ITER {
            fill_spsa_direction(&mut direction, &mut rng);

            for i in 0..num_elements {
                let base_val = base_flat[i] + perturbation[i];
                let delta = ck * direction[i];
                plus_data[i] = (base_val + delta).clamp(0.0, 1.0);
                minus_data[i] = (base_val - delta).clamp(0.0, 1.0);
            }

            let p_loss_diff = if perceptual_weight > 0.0 {
                let inv_n = 1.0 / num_elements as f32;
                let mut p_diff_accum = 0.0f32;
                for i in 0..num_elements {
                    let diff_plus = plus_data[i] - base_flat[i];
                    let diff_minus = minus_data[i] - base_flat[i];
                    let inv_edge = 1.5 - edge_flat[i];
                    p_diff_accum += (diff_plus * diff_plus - diff_minus * diff_minus) * inv_edge;
                }
                perceptual_weight * p_diff_accum * inv_n * 100.0
            } else {
                0.0
            };

            let plus_tile =
                Array::from_shape_vec((shape[0], shape[1], shape[2], shape[3]), plus_data.to_vec())
                    .unwrap_or_else(|_| base_tile.clone());

            let minus_tile = Array::from_shape_vec(
                (shape[0], shape[1], shape[2], shape[3]),
                minus_data.to_vec(),
            )
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

            let diff_over_2ck = (loss_plus - loss_minus + p_loss_diff) / (2.0 * ck);
            for i in 0..num_elements {
                grad_estimate[i] += diff_over_2ck * direction[i];
            }
        }

        if valid_directions > 0 {
            let scale = 1.0 / valid_directions as f32;
            for i in 0..num_elements {
                let weighted_grad = grad_estimate[i] * scale * edge_flat[i];
                momentum[i] = MOMENTUM_BETA * momentum[i] + (1.0 - MOMENTUM_BETA) * weighted_grad;
                let sign = if momentum[i] > 0.0 {
                    1.0
                } else if momentum[i] < 0.0 {
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

        if iterations > 0 && k % 20 == 0 {
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
