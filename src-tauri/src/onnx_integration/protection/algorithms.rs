use ndarray::Array4;
use ort::session::Session;
use ort::value::Tensor;

use super::preprocessing::array4_to_tensor_ref;
use super::types::AlgorithmParams;

pub fn get_noise_params(intensity: f32) -> AlgorithmParams {
    AlgorithmParams {
        epsilon: intensity * 0.08 / 0.5,
        max_iterations: 250,
        alpha_multiplier: 2.5,
        perceptual_weight: 0.5,
    }
}

pub fn get_glaze_params(intensity: f32) -> AlgorithmParams {
    AlgorithmParams {
        epsilon: intensity * 0.05 / 0.5,
        max_iterations: 350,
        alpha_multiplier: 2.0,
        perceptual_weight: 1.0,
    }
}

pub fn get_nightshade_params(intensity: f32) -> AlgorithmParams {
    AlgorithmParams {
        epsilon: intensity * 0.045 / 0.5,
        max_iterations: 500,
        alpha_multiplier: 2.5,
        perceptual_weight: 1.5,
    }
}

pub fn get_glaze_style_index(style: &str) -> i32 {
    match style {
        "abstract" => 0,
        "impressionist" => 1,
        "cubist" => 2,
        "sketch" => 3,
        "watercolor" => 4,
        _ => 0,
    }
}

pub fn get_nightshade_target_index(target: &str) -> i32 {
    match target {
        "dog" => 0,
        "cat" => 1,
        "car" => 2,
        "landscape" => 3,
        "person" => 4,
        "building" => 5,
        "food" => 6,
        "abstract" => 7,
        _ => 0,
    }
}

pub fn run_noise_model(session: &mut Session, input: &Array4<f32>) -> Result<f32, String> {
    let input_tensor = array4_to_tensor_ref(input)?;

    let outputs = session
        .run(ort::inputs![input_tensor])
        .map_err(|e| format!("Failed to run noise model: {}", e))?;

    outputs[0]
        .try_extract_scalar::<f32>()
        .map_err(|e| format!("Failed to extract noise model output: {}", e))
}

pub fn run_glaze_model(
    session: &mut Session,
    input: &Array4<f32>,
    style_index: i32,
) -> Result<f32, String> {
    let input_tensor = array4_to_tensor_ref(input)?;

    let style_tensor = Tensor::from_array(([1_usize], vec![style_index]))
        .map_err(|e| format!("Failed to create style index tensor: {}", e))?;

    let outputs = session
        .run(ort::inputs![
            "input" => input_tensor,
            "style_index" => style_tensor
        ])
        .map_err(|e| format!("Failed to run glaze model: {}", e))?;

    outputs[0]
        .try_extract_scalar::<f32>()
        .map_err(|e| format!("Failed to extract glaze model output: {}", e))
}

pub fn run_nightshade_model(
    session: &mut Session,
    input: &Array4<f32>,
    target_index: i32,
) -> Result<f32, String> {
    let input_tensor = array4_to_tensor_ref(input)?;

    let target_tensor = Tensor::from_array(([1_usize], vec![target_index]))
        .map_err(|e| format!("Failed to create target index tensor: {}", e))?;

    let outputs = session
        .run(ort::inputs![
            "input" => input_tensor,
            "target_index" => target_tensor
        ])
        .map_err(|e| format!("Failed to run nightshade model: {}", e))?;

    outputs[0]
        .try_extract_scalar::<f32>()
        .map_err(|e| format!("Failed to extract nightshade model output: {}", e))
}
