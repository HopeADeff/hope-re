pub mod system_info;

pub use system_info::get_system_info;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ExecutionProviderInfo {
    pub name: String,
    pub available: bool,
    pub registered: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct InferenceCapabilities {
    pub providers: Vec<ExecutionProviderInfo>,
    pub platform: String,
}

fn get_platform_name() -> String {
    #[cfg(target_os = "windows")]
    return "Windows x64".to_string();

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    return "macOS x64".to_string();

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    return "macOS ARM64".to_string();

    #[cfg(target_os = "ios")]
    return "iOS".to_string();

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    return "Linux x64".to_string();

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    return "Linux ARM64".to_string();

    #[cfg(target_os = "android")]
    return "Android".to_string();

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "linux",
        target_os = "android"
    )))]
    return "Unknown".to_string();
}

#[cfg(target_os = "windows")]
fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
    use ort::ep::{DirectML, ExecutionProvider, CUDA};

    let mut eps: Vec<ort::ep::ExecutionProviderDispatch> = Vec::new();

    if CUDA::default().is_available().unwrap_or(false) {
        eps.push(CUDA::default().build());
    }
    if DirectML::default().is_available().unwrap_or(false) {
        eps.push(DirectML::default().build());
    }

    eps
}

#[cfg(target_os = "macos")]
fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
    use ort::ep::{CoreML, ExecutionProvider};

    let mut eps: Vec<ort::ep::ExecutionProviderDispatch> = Vec::new();

    if CoreML::default().is_available().unwrap_or(false) {
        let coreml = CoreML::default()
            .with_subgraphs()
            .with_compute_units(ort::ep::coreml::ComputeUnits::CPUAndNeuralEngine);
        eps.push(coreml.build());
    }

    eps
}

#[cfg(target_os = "ios")]
fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
    use ort::ep::{CoreML, ExecutionProvider};

    let mut eps: Vec<ort::ep::ExecutionProviderDispatch> = Vec::new();

    if CoreML::default().is_available().unwrap_or(false) {
        let coreml = CoreML::default()
            .with_subgraphs()
            .with_compute_units(ort::ep::coreml::ComputeUnits::CPUAndNeuralEngine);
        eps.push(coreml.build());
    }

    eps
}

#[cfg(all(
    target_os = "linux",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
    use ort::ep::{ExecutionProvider, CUDA, XNNPACK};

    let mut eps: Vec<ort::ep::ExecutionProviderDispatch> = Vec::new();

    if CUDA::default().is_available().unwrap_or(false) {
        eps.push(CUDA::default().build());
    }
    if XNNPACK::default().is_available().unwrap_or(false) {
        eps.push(XNNPACK::default().build());
    }

    eps
}

#[cfg(target_os = "android")]
fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
    use ort::ep::{ExecutionProvider, XNNPACK};

    let mut eps: Vec<ort::ep::ExecutionProviderDispatch> = Vec::new();

    if XNNPACK::default().is_available().unwrap_or(false) {
        eps.push(XNNPACK::default().build());
    }

    eps
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "linux",
    target_os = "android"
)))]
fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
    Vec::new()
}

fn get_inference_capabilities_internal() -> InferenceCapabilities {
    let platform = get_platform_name();
    let mut providers = Vec::new();

    #[cfg(target_os = "windows")]
    {
        use ort::ep::{DirectML, ExecutionProvider, CUDA};

        providers.push(ExecutionProviderInfo {
            name: "CUDA".to_string(),
            available: CUDA::default().is_available().unwrap_or(false),
            registered: false,
        });

        providers.push(ExecutionProviderInfo {
            name: "DirectML".to_string(),
            available: DirectML::default().is_available().unwrap_or(false),
            registered: false,
        });
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        use ort::ep::{CoreML, ExecutionProvider};

        providers.push(ExecutionProviderInfo {
            name: "CoreML".to_string(),
            available: CoreML::default().is_available().unwrap_or(false),
            registered: false,
        });
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        use ort::ep::{ExecutionProvider, CUDA, XNNPACK};

        providers.push(ExecutionProviderInfo {
            name: "CUDA".to_string(),
            available: CUDA::default().is_available().unwrap_or(false),
            registered: false,
        });

        providers.push(ExecutionProviderInfo {
            name: "XNNPACK".to_string(),
            available: XNNPACK::default().is_available().unwrap_or(false),
            registered: false,
        });
    }

    InferenceCapabilities {
        providers,
        platform,
    }
}

#[tauri::command]
pub fn get_inference_capabilities() -> Result<InferenceCapabilities, String> {
    Ok(get_inference_capabilities_internal())
}

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
