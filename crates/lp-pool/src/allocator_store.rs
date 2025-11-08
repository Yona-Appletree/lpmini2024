use crate::error::AllocError;
use crate::pool::LpAllocator;

#[cfg(feature = "alloc-meta")]
use crate::collections::alloc_meta::AllocationMetaMap;

#[cfg(any(feature = "std", test))]
use core::cell::{Cell, RefCell};
#[cfg(any(feature = "std", test))]
use std::thread_local;

#[cfg(all(not(feature = "std"), not(test)))]
use core::sync::atomic::{AtomicU32, Ordering};
#[cfg(all(not(feature = "std"), not(test)))]
use spin::Mutex;

#[cfg(any(feature = "std", test))]
thread_local! {
    static ALLOCATOR: RefCell<Option<LpAllocator>> = const { RefCell::new(None) };
    static GUARD_DEPTH: Cell<u32> = const { Cell::new(0) };
    static ALLOW_DEPTH: Cell<u32> = const { Cell::new(0) };
}

#[cfg(all(not(feature = "std"), not(test)))]
static ALLOCATOR: Mutex<Option<LpAllocator>> = Mutex::new(None);
#[cfg(all(not(feature = "std"), not(test)))]
static GUARD_DEPTH: AtomicU32 = AtomicU32::new(0);
#[cfg(all(not(feature = "std"), not(test)))]
static ALLOW_DEPTH: AtomicU32 = AtomicU32::new(0);

pub(crate) fn set_allocator(allocator: LpAllocator) {
    #[cfg(any(feature = "std", test))]
    {
        ALLOCATOR.with(|cell| {
            *cell.borrow_mut() = Some(allocator);
        });
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        *ALLOCATOR.lock() = Some(allocator);
    }
    #[cfg(feature = "alloc-meta")]
    {
        clear_meta();
    }
}

pub(crate) fn allocator_exists() -> bool {
    #[cfg(any(feature = "std", test))]
    {
        ALLOCATOR.with(|cell| {
            if cell.borrow().is_some() {
                return true;
            }
            #[cfg(feature = "default_pool")]
            {
                ensure_default_pool().is_ok()
            }
            #[cfg(not(feature = "default_pool"))]
            {
                false
            }
        })
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        ALLOCATOR.lock().is_some()
    }
}

