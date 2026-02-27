use crate::onnx_integration::capabilities::build_execution_providers;

#[tauri::command]
pub fn create_ort_session(model_path: String) -> Result<String, String> {
    use ort::session::Session;

    let eps = build_execution_providers();

    if eps.is_empty() {
        return Err("No execution providers available".to_string());
    }

    Session::builder()
        .and_then(|b| b.with_execution_providers(eps))
        .and_then(|b| b.commit_from_file(&model_path))
        .map_err(|e| format!("Failed to create session: {}", e))?;

    Ok("Session created successfully".to_string())
}
