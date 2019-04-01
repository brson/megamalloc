use std::alloc::{GlobalAlloc, Layout, System};
use crate::{AllocStats, Result};

pub struct Allocator(System);

impl Allocator {
    pub const fn new() -> Allocator {
        Allocator(System)
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
    pub fn name(&self) -> &'static str { "system-glibc" }

    pub fn fetch_stats(&self) -> Result<Option<AllocStats>> {
        fetch_stats()
    }
}

// usually correct ...
#[allow(bad_style)]
type c_int = i32;

// https://www.gnu.org/software/libc/manual/html_node/Statistics-of-Malloc.html
#[repr(C)]
#[allow(bad_style)]
struct mallinfo {
    arena: c_int,
    ordblks: c_int,
    smblks: c_int,
    hblks: c_int,
    hblkhd: c_int,
    usmblks: c_int,
    fsmblks: c_int,
    uordblks: c_int,
    fordblks: c_int,
    keepcost: c_int,
}

extern "C" {
    fn mallinfo() -> mallinfo;
}

fn fetch_stats() -> Result<Option<AllocStats>> {
    let info = unsafe { mallinfo() };

    Ok(Some(vec![
        ("arena", info.arena as usize),
        ("ordblks", info.ordblks as usize),
        ("smblks", info.smblks as usize),
        ("hblks", info.hblks as usize),
        ("hblkhd", info.hblkhd as usize),
        ("usmblks", info.usmblks as usize),
        ("fsmblks", info.fsmblks as usize),
        ("uordblks", info.uordblks as usize),
        ("fordblks", info.fordblks as usize),
        ("keepcost", info.keepcost as usize),
    ]))
}
