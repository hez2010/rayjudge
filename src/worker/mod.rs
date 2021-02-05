#[cfg(target_os = "linux")]
pub mod linux_worker;
#[cfg(target_os = "windows")]
pub mod windows_worker;

pub mod worker;