#![no_std]
//! # lp-pool
//!
//! A memory pool allocator for embedded and `no_std` environments with support for `allocator-api2`.
//!
//! ## Features
//!
//! - **Variable-size block allocator**: Efficient pool allocator with free list management and block coalescing
//! - **Thread-local access**: Ergonomic thread-local pool access via `LpMemoryPool::run()`
//! - **Grow/shrink support**: Dynamic resizing of allocations
//! - **allocator-api2 compatible**: Implements `Allocator` trait for use with standard collections
//! - **Pool-backed collections**: Custom `LpVec`, `LpString`, `LpBTreeMap`, and `LpBox`
//! - **Allocation types tracking**: Optional tracking of allocation types and scopes (via `alloc-meta` feature)
//!
//! ## Example
//!
//! ```rust,no_run
//! use lp_pool::{LpMemoryPool, LpVec};
//! use core::ptr::NonNull;
//!
//! // Allocate a memory region
//! let mut memory = [0u8; 4096];
//! let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
//!
//! // Create pool
//! let pool = unsafe { LpMemoryPool::new(memory_ptr, 4096).unwrap() };
//!
//! // Use collections within the pool
//! pool.run(|| {
//!     let mut vec = LpVec::new();
//!     vec.try_push(1)?;
//!     vec.try_push(2)?;
//!     assert_eq!(vec.len(), 2);
//!     Ok::<(), lp_pool::AllocError>(())
//! }).unwrap();
//! ```
//!
//! ## Limitations
//!
//! - **BTreeMap implementation**: Uses a simplified binary search tree (not a true B-tree), so performance may degrade with unbalanced data
//! - **Coalescing overhead**: Block coalescing requires scanning memory, which has O(n) complexity for finding previous blocks

extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std;

pub mod allocator;
mod allocator_store;
pub mod block_header;
pub mod collections;
pub mod error;
pub mod fmt;
#[cfg(any(feature = "std", test))]
mod guarded_alloc;
pub mod memory_pool;
mod pool;

pub use allocator::LpAllocatorWrapper;
pub use collections::{
    print_memory_stats, print_memory_stats_with, LpBTreeMap, LpBox, LpBoxDyn, LpString, LpVec,
    LpVecIter, LpVecIterMut,
};
pub use error::AllocError;
pub use fmt::{lp_format, write_lp_string};
#[cfg(any(feature = "std", test))]
pub use guarded_alloc::{allow_global_alloc, ScopedGlobalAllocGuard};
pub use memory_pool::{LpMemoryPool, PoolStats};

pub use allocator_store::enter_global_alloc_allowance;

#[cfg(any(feature = "std", test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: guarded_alloc::GuardedAllocator = guarded_alloc::GuardedAllocator;

#[cfg(test)]
mod tests;
