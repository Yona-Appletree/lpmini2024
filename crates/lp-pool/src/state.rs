#[cfg(feature = "alloc-meta")]
use crate::collections::alloc_meta::AllocationMetaMap;
use crate::error::AllocError;
use crate::pool::LpAllocator;
#[cfg(any(feature = "std", test))]
use core::cell::Cell;

#[cfg(any(feature = "std", test))]
mod allocator_storage {
    use core::cell::RefCell;
    use std::thread_local;

    use super::*;

    thread_local! {
        static ALLOCATOR: RefCell<Option<LpAllocator>> = const { RefCell::new(None) };
    }

    pub(super) fn set(pool: LpAllocator) {
        ALLOCATOR.with(|cell| {
            let mut borrow = cell.borrow_mut();
            borrow.replace(pool);
        });
    }

    pub(super) fn exists() -> bool {
        ALLOCATOR.with(|cell| cell.borrow().is_some())
    }

    pub(super) fn with_ref<F, R>(f: F) -> Result<R, AllocError>
    where
        F: FnOnce(&LpAllocator) -> Result<R, AllocError>,
    {
        ALLOCATOR.with(|cell| {
            let borrow = cell.borrow();
            let pool = borrow.as_ref().ok_or(AllocError::PoolExhausted)?;
            f(pool)
        })
    }

    pub(super) fn with_mut<F, R>(f: F) -> Result<R, AllocError>
    where
        F: FnOnce(&mut LpAllocator) -> Result<R, AllocError>,
    {
        ALLOCATOR.with(|cell| {
            let mut borrow = cell.borrow_mut();
            let pool = borrow.as_mut().ok_or(AllocError::PoolExhausted)?;
            f(pool)
        })
    }
}

#[cfg(all(not(feature = "std"), not(test)))]
mod allocator_storage {
    use spin::Mutex;

    use super::*;

    static ALLOCATOR: Mutex<Option<LpAllocator>> = Mutex::new(None);

    pub(super) fn set(pool: LpAllocator) {
        let mut guard = ALLOCATOR.lock();
        guard.replace(pool);
    }

    pub(super) fn exists() -> bool {
        ALLOCATOR.lock().is_some()
    }

    pub(super) fn with_ref<F, R>(f: F) -> Result<R, AllocError>
    where
        F: FnOnce(&LpAllocator) -> Result<R, AllocError>,
    {
        let guard = ALLOCATOR.lock();
        let pool = guard.as_ref().ok_or(AllocError::PoolExhausted)?;
        f(pool)
    }

    pub(super) fn with_mut<F, R>(f: F) -> Result<R, AllocError>
    where
        F: FnOnce(&mut LpAllocator) -> Result<R, AllocError>,
    {
        let mut guard = ALLOCATOR.lock();
        let pool = guard.as_mut().ok_or(AllocError::PoolExhausted)?;
        f(pool)
    }
}

#[cfg(feature = "alloc-meta")]
#[cfg(any(feature = "std", test))]
mod meta_storage {
    use core::cell::RefCell;
    use std::thread_local;

    use super::*;

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
    use spin::Mutex;

