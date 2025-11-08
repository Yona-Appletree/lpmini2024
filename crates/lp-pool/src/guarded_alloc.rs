#![cfg(any(feature = "std", test))]

use std::alloc::{GlobalAlloc, Layout, System};

use crate::state;

/// Execute `f` while temporarily allowing allocations from the system/global allocator.
///
/// This is primarily intended for host-side tests where small pieces of test code still
/// rely on standard library containers (e.g. `alloc::vec![]`, `alloc::format!`). The guard
/// lasts exactly for the duration of the closure.
pub fn with_global_alloc<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let guard = ScopedGlobalAllocBypass::enter();
    let result = f();
    drop(guard);
    result
}

/// Guard that disables the global allocator for the duration of an `LpMemoryPool::run`
/// call. Dropping the guard re-enables the allocator. Created automatically by
/// `LpMemoryPool::run`.
pub struct ScopedGlobalAllocGuard {
    _token: state::GlobalAllocGuardToken,
}

impl ScopedGlobalAllocGuard {
    pub fn enter() -> Self {
        let token = state::enter_global_alloc_guard();
        ScopedGlobalAllocGuard { _token: token }
    }
}

/// RAII handle used by `with_global_alloc` to temporarily allow host allocations.
pub struct ScopedGlobalAllocBypass {
    _token: state::GlobalAllocAllowToken,
}

impl ScopedGlobalAllocBypass {
    pub fn enter() -> Self {
        let token = state::enter_global_alloc_allowance();
        ScopedGlobalAllocBypass { _token: token }
    }
}

/// Global allocator wrapper used in std/test builds to enforce that code inside an
/// `LpMemoryPool::run` scope does not fall back to the host allocator.
pub struct GuardedAllocator;

unsafe impl GlobalAlloc for GuardedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        deny_when_guarded(|| System.alloc(layout))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        deny_when_guarded(|| System.alloc_zeroed(layout))
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        deny_when_guarded(|| System.realloc(ptr, layout, new_size))
    }
}

pub fn guard_active() -> bool {
    state::global_alloc_guard_active() && !state::global_alloc_allowance_active()
}

unsafe fn deny_when_guarded<F>(f: F) -> *mut u8
where
    F: FnOnce() -> *mut u8,
{
    if guard_active() {
        state::force_clear_global_alloc_guard();
        panic!("global allocation attempted while lp_pool guard is active");
    }
    f()
}
