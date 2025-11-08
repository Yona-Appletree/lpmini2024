#[cfg(feature = "alloc-meta")]
use crate::collections::alloc_meta::AllocationMetaMap;
use crate::error::AllocError;
use crate::pool::LpAllocator;

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
        META.with(|cell| cell.borrow_mut().clear());
    }

    pub(super) fn with_ref<F, R>(f: F) -> R
    where
        F: FnOnce(&AllocationMetaMap) -> R,
    {
        META.with(|cell| f(&cell.borrow()))
    }

    pub(super) fn with_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut AllocationMetaMap) -> R,
    {
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
        let mut guard = META.lock();
        guard.clear();
    }

    pub(super) fn with_ref<F, R>(f: F) -> R
    where
        F: FnOnce(&AllocationMetaMap) -> R,
    {
        let guard = META.lock();
        f(&*guard)
    }

    pub(super) fn with_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut AllocationMetaMap) -> R,
    {
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
