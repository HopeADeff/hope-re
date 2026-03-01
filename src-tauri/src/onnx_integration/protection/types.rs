use ndarray::Array4;
use ort::session::Session;

pub const TILE_SIZE: u32 = 224;
pub const TILE_OVERLAP: u32 = 32;
pub const SPSA_DIRECTIONS_PER_ITER: usize = 8;

pub type ModelRunFn = dyn FnMut(&mut Session, &Array4<f32>) -> Result<f32, String>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProtectionSettings {
    pub algorithm: String,
    pub intensity: f32,
    pub output_quality: u8,
    pub render_quality: u8,
    pub glaze_style: Option<String>,
    pub nightshade_target: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProtectionResult {
    pub image_base64: String,
    pub success: bool,
    pub message: String,
    pub model_used: bool,
}

pub struct TileProgress {
    pub app: tauri::AppHandle,
    pub tile_current: u32,
    pub tile_total: u32,
}

#[derive(Debug, Clone)]
pub struct AlgorithmParams {
    pub epsilon: f32,
    pub max_iterations: u32,
    pub alpha_multiplier: f32,
    pub perceptual_weight: f32,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProtectionProgress {
    pub stage: String,
    pub tile_current: u32,
    pub tile_total: u32,
    pub iteration_current: u32,
    pub iteration_total: u32,
    pub percent: f64,
}
