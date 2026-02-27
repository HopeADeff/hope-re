use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Clone, Serialize)]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
    pub hostname: String,
}

pub fn get_platform_info() -> PlatformInfo {
    PlatformInfo {
        os: format_os_info(),
        arch: get_arch(),
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
    }
}

fn format_os_info() -> String {
    let os_type = get_os_type();
    let version = System::os_version().unwrap_or_else(|| String::new());

    if version.is_empty() {
        os_type
    } else {
        format!("{} {}", os_type, version)
    }
}

fn get_os_type() -> String {
    #[cfg(target_os = "windows")]
    return "Windows".to_string();

    #[cfg(target_os = "macos")]
    return "macOS".to_string();

    #[cfg(target_os = "linux")]
    return "Linux".to_string();

    #[cfg(target_os = "ios")]
    return "iOS".to_string();

    #[cfg(target_os = "android")]
    return "Android".to_string();

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "ios",
        target_os = "android"
    )))]
    return "Unknown".to_string();
}

fn get_arch() -> String {
    #[cfg(target_arch = "x86_64")]
    return "x64".to_string();

    #[cfg(target_arch = "aarch64")]
    return "ARM64".to_string();

    #[cfg(target_arch = "x86")]
    return "x86".to_string();

    #[cfg(target_arch = "arm")]
    return "ARM".to_string();

    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "x86",
        target_arch = "arm"
    )))]
    return "Unknown".to_string();
}
