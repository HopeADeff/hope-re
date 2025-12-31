use sysinfo::Disks;

pub fn get_storage_info() -> String {
    let disks = Disks::new_with_refreshed_list();
    let total_storage: u64 = disks.iter().map(|d| d.total_space()).sum();
    let storage_gb = total_storage / 1024 / 1024 / 1024;
    format!("{}GB", storage_gb)
}
