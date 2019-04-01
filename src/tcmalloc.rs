use _tcmalloc::TCMalloc;
use std::alloc::{GlobalAlloc, Layout};
use crate::{AllocStats, Result};

pub struct Allocator(TCMalloc);

impl Allocator {
    pub const fn new() -> Allocator {
        Allocator(TCMalloc)
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
    pub fn name(&self) -> &'static str { "tcmalloc" }

    pub fn fetch_stats(&self) -> Result<Option<AllocStats>> {
        Ok(None)
    }
}