    use super::*;

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

pub(crate) fn set_allocator(allocator: LpAllocator) {
    allocator_storage::set(allocator);
    #[cfg(feature = "alloc-meta")]
    {
        meta_storage::clear();
    }
}

pub(crate) fn allocator_exists() -> bool {
    allocator_storage::exists()
}

pub(crate) fn with_allocator<F, R>(f: F) -> Result<R, AllocError>
where
    F: FnOnce(&LpAllocator) -> Result<R, AllocError>,
{
    allocator_storage::with_ref(f)
}

pub(crate) fn with_allocator_mut<F, R>(f: F) -> Result<R, AllocError>
where
    F: FnOnce(&mut LpAllocator) -> Result<R, AllocError>,
{
    allocator_storage::with_mut(f)
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

#[cfg(any(feature = "std", test))]
mod guard_storage {
    use super::*;
    use std::thread_local;

    thread_local! {
        static DEPTH: Cell<u32> = const { Cell::new(0) };
        static ALLOW_DEPTH: Cell<u32> = const { Cell::new(0) };
    }

    pub(super) fn push() {
        DEPTH.with(|depth| depth.set(depth.get().saturating_add(1)));
    }

    pub(super) fn pop() {
        DEPTH.with(|depth| {
            let current = depth.get();
            if current > 0 {
                depth.set(current - 1);
            }
        });
    }

    pub(super) fn active() -> bool {
        DEPTH.with(|depth| depth.get() > 0)
    }

    pub(super) fn force_clear() {
        DEPTH.with(|depth| depth.set(0));
        ALLOW_DEPTH.with(|depth| depth.set(0));
    }

    pub(super) fn push_allow() {
        ALLOW_DEPTH.with(|depth| depth.set(depth.get().saturating_add(1)));
    }

    pub(super) fn pop_allow() {
        ALLOW_DEPTH.with(|depth| {
            let current = depth.get();
            if current > 0 {
                depth.set(current - 1);
            }
        });
    }

    pub(super) fn allow_active() -> bool {
        ALLOW_DEPTH.with(|depth| depth.get() > 0)
    }
}

#[cfg(any(feature = "std", test))]
pub(crate) struct GlobalAllocGuardToken;

#[cfg(any(feature = "std", test))]
pub(crate) fn enter_global_alloc_guard() -> GlobalAllocGuardToken {
    guard_storage::push();
    GlobalAllocGuardToken
}

#[cfg(any(feature = "std", test))]
pub(crate) fn global_alloc_guard_active() -> bool {
    guard_storage::active()
}

#[cfg(any(feature = "std", test))]
pub(crate) fn global_alloc_allowance_active() -> bool {
    guard_storage::allow_active()
}

#[cfg(any(feature = "std", test))]
pub(crate) struct GlobalAllocAllowToken;

#[cfg(any(feature = "std", test))]
pub(crate) fn enter_global_alloc_allowance() -> GlobalAllocAllowToken {
    guard_storage::push_allow();
    GlobalAllocAllowToken
}

#[cfg(any(feature = "std", test))]
impl Drop for GlobalAllocAllowToken {
    fn drop(&mut self) {
        guard_storage::pop_allow();
    }
}

#[cfg(any(feature = "std", test))]
pub(crate) fn force_clear_global_alloc_guard() {
    guard_storage::force_clear();
}

#[cfg(any(feature = "std", test))]
impl Drop for GlobalAllocGuardToken {
    fn drop(&mut self) {
        guard_storage::pop();
    }
}

#[cfg(all(not(feature = "std"), not(test)))]
pub(crate) struct GlobalAllocGuardToken;

#[cfg(all(not(feature = "std"), not(test)))]
#[allow(clippy::unused_unit)]
pub(crate) fn enter_global_alloc_guard() -> GlobalAllocGuardToken {
    GlobalAllocGuardToken
}

#[cfg(all(not(feature = "std"), not(test)))]
pub(crate) fn global_alloc_guard_active() -> bool {
    false
}

#[cfg(all(not(feature = "std"), not(test)))]
pub(crate) fn global_alloc_allowance_active() -> bool {
    false
}

#[cfg(all(not(feature = "std"), not(test)))]
pub(crate) struct GlobalAllocAllowToken;

#[cfg(all(not(feature = "std"), not(test)))]
pub(crate) fn enter_global_alloc_allowance() -> GlobalAllocAllowToken {
    GlobalAllocAllowToken
}

#[cfg(all(not(feature = "std"), not(test)))]
impl Drop for GlobalAllocAllowToken {
    fn drop(&mut self) {}
}

#[cfg(all(not(feature = "std"), not(test)))]
pub(crate) fn force_clear_global_alloc_guard() {}
