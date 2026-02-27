use crate::system_info::get_platform_info;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ExecutionProviderInfo {
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct InferenceCapabilities {
    pub providers: Vec<ExecutionProviderInfo>,
    pub platform: String,
}

#[cfg(target_os = "windows")]
pub(super) fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
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
pub(super) fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
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
pub(super) fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
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
pub(super) fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
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
pub(super) fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
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
pub(super) fn build_execution_providers() -> Vec<ort::ep::ExecutionProviderDispatch> {
    Vec::new()
}

fn get_inference_capabilities_internal() -> InferenceCapabilities {
    let platform_info = get_platform_info();
    let platform = format!("{} {}", platform_info.os, platform_info.arch);
    let mut providers = Vec::new();

    #[cfg(target_os = "windows")]
    {
        use ort::ep::{DirectML, ExecutionProvider, CUDA};

        if CUDA::default().is_available().unwrap_or(false) {
            providers.push(ExecutionProviderInfo {
                name: "CUDA".to_string(),
            });
        }

        if DirectML::default().is_available().unwrap_or(false) {
            providers.push(ExecutionProviderInfo {
                name: "DirectML".to_string(),
            });
        }
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        use ort::ep::{CoreML, ExecutionProvider};

        if CoreML::default().is_available().unwrap_or(false) {
            providers.push(ExecutionProviderInfo {
                name: "CoreML".to_string(),
            });
        }
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        use ort::ep::{ExecutionProvider, CUDA, XNNPACK};

        if CUDA::default().is_available().unwrap_or(false) {
            providers.push(ExecutionProviderInfo {
                name: "CUDA".to_string(),
            });
        }

        if XNNPACK::default().is_available().unwrap_or(false) {
            providers.push(ExecutionProviderInfo {
                name: "XNNPACK".to_string(),
            });
        }
    }

    providers.push(ExecutionProviderInfo {
        name: "CPU".to_string(),
    });

    InferenceCapabilities {
        providers,
        platform,
    }
}

#[tauri::command]
pub fn get_inference_capabilities() -> Result<InferenceCapabilities, String> {
    Ok(get_inference_capabilities_internal())
}
