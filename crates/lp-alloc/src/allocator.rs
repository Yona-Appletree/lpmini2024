use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(any(feature = "std", test))]
use std::alloc::{GlobalAlloc, Layout, System};
#[cfg(any(feature = "std", test))]
use std::eprintln;
#[cfg(any(feature = "std", test))]
use std::thread_local;

#[cfg(all(not(feature = "std"), not(test)))]
use core::alloc::{GlobalAlloc, Layout};

#[cfg(all(not(feature = "std"), not(test)))]
extern crate alloc;
#[cfg(all(not(feature = "std"), not(test)))]
use alloc::alloc::alloc as system_alloc;
#[cfg(all(not(feature = "std"), not(test)))]
use alloc::alloc::alloc_zeroed as system_alloc_zeroed;
#[cfg(all(not(feature = "std"), not(test)))]
use alloc::alloc::dealloc as system_dealloc;
#[cfg(all(not(feature = "std"), not(test)))]
use alloc::alloc::realloc as system_realloc;

// Global allocated counter (shared across all threads)
static GLOBAL_ALLOCATED: AtomicUsize = AtomicUsize::new(0);

// Limits: thread-local when std is available, global when not
#[cfg(any(feature = "std", test))]
thread_local! {
    static HARD_LIMIT: core::cell::Cell<usize> = const { core::cell::Cell::new(usize::MAX) };
    static SOFT_LIMIT: core::cell::Cell<usize> = const { core::cell::Cell::new(usize::MAX) };
}

#[cfg(all(not(feature = "std"), not(test)))]
static GLOBAL_HARD_LIMIT: AtomicUsize = AtomicUsize::new(usize::MAX);
#[cfg(all(not(feature = "std"), not(test)))]
static GLOBAL_SOFT_LIMIT: AtomicUsize = AtomicUsize::new(usize::MAX);

/// Global allocator wrapper with hard and soft memory limits
/// All instances share the same tracking state, so you can use any instance
/// as the global allocator and the tracking will work correctly.
#[derive(Copy, Clone)]
pub struct LimitedAllocator;

impl LimitedAllocator {
    pub const fn new() -> Self {
        LimitedAllocator
    }

    pub fn set_hard_limit(&self, limit_bytes: usize) {
        #[cfg(any(feature = "std", test))]
        {
            HARD_LIMIT.with(|limit| limit.set(limit_bytes));
        }
        #[cfg(all(not(feature = "std"), not(test)))]
        {
            GLOBAL_HARD_LIMIT.store(limit_bytes, Ordering::Relaxed);
        }
    }

    pub fn set_soft_limit(&self, limit_bytes: usize) {
        #[cfg(any(feature = "std", test))]
        {
            SOFT_LIMIT.with(|limit| limit.set(limit_bytes));
        }
        #[cfg(all(not(feature = "std"), not(test)))]
        {
            GLOBAL_SOFT_LIMIT.store(limit_bytes, Ordering::Relaxed);
        }
    }

    pub fn soft_limit(&self) -> usize {
        #[cfg(any(feature = "std", test))]
        {
            SOFT_LIMIT.with(|limit| limit.get())
        }
        #[cfg(all(not(feature = "std"), not(test)))]
        {
            GLOBAL_SOFT_LIMIT.load(Ordering::Relaxed)
        }
    }

    pub fn allocated_bytes(&self) -> usize {
        GLOBAL_ALLOCATED.load(Ordering::Relaxed)
    }

    #[cfg(any(feature = "std", test))]
    fn hard_limit(&self) -> usize {
        HARD_LIMIT.with(|limit| limit.get())
    }

    #[cfg(all(not(feature = "std"), not(test)))]
    fn hard_limit(&self) -> usize {
        GLOBAL_HARD_LIMIT.load(Ordering::Relaxed)
    }
}

