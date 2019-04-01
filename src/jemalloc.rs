use _jemallocator::Jemalloc;
use _jemalloc_ctl::{stats, epoch};
use std::alloc::{GlobalAlloc, Layout};
use crate::{AllocStats, Result};

pub struct Allocator(Jemalloc);

impl Allocator {
    pub const fn new() -> Allocator {
        Allocator(Jemalloc)
    }
}

unsafe impl GlobalAlloc for Allocator {
    #[inline(always)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.alloc(layout)
    }

    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.dealloc(ptr, layout)
    }
}

impl Allocator {
    pub fn name(&self) -> &'static str { "jemalloc" }

    pub fn fetch_stats(&self) -> Result<Option<AllocStats>> {
        // Stats are cached. Need to advance epoch to refresh.
        epoch::advance()?;

        Ok(Some(vec![
            ("allocated", stats::allocated::read()?),
            ("active", stats::active::read()?),
            ("metadata", stats::metadata::read()?),
            ("resident", stats::resident::read()?),
            ("mapped", stats::mapped::read()?),
            ("retained", stats::retained::read()?),
        ]))
    }
}
