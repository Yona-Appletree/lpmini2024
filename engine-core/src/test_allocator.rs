/// Test-only allocator with hard memory limit
/// 
/// Wraps the system allocator and tracks total allocated memory.
/// Panics immediately when limit is exceeded.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct LimitedAllocator {
    allocated: AtomicUsize,
    limit: usize,
}

impl LimitedAllocator {
    pub const fn new(limit_mb: usize) -> Self {
        LimitedAllocator {
            allocated: AtomicUsize::new(0),
            limit: limit_mb * 1024 * 1024,
        }
    }
    
    pub fn allocated_bytes(&self) -> usize {
        self.allocated.load(Ordering::Relaxed)
    }
    
    pub fn allocated_mb(&self) -> usize {
        self.allocated_bytes() / (1024 * 1024)
    }
}

unsafe impl GlobalAlloc for LimitedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let current = self.allocated.fetch_add(size, Ordering::Relaxed);
        let new_total = current + size;
        
        if new_total > self.limit {
            // Print to stderr and abort immediately - don't panic (avoid recursion)
            eprintln!("\n!!! MEMORY LIMIT EXCEEDED !!!");
            eprintln!("Current: {}MB / Limit: {}MB", new_total / (1024 * 1024), self.limit / (1024 * 1024));
            eprintln!("Attempted allocation: {} bytes", size);
            eprintln!("Aborting to prevent system crash...\n");
            std::process::abort();
        }
        
        // Log large allocations (>10MB)
        if size > 10 * 1024 * 1024 {
            eprintln!("WARNING: Large allocation: {}MB", size / (1024 * 1024));
        }
        
        let ptr = System.alloc(layout);
        
        if ptr.is_null() {
            // Allocation failed, revert the counter
            self.allocated.fetch_sub(size, Ordering::Relaxed);
        }
        
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        self.allocated.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_allocator_tracks_memory() {
        let allocator = LimitedAllocator::new(10); // 10MB limit
        
        unsafe {
            let layout = Layout::from_size_align(1024, 8).unwrap();
            let ptr = allocator.alloc(layout);
            
            assert!(!ptr.is_null());
            assert!(allocator.allocated_bytes() >= 1024);
            
            allocator.dealloc(ptr, layout);
        }
    }
    
    #[test]
    #[should_panic(expected = "MEMORY LIMIT EXCEEDED")]
    fn test_allocator_enforces_limit() {
        let allocator = LimitedAllocator::new(1); // 1MB limit
        
        unsafe {
            // Try to allocate 10MB - should panic
            let layout = Layout::from_size_align(10 * 1024 * 1024, 8).unwrap();
            let _ = allocator.alloc(layout);
        }
    }
}

