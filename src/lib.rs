mod alloc_wrapper;

pub use alloc_wrapper::Allocator;

#[cfg(feature = "global")]
#[global_allocator]
pub static A: Allocator = Allocator::new();

pub type AllocStats = Vec<(&'static str, usize)>;

pub use failure::Error;
pub type Result<T> = failure::Fallible<T>;

#[cfg(all(feature = "alloc", feature = "system_alloc"))]
#[path = "system.rs"]
mod imp;

#[cfg(all(feature = "alloc", feature = "jemalloc"))]
#[path = "jemalloc.rs"]
mod imp;

#[cfg(all(feature = "alloc", feature = "ralloc"))]
#[path = "ralloc.rs"]
mod imp;

#[cfg(all(feature = "alloc", feature = "tcmalloc"))]
#[path = "tcmalloc.rs"]
mod imp;

#[cfg(not(feature = "alloc"))]
#[path = "no_allocator.rs"]
mod imp;
