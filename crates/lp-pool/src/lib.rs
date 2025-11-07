#![no_std]
//! # lp-pool
//! 
//! A memory pool allocator for embedded and `no_std` environments with support for `allocator-api2`.
//! 
//! ## Features
//! 
//! - **Fixed-size block allocator**: Efficient pool allocator with free list management
//! - **Thread-local access**: Ergonomic thread-local pool access via `LpMemoryPool::run()`
//! - **Grow/shrink support**: Dynamic resizing of allocations
//! - **allocator-api2 compatible**: Implements `Allocator` trait for use with standard collections
//! - **Pool-backed collections**: Custom `PoolVec`, `PoolString`, `PoolBTreeMap`, and `PoolBox`
//! - **Allocation metadata tracking**: Optional tracking of allocation types and scopes (via `alloc-meta` feature)
//! 
//! ## Example
//! 
//! ```rust,no_run
//! use lp_pool::{LpMemoryPool, PoolVec};
//! use core::ptr::NonNull;
//! 
//! // Allocate a memory region
//! let mut memory = [0u8; 4096];
//! let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
//! 
//! // Create pool
//! let pool = unsafe { LpMemoryPool::new(memory_ptr, 4096, 64).unwrap() };
//! 
//! // Use collections within the pool
//! pool.run(|| {
//!     let mut vec = PoolVec::new();
//!     vec.try_push(1)?;
//!     vec.try_push(2)?;
//!     assert_eq!(vec.len(), 2);
//!     Ok::<(), lp_pool::AllocError>(())
//! }).unwrap();
//! ```
//! 
//! ## Limitations
//! 
//! - **Fixed-size blocks**: All allocations must fit within the configured `block_size`
//! - **No alignment guarantees**: Blocks are aligned to `block_size` boundaries, which may not match all alignment requirements
//! - **BTreeMap implementation**: Uses a simplified binary search tree (not a true B-tree), so performance may degrade with unbalanced data

extern crate alloc;

pub mod error;
pub mod pool;
pub mod memory_pool;
pub mod allocator;
pub mod collections;

pub use error::AllocError;
pub use memory_pool::{LpMemoryPool, PoolStats};
pub use allocator::PoolAllocatorWrapper;
pub use collections::{PoolVec, PoolString, PoolBTreeMap, PoolBox, print_memory_stats, print_memory_stats_with};

