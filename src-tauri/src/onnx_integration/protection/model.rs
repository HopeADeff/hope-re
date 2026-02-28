use std::path::PathBuf;

use ort::session::Session;

use crate::onnx_integration::model_downloader::resolve_downloaded_model;

fn is_git_lfs_pointer(bytes: &[u8]) -> bool {
    bytes.starts_with(b"version https://git-lfs.github.com/spec/")
}

pub fn load_model(model_path: &std::path::Path) -> Result<Session, String> {
    let model_bytes = std::fs::read(model_path)
        .map_err(|e| format!("Failed to read model at {:?}: {}", model_path, e))?;

    if is_git_lfs_pointer(&model_bytes) {
        return Err(format!(
            "Model at {:?} is a Git LFS pointer ({}B), not an actual ONNX file. Run `git lfs pull` to download the real model",
            model_path,
            model_bytes.len()
        ));
    }

    Session::builder()
        .map_err(|e| format!("Failed to create session builder: {}", e))?
        .commit_from_memory(&model_bytes)
        .map_err(|e| format!("Failed to load model from {:?}: {}", model_path, e))
}

pub fn resolve_model_path(app: &tauri::AppHandle, model_filename: &str) -> Result<PathBuf, String> {
    use tauri::Manager;

    if let Some(downloaded) = resolve_downloaded_model(app, model_filename) {
        return Ok(downloaded);
    }

    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to resolve resource directory: {}", e))?;

    let candidates = [
        resource_dir.join(model_filename),
        resource_dir.join("models").join(model_filename),
        resource_dir
            .join("src-models")
            .join("models")
            .join(model_filename),
    ];

    for path in &candidates {
        if path.exists() {
            return Ok(path.clone());
        }
    }

    if cfg!(debug_assertions) {
        let dev_candidates = [
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join("src-models")
                .join("models")
                .join(model_filename),
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("src-models")
                .join("models")
                .join(model_filename),
        ];

        for path in &dev_candidates {
            if let Ok(canonical) = path.canonicalize() {
                if canonical.exists() {
                    return Ok(canonical);
                }
            }
        }
    }

    Err(format!(
        "Model file {} not found. Please download models from the app settings or ensure they are available in {:?}",
        model_filename, resource_dir
    ))
}
