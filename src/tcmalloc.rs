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
        fetch_stats()
    }
}

type c_char = u8;
type size_t = usize;

extern "C" {
    fn MallocExtension_GetNumericProperty(property: &c_char, value: &mut size_t);
}

fn fetch_stats() -> Result<Option<AllocStats>> {
    let mut generic_current_allocated_bytes = 0;
    let mut generic_heap_size = 0;
    let mut tcmalloc_pageheap_free_bytes = 0;
    let mut tcmalloc_pageheap_unmapped_bytes = 0;
    let mut tcmalloc_slack_bytes = 0;
    let mut tcmalloc_max_total_thread_cache_bytes = 0;
    let mut tcmalloc_current_total_thread_cache_bytes = 0;

    unsafe {
        MallocExtension_GetNumericProperty(&b"generic.current_allocated_bytes\0"[0], &mut generic_current_allocated_bytes);
        MallocExtension_GetNumericProperty(&b"generic.heap_size\0"[0], &mut generic_heap_size);
        MallocExtension_GetNumericProperty(&b"tcmalloc.pageheap_free_bytes\0"[0], &mut tcmalloc_pageheap_free_bytes);
        MallocExtension_GetNumericProperty(&b"tcmalloc.pageheap_unmapped_bytes\0"[0], &mut tcmalloc_pageheap_unmapped_bytes);
        MallocExtension_GetNumericProperty(&b"tcmalloc.slack_bytes\0"[0], &mut tcmalloc_slack_bytes);
        MallocExtension_GetNumericProperty(&b"tcmalloc.max_total_thread_cache_bytes\0"[0], &mut tcmalloc_max_total_thread_cache_bytes);
        MallocExtension_GetNumericProperty(&b"tcmalloc.current_total_thread_cache_bytes\0"[0], &mut tcmalloc_current_total_thread_cache_bytes);
    }

    Ok(Some(vec![
        ("current_allocated_bytes", generic_current_allocated_bytes),
        ("heap_size", generic_heap_size),
        ("pageheap_free_bytes", tcmalloc_pageheap_free_bytes),
        ("pagehap_unmapped_bytes", tcmalloc_pageheap_unmapped_bytes),
        ("slack_bytes", tcmalloc_slack_bytes),
        ("max_total_thread_cache_bytes", tcmalloc_max_total_thread_cache_bytes),
        ("current_allocated_bytes", tcmalloc_current_total_thread_cache_bytes),
    ]))
}
