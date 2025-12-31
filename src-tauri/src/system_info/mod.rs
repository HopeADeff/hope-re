pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod platform;
pub mod storage;

pub use cpu::get_cpu_info;
pub use gpu::get_gpu_info;
pub use memory::get_memory_info;
pub use platform::{get_platform_info, PlatformInfo};
pub use storage::get_storage_info;
