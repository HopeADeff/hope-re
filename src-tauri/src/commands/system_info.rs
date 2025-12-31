use serde::Serialize;

use crate::system_info::{self, PlatformInfo};

#[derive(Debug, Clone, Serialize)]
pub struct SystemInfo {
    pub platform: PlatformInfo,
    pub cpu: String,
    pub memory: String,
    pub gpu: String,
    pub storage: String,
}

#[tauri::command]
pub fn get_system_info() -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        platform: system_info::get_platform_info(),
        cpu: system_info::get_cpu_info(),
        memory: system_info::get_memory_info(),
        gpu: system_info::get_gpu_info(),
        storage: system_info::get_storage_info(),
    })
}
