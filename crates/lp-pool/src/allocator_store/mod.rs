mod guards;
mod storage;

#[cfg(feature = "alloc-meta")]
mod meta;

pub(crate) use guards::{
    enter_global_alloc_allowance, enter_global_alloc_guard, force_clear_global_alloc_guard,
    global_alloc_allowance_active, global_alloc_guard_active, GlobalAllocGuardToken,
};
pub(crate) use storage::{allocator_exists, set_allocator, with_allocator, with_allocator_mut};

#[cfg(feature = "alloc-meta")]
pub(crate) use meta::{clear_meta, with_meta, with_meta_mut};
