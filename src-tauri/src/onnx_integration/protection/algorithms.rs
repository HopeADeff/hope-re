use ndarray::Array4;
use ort::session::Session;
use ort::value::Tensor;

use super::types::AlgorithmParams;

pub fn get_noise_params(intensity: f32) -> AlgorithmParams {
    AlgorithmParams {
        epsilon: intensity * 0.16 / 0.5,
        max_iterations: 80,
        alpha_multiplier: 5.0,
        perceptual_weight: 0.3,
    }
}

pub fn get_glaze_params(intensity: f32) -> AlgorithmParams {
    AlgorithmParams {
        epsilon: intensity * 0.10 / 0.5,
        max_iterations: 100,
        alpha_multiplier: 4.5,
        perceptual_weight: 0.6,
    }
}

pub fn get_nightshade_params(intensity: f32) -> AlgorithmParams {
    AlgorithmParams {
        epsilon: intensity * 0.12 / 0.5,
        max_iterations: 120,
        alpha_multiplier: 5.0,
        perceptual_weight: 0.8,
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

fn create_image_tensor(input: &Array4<f32>) -> Result<Tensor<f32>, String> {
    let shape = input.shape();
    let data: Vec<f32> = input.iter().copied().collect();
    Tensor::from_array((
        [shape[0], shape[1], shape[2], shape[3]],
        data.into_boxed_slice(),
    ))
    .map_err(|e| format!("Failed to create input tensor: {}", e))
}

pub fn run_noise_model(session: &mut Session, input: &Array4<f32>) -> Result<f32, String> {
    let input_tensor = create_image_tensor(input)?;

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
    let input_tensor = create_image_tensor(input)?;

    let style_tensor = Tensor::from_array(([1_usize], vec![style_index]))
        .map_err(|e| format!("Failed to create style index tensor: {}", e))?;

    let outputs = session
        .run(ort::inputs![input_tensor, style_tensor])
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
    let input_tensor = create_image_tensor(input)?;

    let target_tensor = Tensor::from_array(([1_usize], vec![target_index]))
        .map_err(|e| format!("Failed to create target index tensor: {}", e))?;

    let outputs = session
        .run(ort::inputs![input_tensor, target_tensor])
        .map_err(|e| format!("Failed to run nightshade model: {}", e))?;

    outputs[0]
        .try_extract_scalar::<f32>()
        .map_err(|e| format!("Failed to extract nightshade model output: {}", e))
}