#[cfg(all(feature = "default_pool", any(feature = "std", test)))]
fn ensure_default_pool() -> Result<(), AllocError> {
    use alloc::boxed::Box;
    use alloc::vec;
    use core::ptr::NonNull;

    ALLOCATOR.with(|cell| {
        if cell.borrow().is_some() {
            return Ok(());
        }

        const DEFAULT_POOL_SIZE: usize = 512 * 1024;
        let mut boxed = vec![0u8; DEFAULT_POOL_SIZE].into_boxed_slice();

        match NonNull::new(boxed.as_mut_ptr()) {
            Some(ptr) => {
                match unsafe { LpAllocator::new(ptr, DEFAULT_POOL_SIZE) } {
                    Ok(allocator) => {
                        // Leak the boxed slice so the memory stays valid
                        Box::leak(boxed);
                        *cell.borrow_mut() = Some(allocator);
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
            None => Err(AllocError::PoolExhausted),
        }
    })
}

pub(crate) fn with_allocator<F, R>(f: F) -> Result<R, AllocError>
where
    F: FnOnce(&LpAllocator) -> Result<R, AllocError>,
{
    #[cfg(any(feature = "std", test))]
    {
        ALLOCATOR.with(|cell| {
            if cell.borrow().is_none() {
                #[cfg(feature = "default_pool")]
                {
                    if let Err(err) = ensure_default_pool() {
                        return Err(err);
                    }
                }
                #[cfg(not(feature = "default_pool"))]
                {
                    return Err(AllocError::PoolExhausted);
                }
            }
            let borrow = cell.borrow();
            let allocator = borrow.as_ref().ok_or(AllocError::PoolExhausted)?;
            f(allocator)
        })
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        let guard = ALLOCATOR.lock();
        let allocator = guard.as_ref().ok_or(AllocError::PoolExhausted)?;
        f(allocator)
    }
}

pub(crate) fn with_allocator_mut<F, R>(f: F) -> Result<R, AllocError>
where
    F: FnOnce(&mut LpAllocator) -> Result<R, AllocError>,
{
    #[cfg(any(feature = "std", test))]
    {
        ALLOCATOR.with(|cell| {
            if cell.borrow().is_none() {
                #[cfg(feature = "default_pool")]
                {
                    if let Err(err) = ensure_default_pool() {
                        return Err(err);
                    }
                }
                #[cfg(not(feature = "default_pool"))]
                {
                    return Err(AllocError::PoolExhausted);
                }
            }
            let mut borrow = cell.borrow_mut();
            let allocator = borrow.as_mut().ok_or(AllocError::PoolExhausted)?;
            f(allocator)
        })
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        let mut guard = ALLOCATOR.lock();
        let allocator = guard.as_mut().ok_or(AllocError::PoolExhausted)?;
        f(allocator)
    }
}

#[cfg(any(feature = "std", test))]
fn guard_depth() -> u32 {
    GUARD_DEPTH.with(|depth| depth.get())
}
#[cfg(all(not(feature = "std"), not(test)))]
fn guard_depth() -> u32 {
    GUARD_DEPTH.load(Ordering::SeqCst)
}

#[cfg(any(feature = "std", test))]
fn allow_depth() -> u32 {
    ALLOW_DEPTH.with(|depth| depth.get())
}
#[cfg(all(not(feature = "std"), not(test)))]
fn allow_depth() -> u32 {
    ALLOW_DEPTH.load(Ordering::SeqCst)
}

pub(crate) fn global_alloc_guard_active() -> bool {
    guard_depth() > 0
}

pub(crate) fn global_alloc_allowance_active() -> bool {
    allow_depth() > 0
}

pub(crate) struct GlobalAllocGuardToken;

pub(crate) fn enter_global_alloc_guard() -> GlobalAllocGuardToken {
    #[cfg(any(feature = "std", test))]
    {
        GUARD_DEPTH.with(|depth| depth.set(depth.get().saturating_add(1)));
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        GUARD_DEPTH.fetch_add(1, Ordering::SeqCst);
    }
    GlobalAllocGuardToken
}

impl Drop for GlobalAllocGuardToken {
    fn drop(&mut self) {
        #[cfg(any(feature = "std", test))]
        {
            GUARD_DEPTH.with(|depth| {
                let current = depth.get();
                if current > 0 {
                    depth.set(current - 1);
                }
            });
        }
        #[cfg(all(not(feature = "std"), not(test)))]
        {
            decrement(&GUARD_DEPTH);
        }
    }
}

pub(crate) struct GlobalAllocAllowToken;

pub(crate) fn enter_global_alloc_allowance() -> GlobalAllocAllowToken {
    #[cfg(any(feature = "std", test))]
    {
        ALLOW_DEPTH.with(|depth| depth.set(depth.get().saturating_add(1)));
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        ALLOW_DEPTH.fetch_add(1, Ordering::SeqCst);
    }
    GlobalAllocAllowToken
}

impl Drop for GlobalAllocAllowToken {
    fn drop(&mut self) {
        #[cfg(any(feature = "std", test))]
        {
            ALLOW_DEPTH.with(|depth| {
                let current = depth.get();
                if current > 0 {
                    depth.set(current - 1);
                }
            });
        }
        #[cfg(all(not(feature = "std"), not(test)))]
        {
            decrement(&ALLOW_DEPTH);
        }
    }
}

#[cfg(all(not(feature = "std"), not(test)))]
fn decrement(counter: &AtomicU32) {
    loop {
        let current = counter.load(Ordering::SeqCst);
        if current == 0 {
            break;
        }
        if counter
            .compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            break;
        }
    }
}

pub(crate) fn force_clear_global_alloc_guard() {
    #[cfg(any(feature = "std", test))]
    {
        GUARD_DEPTH.with(|depth| depth.set(0));
        ALLOW_DEPTH.with(|depth| depth.set(0));
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        GUARD_DEPTH.store(0, Ordering::SeqCst);
        ALLOW_DEPTH.store(0, Ordering::SeqCst);
    }
}

#[cfg(feature = "alloc-meta")]
pub(crate) fn with_meta<F, R>(f: F) -> R
where
    F: FnOnce(&AllocationMetaMap) -> R,
{
    meta_storage::with_ref(f)
}

#[cfg(feature = "alloc-meta")]
pub(crate) fn with_meta_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut AllocationMetaMap) -> R,
{
    meta_storage::with_mut(f)
}

#[cfg(feature = "alloc-meta")]
pub(crate) fn clear_meta() {
    meta_storage::clear();
}

#[cfg(feature = "alloc-meta")]
#[cfg(any(feature = "std", test))]
mod meta_storage {
    use super::*;
    use core::cell::RefCell;
    use std::thread_local;

    thread_local! {
        static META: RefCell<AllocationMetaMap> =
            const { RefCell::new(AllocationMetaMap::new()) };
    }

    pub(super) fn clear() {
        let _allow = super::enter_global_alloc_allowance();
        META.with(|cell| cell.borrow_mut().clear());
    }

    pub(super) fn with_ref<F, R>(f: F) -> R
    where
        F: FnOnce(&AllocationMetaMap) -> R,
    {
        let _allow = super::enter_global_alloc_allowance();
        META.with(|cell| f(&cell.borrow()))
    }

    pub(super) fn with_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut AllocationMetaMap) -> R,
    {
        let _allow = super::enter_global_alloc_allowance();
        META.with(|cell| f(&mut cell.borrow_mut()))
    }
}

#[cfg(feature = "alloc-meta")]
#[cfg(all(not(feature = "std"), not(test)))]
mod meta_storage {
    use super::*;
    use spin::Mutex;

    static META: Mutex<AllocationMetaMap> = Mutex::new(AllocationMetaMap::new());

    pub(super) fn clear() {
        let _allow = super::enter_global_alloc_allowance();
        let mut guard = META.lock();
        guard.clear();
    }

    pub(super) fn with_ref<F, R>(f: F) -> R
    where
        F: FnOnce(&AllocationMetaMap) -> R,
    {
        let _allow = super::enter_global_alloc_allowance();
        let guard = META.lock();
        f(&*guard)
    }

    pub(super) fn with_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut AllocationMetaMap) -> R,
    {
        let _allow = super::enter_global_alloc_allowance();
        let mut guard = META.lock();
        f(&mut *guard)
    }
}
