#[cfg(feature = "alloc-meta")]
use crate::collections::alloc_meta::AllocationMetaMap;

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
        let _allow = super::super::guards::enter_global_alloc_allowance();
        META.with(|cell| cell.borrow_mut().clear());
    }

    pub(super) fn with_ref<F, R>(f: F) -> R
    where
        F: FnOnce(&AllocationMetaMap) -> R,
    {
        let _allow = super::super::guards::enter_global_alloc_allowance();
        META.with(|cell| f(&cell.borrow()))
    }

    pub(super) fn with_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut AllocationMetaMap) -> R,
    {
        let _allow = super::super::guards::enter_global_alloc_allowance();
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
        let _allow = super::super::guards::enter_global_alloc_allowance();
        let mut guard = META.lock();
        guard.clear();
    }

    pub(super) fn with_ref<F, R>(f: F) -> R
    where
        F: FnOnce(&AllocationMetaMap) -> R,
    {
        let _allow = super::super::guards::enter_global_alloc_allowance();
        let guard = META.lock();
        f(&*guard)
    }

    pub(super) fn with_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut AllocationMetaMap) -> R,
    {
        let _allow = super::super::guards::enter_global_alloc_allowance();
        let mut guard = META.lock();
        f(&mut *guard)
    }
}
