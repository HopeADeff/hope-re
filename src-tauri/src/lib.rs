mod commands;
#[cfg(not(all(target_os = "android", not(target_arch = "aarch64"))))]
mod onnx_integration;
#[cfg(all(target_os = "android", not(target_arch = "aarch64")))]
mod onnx_stubs;
mod system_info;

use commands::{
    check_models_status, create_ort_session, download_model, get_inference_capabilities,
    get_system_info, protect_image,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            get_inference_capabilities,
            create_ort_session,
            protect_image,
            check_models_status,
            download_model
        ])
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {
                use tauri::Manager;
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_decorations(false);
                }
            }

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
