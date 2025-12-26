#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_os::init())
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
