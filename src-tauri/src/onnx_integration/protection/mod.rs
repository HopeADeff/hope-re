mod algorithms;
mod encoding;
mod model;
mod preprocessing;
mod spsa;
mod tiling;
mod types;

use ndarray::Array4;
use ort::session::Session;

use algorithms::{
    get_glaze_params, get_glaze_style_index, get_nightshade_params, get_nightshade_target_index,
    get_noise_params, run_glaze_model, run_nightshade_model, run_noise_model,
};
use encoding::{apply_fallback_noise, encode_image_to_base64};
use model::{load_model, resolve_model_path};
use tiling::apply_model_protection;
use types::ProtectionProgress;
pub use types::{ProtectionResult, ProtectionSettings};

use tauri::Emitter;

#[tauri::command]
pub async fn protect_image(
    app: tauri::AppHandle,
    image_base64: String,
    settings: ProtectionSettings,
) -> Result<ProtectionResult, String> {
    let image_data = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        image_base64
            .trim_start_matches("data:image/png;base64,")
            .trim_start_matches("data:image/jpeg;base64,")
            .trim_start_matches("data:image/jpg;base64,"),
    )
    .map_err(|e| format!("Failed to decode base64 image data: {}", e))?;

    let img = image::load_from_memory(&image_data)
        .map_err(|e| format!("Failed to load image from decoded data: {}", e))?;

    let intensity = settings.intensity;
    let quality = settings.output_quality.clamp(1, 100);
    let render_q = settings.render_quality.clamp(0, 100);
    let render_factor = render_q as f32 / 100.0;

    let _ = app.emit(
        "protection-progress",
        ProtectionProgress {
            stage: "loading".to_string(),
            tile_current: 0,
            tile_total: 0,
            iteration_current: 0,
            iteration_total: 0,
            percent: 2.0,
        },
    );

    let (protected, message, model_used) = match settings.algorithm.as_str() {
        "noise" => {
            let params = get_noise_params(intensity);
            let iterations = (params.max_iterations as f32 * render_factor).max(1.0) as u32;

            let model_result =
                resolve_model_path(&app, "noise_algorithm.onnx").and_then(|path| load_model(&path));

            match model_result {
                Ok(mut session) => {
                    log::info!("Noise ONNX model loaded successfully");
                    let mut run = |s: &mut Session, input: &Array4<f32>| -> Result<f32, String> {
                        run_noise_model(s, input)
                    };
                    let result = apply_model_protection(
                        &img,
                        &mut session,
                        &params,
                        iterations,
                        &mut run,
                        &app,
                    )?;
                    (
                        result,
                        "Noise protection applied with ONNX model".to_string(),
                        true,
                    )
                }
                Err(e) => {
                    log::error!("Noise ONNX model failed to load: {}", e);
                    let seed = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                        .as_secs() as u32;
                    let result = apply_fallback_noise(&img, intensity, seed, iterations);
                    (
                        result,
                        format!("Noise protection applied with fallback ({})", e),
                        false,
                    )
                }
            }
        }
        "glaze" => {
            let style = settings.glaze_style.as_deref().unwrap_or("abstract");
            let style_index = get_glaze_style_index(style);
            let params = get_glaze_params(intensity);
            let iterations = (params.max_iterations as f32 * render_factor).max(1.0) as u32;

            let model_result =
                resolve_model_path(&app, "glaze_algorithm.onnx").and_then(|path| load_model(&path));

            match model_result {
                Ok(mut session) => {
                    log::info!("Glaze ONNX model loaded successfully");
                    let mut run =
                        move |s: &mut Session, input: &Array4<f32>| -> Result<f32, String> {
                            run_glaze_model(s, input, style_index)
                        };
                    let result = apply_model_protection(
                        &img,
                        &mut session,
                        &params,
                        iterations,
                        &mut run,
                        &app,
                    )?;
                    (
                        result,
                        format!("Glaze ({}) protection applied with ONNX model", style),
                        true,
                    )
                }
                Err(e) => {
                    log::error!("Glaze ONNX model failed to load: {}", e);
                    let seed = get_glaze_style_index(style) as u32;
                    let effective_intensity = intensity * 0.8;
                    let result = apply_fallback_noise(&img, effective_intensity, seed, iterations);
                    (
                        result,
                        format!("Glaze ({}) protection applied with fallback ({})", style, e),
                        false,
                    )
                }
            }
        }
        "nightshade" => {
            let target = settings.nightshade_target.as_deref().unwrap_or("dog");
            let target_index = get_nightshade_target_index(target);
            let params = get_nightshade_params(intensity);
            let iterations = (params.max_iterations as f32 * render_factor).max(1.0) as u32;

            let model_result = resolve_model_path(&app, "nightshade_algorithm.onnx")
                .and_then(|path| load_model(&path));

            match model_result {
                Ok(mut session) => {
                    log::info!("Nightshade ONNX model loaded successfully");
                    let mut run =
                        move |s: &mut Session, input: &Array4<f32>| -> Result<f32, String> {
                            run_nightshade_model(s, input, target_index)
                        };
                    let result = apply_model_protection(
                        &img,
                        &mut session,
                        &params,
                        iterations,
                        &mut run,
                        &app,
                    )?;
                    (
                        result,
                        format!("Nightshade ({}) protection applied with ONNX model", target),
                        true,
                    )
                }
                Err(e) => {
                    log::error!("Nightshade ONNX model failed to load: {}", e);
                    let seed = get_nightshade_target_index(target) as u32 + 100;
                    let effective_intensity = intensity * 1.2;
                    let result = apply_fallback_noise(&img, effective_intensity, seed, iterations);
                    (
                        result,
                        format!(
                            "Nightshade ({}) protection applied with fallback ({})",
                            target, e
                        ),
                        false,
                    )
                }
            }
        }
        _ => return Err(format!("Unknown algorithm: {}", settings.algorithm)),
    };

    let _ = app.emit(
        "protection-progress",
        ProtectionProgress {
            stage: "encoding".to_string(),
            tile_current: 0,
            tile_total: 0,
            iteration_current: 0,
            iteration_total: 0,
            percent: 96.0,
        },
    );

    let image_base64 = encode_image_to_base64(&protected, quality)?;

    let _ = app.emit(
        "protection-progress",
        ProtectionProgress {
            stage: "complete".to_string(),
            tile_current: 0,
            tile_total: 0,
            iteration_current: 0,
            iteration_total: 0,
            percent: 100.0,
        },
    );

    Ok(ProtectionResult {
        image_base64,
        success: true,
        message,
        model_used,
    })
}
