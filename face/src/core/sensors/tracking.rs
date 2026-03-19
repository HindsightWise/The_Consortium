use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A Masterpiece-grade Memory Forensics Allocator
/// Designed to track the exact physical memory footprint of The Company on the M1 substrate.
pub struct SovereignAllocator {
    pub allocated_bytes: AtomicUsize,
    pub deallocated_bytes: AtomicUsize,
}

impl Default for SovereignAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignAllocator {
    pub const fn new() -> Self {
        Self {
            allocated_bytes: AtomicUsize::new(0),
            deallocated_bytes: AtomicUsize::new(0),
        }
    }

    pub fn current_usage(&self) -> usize {
        let alloc = self.allocated_bytes.load(Ordering::Relaxed);
        let dealloc = self.deallocated_bytes.load(Ordering::Relaxed);
        alloc.saturating_sub(dealloc)
    }
}

unsafe impl GlobalAlloc for SovereignAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocated_bytes.fetch_add(layout.size(), Ordering::Relaxed);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.deallocated_bytes.fetch_add(layout.size(), Ordering::Relaxed);
        System.dealloc(ptr, layout)
    }
}