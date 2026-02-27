use std::path::PathBuf;

use futures_util::StreamExt;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};

const GITHUB_RELEASE_URL: &str = "https://github.com/HopeArtOrg/hope-re/releases/download";

const MODELS: [&str; 3] = [
    "noise_algorithm.onnx",
    "glaze_algorithm.onnx",
    "nightshade_algorithm.onnx",
];

#[derive(Debug, Clone, Serialize)]
pub struct ModelStatus {
    pub name: String,
    pub exists: bool,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelsCheckResult {
    pub models: Vec<ModelStatus>,
    pub all_ready: bool,
    pub models_dir: String,
}

#[derive(Debug, Clone, Serialize)]
struct DownloadProgress {
    model_name: String,
    downloaded_bytes: u64,
    total_bytes: u64,
    progress_percent: f64,
}

fn get_models_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to resolve app data directory: {}", e))?;

    let models_dir = app_data.join("models");

    if !models_dir.exists() {
        std::fs::create_dir_all(&models_dir)
            .map_err(|e| format!("Failed to create models directory: {}", e))?;
    }

    Ok(models_dir)
}

pub fn resolve_downloaded_model(app: &AppHandle, model_filename: &str) -> Option<PathBuf> {
    let models_dir = get_models_dir(app).ok()?;
    let path = models_dir.join(model_filename);

    if path.exists() && std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0) > 1024 {
        Some(path)
    } else {
        None
    }
}

fn get_app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[tauri::command]
pub async fn check_models_status(app: AppHandle) -> Result<ModelsCheckResult, String> {
    let models_dir = get_models_dir(&app)?;
    let mut models = Vec::new();
    let mut all_ready = true;

    for model_name in &MODELS {
        let path = models_dir.join(model_name);
        let (exists, size_bytes) = if path.exists() {
            let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            (size > 1024, size)
        } else {
            (false, 0)
        };

        if !exists {
            all_ready = false;
        }

        models.push(ModelStatus {
            name: model_name.to_string(),
            exists,
            size_bytes,
        });
    }

    Ok(ModelsCheckResult {
        models,
        all_ready,
        models_dir: models_dir.to_string_lossy().to_string(),
    })
}

#[tauri::command]
pub async fn download_model(app: AppHandle, model_name: String) -> Result<String, String> {
    if !MODELS.contains(&model_name.as_str()) {
        return Err(format!("Unknown model: {}", model_name));
    }

    let models_dir = get_models_dir(&app)?;
    let dest_path = models_dir.join(&model_name);

    if dest_path.exists() {
        let size = std::fs::metadata(&dest_path).map(|m| m.len()).unwrap_or(0);
        if size > 1024 {
            return Ok(format!("{} already downloaded", model_name));
        }
    }

    let version = get_app_version();
    let url = format!("{}/{}/{}", GITHUB_RELEASE_URL, version, model_name);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client
        .get(&url)
        .header("Accept", "application/octet-stream")
        .send()
        .await
        .map_err(|e| format!("Failed to download {}: {}", model_name, e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Download failed for {} (HTTP {}). The model may not be available for version {}",
            model_name,
            response.status(),
            version
        ));
    }

    let total_bytes = response.content_length().unwrap_or(0);
    let temp_path = dest_path.with_extension("onnx.tmp");

    let mut file = tokio::fs::File::create(&temp_path)
        .await
        .map_err(|e| format!("Failed to create temp file: {}", e))?;

    let mut stream = response.bytes_stream();
    let mut downloaded_bytes: u64 = 0;
    let mut last_emitted_percent: f64 = -1.0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Download interrupted: {}", e))?;

        tokio::io::AsyncWriteExt::write_all(&mut file, &chunk)
            .await
            .map_err(|e| format!("Failed to write chunk: {}", e))?;

        downloaded_bytes += chunk.len() as u64;

        let progress_percent = if total_bytes > 0 {
            (downloaded_bytes as f64 / total_bytes as f64 * 100.0).min(100.0)
        } else {
            0.0
        };

        if (progress_percent - last_emitted_percent).abs() >= 0.5 {
            last_emitted_percent = progress_percent;
            let _ = app.emit(
                "model-download-progress",
                DownloadProgress {
                    model_name: model_name.clone(),
                    downloaded_bytes,
                    total_bytes,
                    progress_percent,
                },
            );
        }
    }

    tokio::io::AsyncWriteExt::flush(&mut file)
        .await
        .map_err(|e| format!("Failed to flush file: {}", e))?;

    drop(file);

    tokio::fs::rename(&temp_path, &dest_path)
        .await
        .map_err(|e| format!("Failed to finalize download: {}", e))?;

    let _ = app.emit(
        "model-download-progress",
        DownloadProgress {
            model_name: model_name.clone(),
            downloaded_bytes,
            total_bytes,
            progress_percent: 100.0,
        },
    );

    Ok(format!("{} downloaded successfully", model_name))
}