unsafe impl GlobalAlloc for LimitedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let current = GLOBAL_ALLOCATED.fetch_add(size, Ordering::Relaxed);
        let new_total = current + size;

        let hard_limit = self.hard_limit();
        if new_total > hard_limit {
            // Print to stderr and abort immediately - don't panic (avoid recursion)
            #[cfg(any(feature = "std", test))]
            {
                eprintln!("\n!!! HARD MEMORY LIMIT EXCEEDED !!!");
                eprintln!("Current: {} bytes / Limit: {} bytes", new_total, hard_limit);
                eprintln!("Attempted allocation: {} bytes", size);
                eprintln!("Aborting to prevent system crash...\n");
                std::process::abort();
            }
            #[cfg(all(not(feature = "std"), not(test)))]
            {
                // In no_std, we can't print or abort safely, so panic
                // This is not ideal but necessary for no_std environments
                panic!(
                    "HARD MEMORY LIMIT EXCEEDED: {} bytes / Limit: {} bytes",
                    new_total, hard_limit
                );
            }
        }

        #[cfg(any(feature = "std", test))]
        let ptr = System.alloc(layout);
        #[cfg(all(not(feature = "std"), not(test)))]
        let ptr = system_alloc(layout);

        if ptr.is_null() {
            // Allocation failed, revert the counter
            GLOBAL_ALLOCATED.fetch_sub(size, Ordering::Relaxed);
        }

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        #[cfg(any(feature = "std", test))]
        System.dealloc(ptr, layout);
        #[cfg(all(not(feature = "std"), not(test)))]
        system_dealloc(ptr, layout);
        GLOBAL_ALLOCATED.fetch_sub(layout.size(), Ordering::Relaxed);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let current = GLOBAL_ALLOCATED.fetch_add(size, Ordering::Relaxed);
        let new_total = current + size;

        let hard_limit = self.hard_limit();
        if new_total > hard_limit {
            #[cfg(any(feature = "std", test))]
            {
                eprintln!("\n!!! HARD MEMORY LIMIT EXCEEDED !!!");
                eprintln!("Current: {} bytes / Limit: {} bytes", new_total, hard_limit);
                eprintln!("Attempted allocation: {} bytes", size);
                eprintln!("Aborting to prevent system crash...\n");
                std::process::abort();
            }
            #[cfg(all(not(feature = "std"), not(test)))]
            {
                // In no_std, we can't print or abort safely, so panic
                // This is not ideal but necessary for no_std environments
                panic!(
                    "HARD MEMORY LIMIT EXCEEDED: {} bytes / Limit: {} bytes",
                    new_total, hard_limit
                );
            }
        }

        #[cfg(any(feature = "std", test))]
        let ptr = System.alloc_zeroed(layout);
        #[cfg(all(not(feature = "std"), not(test)))]
        let ptr = system_alloc_zeroed(layout);

        if ptr.is_null() {
            GLOBAL_ALLOCATED.fetch_sub(size, Ordering::Relaxed);
        }

        ptr
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let old_size = layout.size();
        let size_diff = new_size.saturating_sub(old_size);

        if size_diff > 0 {
            let current = GLOBAL_ALLOCATED.fetch_add(size_diff, Ordering::Relaxed);
            let new_total = current + size_diff;

            let hard_limit = self.hard_limit();
            if new_total > hard_limit {
                // Revert the counter before aborting
                GLOBAL_ALLOCATED.fetch_sub(size_diff, Ordering::Relaxed);
                #[cfg(any(feature = "std", test))]
                {
                    eprintln!("\n!!! HARD MEMORY LIMIT EXCEEDED !!!");
                    eprintln!("Current: {} bytes / Limit: {} bytes", new_total, hard_limit);
                    eprintln!(
                        "Attempted reallocation: {} bytes -> {} bytes",
                        old_size, new_size
                    );
                    eprintln!("Aborting to prevent system crash...\n");
                    std::process::abort();
                }
                #[cfg(all(not(feature = "std"), not(test)))]
                {
                    // In no_std, we can't print or abort safely, so panic
                    // This is not ideal but necessary for no_std environments
                    panic!("HARD MEMORY LIMIT EXCEEDED: {} bytes / Limit: {} bytes (realloc: {} -> {})", new_total, hard_limit, old_size, new_size);
                }
            }
        } else {
            // Shrinking - update counter
            let size_reduction = old_size - new_size;
            GLOBAL_ALLOCATED.fetch_sub(size_reduction, Ordering::Relaxed);
        }

        #[cfg(any(feature = "std", test))]
        let new_ptr = System.realloc(ptr, layout, new_size);
        #[cfg(all(not(feature = "std"), not(test)))]
        let new_ptr = system_realloc(ptr, layout, new_size);

        if new_ptr.is_null() && size_diff > 0 {
            // Reallocation failed, revert the counter
            GLOBAL_ALLOCATED.fetch_sub(size_diff, Ordering::Relaxed);
        }

        new_ptr
    }
}
