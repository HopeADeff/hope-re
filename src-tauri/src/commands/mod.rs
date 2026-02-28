pub mod system_info;

pub use self::system_info::get_system_info;

#[cfg(not(all(target_os = "android", not(target_arch = "aarch64"))))]
pub use crate::onnx_integration::{
    check_models_status, create_ort_session, download_model, get_inference_capabilities,
    protect_image,
};

#[cfg(all(target_os = "android", not(target_arch = "aarch64")))]
pub use crate::onnx_stubs::{
    check_models_status, create_ort_session, download_model, get_inference_capabilities,
    protect_image,
};
