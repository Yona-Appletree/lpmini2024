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

#[cfg(test)]
mod integration_tests {
    use super::*;
    use core::ptr::NonNull;
    
    #[test]
    fn test_all_collections_together() {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 16384, 128).unwrap() };
        
        pool.run(|| {
            // Test PoolVec
            let mut vec = PoolVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;
            assert_eq!(vec.len(), 3);
            
            // Test PoolString
            let mut s = PoolString::new();
            s.try_push_str("hello")?;
            s.try_push_str(" world")?;
            assert_eq!(s.as_str(), "hello world");
            
            // Test PoolBTreeMap
            let mut map = PoolBTreeMap::new();
            map.try_insert("key1", 100)?;
            map.try_insert("key2", 200)?;
            assert_eq!(map.get(&"key1"), Some(&100));
            assert_eq!(map.get(&"key2"), Some(&200));
            
            // Test PoolBox
            let boxed = PoolBox::try_new(42i32)?;
            assert_eq!(*boxed, 42);
            
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[test]
    fn test_allocator_wrapper_with_collections() {
        use allocator_api2::alloc::Allocator;
        
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 16384, 128).unwrap() };
        
        pool.run(|| {
            let allocator = PoolAllocatorWrapper;
            let layout = core::alloc::Layout::from_size_align(64, 8).unwrap();
            
            // Allocate using allocator-api2
            let ptr = Allocator::allocate(&allocator, layout).unwrap();
            assert_eq!(ptr.len(), 128); // block_size
            
            // Deallocate
            unsafe {
                Allocator::deallocate(&allocator, NonNull::new(ptr.as_ptr() as *mut u8).unwrap(), layout);
            }
            
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[test]
    fn test_memory_usage_tracking() {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 16384, 128).unwrap() };
        
        let before = pool.used_bytes().unwrap();
        assert_eq!(before, 0);
        
        pool.run(|| {
            let before_inner = pool.used_bytes().unwrap();
            
            let _vec = PoolVec::<i32>::new();
            let _boxed = PoolBox::try_new(42i32)?;
            
            let during = pool.used_bytes().unwrap();
            assert!(during > before_inner);
            
            // Collections are dropped here when closure returns
            Ok::<(), AllocError>(())
        }).unwrap();
        
        // After dropping, memory should be freed
        let after = pool.used_bytes().unwrap();
        assert_eq!(after, before);
    }
}
