pub use imp::Allocator;

#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[path = "system_glibc.rs"]
mod imp;

#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
#[path = "system_generic.rs"]
mod imp;

