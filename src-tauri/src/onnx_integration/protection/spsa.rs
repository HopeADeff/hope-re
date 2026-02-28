use ndarray::{Array, Array4};
use ort::session::Session;
use tauri::Emitter;

use super::types::{AlgorithmParams, ModelRunFn, ProtectionProgress, SPSA_DIRECTIONS_PER_ITER};

pub fn seeded_rand(seed: u32) -> f32 {
    let mut state = seed;
    state = state.wrapping_mul(1103515245).wrapping_add(12345);
    ((state >> 16) & 0x7fff) as f32 / 32768.0
}

fn generate_spsa_direction(num_elements: usize, seed: u32) -> Vec<f32> {
    (0..num_elements)
        .map(|i| {
            let r = seeded_rand(seed.wrapping_add(i as u32));
            if r > 0.5 {
                1.0
            } else {
                -1.0
            }
        })
        .collect()
}

pub fn spsa_pgd_on_tile(
    session: &mut Session,
    base_tile: &Array4<f32>,
    params: &AlgorithmParams,
    iterations: u32,
    run_model: &mut ModelRunFn,
    app: &tauri::AppHandle,
    tile_current: u32,
    tile_total: u32,
) -> Result<Array4<f32>, String> {
    let shape = base_tile.shape();
    let num_elements = shape.iter().product::<usize>();
    let mut perturbation = vec![0.0f32; num_elements];
    let epsilon = params.epsilon;
    let alpha = epsilon * params.alpha_multiplier / iterations.max(1) as f32;
    let ck_initial = epsilon * 0.1;

    let base_flat: Vec<f32> = base_tile.iter().copied().collect();

    for k in 0..iterations {
        let ck = ck_initial / ((k + 1) as f32).powf(0.101);
        let ak = alpha / ((k + 1) as f32).powf(0.602);

        let mut grad_estimate = vec![0.0f32; num_elements];

        let iter_seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0))
            .as_nanos() as u32;

        for d in 0..SPSA_DIRECTIONS_PER_ITER {
            let direction = generate_spsa_direction(
                num_elements,
                iter_seed.wrapping_add(k.wrapping_mul(100).wrapping_add(d as u32)),
            );

            let mut plus_data = Vec::with_capacity(num_elements);
            let mut minus_data = Vec::with_capacity(num_elements);

            for i in 0..num_elements {
                let base_val = base_flat[i] + perturbation[i];
                let delta = ck * direction[i];
                plus_data.push((base_val + delta).clamp(0.0, 1.0));
                minus_data.push((base_val - delta).clamp(0.0, 1.0));
            }

            let plus_tile =
                Array::from_shape_vec((shape[0], shape[1], shape[2], shape[3]), plus_data)
                    .unwrap_or_else(|_| base_tile.clone());

            let minus_tile =
                Array::from_shape_vec((shape[0], shape[1], shape[2], shape[3]), minus_data)
                    .unwrap_or_else(|_| base_tile.clone());

            let loss_plus = run_model(session, &plus_tile).unwrap_or(0.0);
            let loss_minus = run_model(session, &minus_tile).unwrap_or(0.0);

            let diff = loss_plus - loss_minus;
            for i in 0..num_elements {
                grad_estimate[i] +=
                    (diff / (2.0 * ck * direction[i])) / SPSA_DIRECTIONS_PER_ITER as f32;
            }
        }

        for i in 0..num_elements {
            let sign = if grad_estimate[i] > 0.0 {
                1.0
            } else if grad_estimate[i] < 0.0 {
                -1.0
            } else {
                0.0
            };
            perturbation[i] -= ak * sign;
            perturbation[i] = perturbation[i].clamp(-epsilon, epsilon);
        }

        if cfg!(debug_assertions) && k % 50 == 0 {
            log::info!("PGD iteration {}/{}", k, iterations);
        }

        if iterations > 0 && k % 10 == 0 {
            let tile_frac = if tile_total > 0 {
                (tile_current - 1) as f64 / tile_total as f64
            } else {
                0.0
            };
            let iter_frac = (k + 1) as f64 / iterations as f64;
            let per_tile = 1.0 / tile_total.max(1) as f64;
            let percent = ((tile_frac + iter_frac * per_tile) * 95.0).min(95.0);
            let _ = app.emit(
                "protection-progress",
                ProtectionProgress {
                    stage: "processing".to_string(),
                    tile_current,
                    tile_total,
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
