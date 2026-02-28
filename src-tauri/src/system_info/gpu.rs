use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct GpuDetails {
    name: String,
    vram_mb: Option<u64>,
}

pub fn has_nvidia_gpu() -> bool {
    let gpu = detect_gpu();
    gpu.name.to_uppercase().contains("NVIDIA")
}

pub fn get_gpu_info() -> String {
    let gpu_details = detect_gpu();

    if let Some(vram) = gpu_details.vram_mb {
        let vram_gb = vram / 1024;
        if vram_gb > 0 {
            format!("{} ({} GB)", gpu_details.name, vram_gb)
        } else {
            format!("{} ({} MB)", gpu_details.name, vram)
        }
    } else {
        gpu_details.name
    }
}

fn detect_gpu() -> GpuDetails {
    #[cfg(target_os = "windows")]
    {
        detect_gpu_windows()
    }

    #[cfg(target_os = "macos")]
    {
        detect_gpu_macos()
    }

    #[cfg(target_os = "linux")]
    {
        detect_gpu_linux()
    }

    #[cfg(target_os = "ios")]
    {
        detect_gpu_ios()
    }

    #[cfg(target_os = "android")]
    {
        detect_gpu_android()
    }

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "ios",
        target_os = "android"
    )))]
    {
        GpuDetails {
            name: "N/A".to_string(),
            vram_mb: None,
        }
    }
}

#[cfg(target_os = "windows")]
fn detect_gpu_windows() -> GpuDetails {
    if let Some(gpu) = detect_gpu_windows_dxgi() {
        return gpu;
    }

    detect_gpu_windows_wmic()
}

#[cfg(target_os = "windows")]
fn detect_gpu_windows_dxgi() -> Option<GpuDetails> {
    use windows::Win32::Graphics::Dxgi::*;
    use windows::Win32::System::Com::*;

    unsafe {
        if CoInitializeEx(None, COINIT_MULTITHREADED).is_err() {
            return None;
        }

        let factory: IDXGIFactory1 = match CreateDXGIFactory1() {
            Ok(f) => f,
            Err(_) => {
                CoUninitialize();
                return None;
            }
        };

        let adapter = match factory.EnumAdapters1(0) {
            Ok(a) => a,
            Err(_) => {
                CoUninitialize();
                return None;
            }
        };

        let desc = match adapter.GetDesc1() {
            Ok(d) => d,
            Err(_) => {
                CoUninitialize();
                return None;
            }
        };

        let name = String::from_utf16_lossy(&desc.Description)
            .trim_end_matches('\0')
            .to_string();

        let vram_mb = (desc.DedicatedVideoMemory / 1024 / 1024) as u64;

        CoUninitialize();

        Some(GpuDetails {
            name,
            vram_mb: if vram_mb > 0 { Some(vram_mb) } else { None },
        })
    }
}

#[cfg(target_os = "windows")]
fn detect_gpu_windows_wmic() -> GpuDetails {
    use std::process::Command;

    let output = Command::new("wmic")
        .args(["path", "win32_VideoController", "get", "name"])
        .output();

    if let Ok(output) = output {
        let gpu_output = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = gpu_output.lines().collect();

        if lines.len() > 1 {
            let gpu_name = lines[1].trim();
            if !gpu_name.is_empty() {
                return GpuDetails {
                    name: gpu_name.to_string(),
                    vram_mb: None,
                };
            }
        }
    }

    GpuDetails {
        name: "N/A".to_string(),
        vram_mb: None,
    }
}

#[cfg(target_os = "macos")]
fn detect_gpu_macos() -> GpuDetails {
    use std::process::Command;

    let output = Command::new("system_profiler")
        .args(["SPDisplaysDataType"])
        .output();

    if let Ok(output) = output {
        let gpu_output = String::from_utf8_lossy(&output.stdout);
        let mut gpu_name = String::new();
        let mut vram_mb: Option<u64> = None;

        for line in gpu_output.lines() {
            let line_trimmed = line.trim();

            if line_trimmed.starts_with("Chipset Model: ") {
                if let Some((_, after)) = line_trimmed.split_once(":  ") {
                    gpu_name = after.trim().to_string();
                }
            }

            if (line_trimmed.contains("VRAM") || line_trimmed.contains("Metal"))
                && line_trimmed.contains(":")
            {
                if let Some((_, after)) = line_trimmed.split_once(": ") {
                    let vram_str = after.trim();
                    vram_mb = parse_vram(vram_str);
                }
            }
        }

        if !gpu_name.is_empty() {
            return GpuDetails {
                name: gpu_name,
                vram_mb,
            };
        }
    }

    GpuDetails {
        name: "N/A".to_string(),
        vram_mb: None,
    }
}

#[cfg(target_os = "linux")]
fn detect_gpu_linux() -> GpuDetails {
    use std::process::Command;

    let output = Command::new("lspci").args(["-v"]).output();

    if let Ok(output) = output {
        let lspci_output = String::from_utf8_lossy(&output.stdout);
        let mut gpu_name = String::new();
        let mut vram_mb: Option<u64> = None;
        let mut in_vga_section = false;

        for line in lspci_output.lines() {
            if line.contains("VGA compatible controller") || line.contains("3D controller") {
                if let Some(gpu_part) = line.split(':').nth(2) {
                    gpu_name = gpu_part.trim().to_string();
                    in_vga_section = true;
                }
            } else if in_vga_section && line.contains("Memory at") && line.contains("size=") {
                if let Some(size_part) = line.split("size=").nth(1) {
                    let size_str = size_part.split(']').next().unwrap_or("").trim();
                    vram_mb = parse_vram(size_str);
                    break;
                }
            } else if in_vga_section && line.is_empty() {
                break;
            }
        }

        if !gpu_name.is_empty() {
            return GpuDetails {
                name: gpu_name,
                vram_mb,
            };
        }
    }

    GpuDetails {
        name: "N/A".to_string(),
        vram_mb: None,
    }
}

#[cfg(target_os = "ios")]
fn detect_gpu_ios() -> GpuDetails {
    GpuDetails {
        name: "Apple GPU (Metal)".to_string(),
        vram_mb: None,
    }
}

#[cfg(target_os = "android")]
fn detect_gpu_android() -> GpuDetails {
    use std::process::Command;

    let output = Command::new("getprop").args(["ro.hardware"]).output();

    if let Ok(output) = output {
        let hardware = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !hardware.is_empty() {
            return GpuDetails {
                name: format!("{} GPU", hardware),
                vram_mb: None,
            };
        }
    }

    GpuDetails {
        name: "Mobile GPU".to_string(),
        vram_mb: None,
    }
}

#[allow(dead_code)]
fn parse_vram(vram_str: &str) -> Option<u64> {
    let vram_lower = vram_str.to_lowercase();

    let number: u64 = vram_str
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .ok()?;

    if vram_lower.contains("gb") {
        Some(number * 1024)
    } else if vram_lower.contains("mb") {
        Some(number)
    } else if vram_lower.contains('g') {
        Some(number * 1024)
    } else {
        Some(number)
    }
}
