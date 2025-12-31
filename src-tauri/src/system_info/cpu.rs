use sysinfo::System;

pub fn get_cpu_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string())
}
