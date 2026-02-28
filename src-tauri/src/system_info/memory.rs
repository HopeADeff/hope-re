use sysinfo::System;

pub fn get_memory_info() -> String {
    let mut sys = System::new();
    sys.refresh_memory();

    let memory_gb = sys.total_memory() / 1024 / 1024 / 1024;
    format!("{}GB", memory_gb)
}
