compile_error!("no allocator selected. turn on an allocator feature");

use std::alloc::{GlobalAlloc, Layout};
use crate::{AllocStats, Result};

pub struct Allocator;

impl Allocator {
    pub const fn new() -> Allocator { Allocator }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        panic!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        panic!()
    }
}

impl Allocator {
    pub fn name(&self) -> &'static str { panic!() }

    pub fn fetch_stats(&self) -> Result<Option<AllocStats>> {
        panic!()
    }
}
