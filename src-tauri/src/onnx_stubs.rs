use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ExecutionProviderInfo {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct InferenceCapabilities {
    pub providers: Vec<ExecutionProviderInfo>,
    pub platform: String,
}

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

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct ProtectionSettings {
    pub algorithm: String,
    pub intensity: f32,
    pub output_quality: u8,
    pub render_quality: u8,
    pub glaze_style: Option<String>,
    pub nightshade_target: Option<String>,
}

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct ProtectionResult {
    pub image_base64: String,
    pub success: bool,
    pub message: String,
    pub model_used: bool,
}

#[tauri::command]
pub fn get_inference_capabilities() -> Result<InferenceCapabilities, String> {
    Ok(InferenceCapabilities {
        providers: vec![ExecutionProviderInfo {
            name: "CPU".to_string(),
        }],
        platform: "Android (unsupported architecture)".to_string(),
    })
}

#[tauri::command]
pub fn create_ort_session(_model_path: String) -> Result<String, String> {
    Err("ONNX Runtime is not available on this Android architecture".to_string())
}

#[tauri::command]
pub async fn check_models_status(_app: tauri::AppHandle) -> Result<ModelsCheckResult, String> {
    Ok(ModelsCheckResult {
        models: Vec::new(),
        all_ready: false,
        models_dir: String::new(),
    })
}

#[tauri::command]
pub async fn download_model(_app: tauri::AppHandle, _model_name: String) -> Result<String, String> {
    Err("ONNX Runtime is not available on this Android architecture".to_string())
}

#[tauri::command]
pub async fn protect_image(
    _app: tauri::AppHandle,
    _image_base64: String,
    _settings: ProtectionSettings,
) -> Result<ProtectionResult, String> {
    Err("ONNX Runtime is not available on this Android architecture".to_string())
}
