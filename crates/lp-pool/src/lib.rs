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
//! - **Allocation metadata tracking**: Optional tracking of allocation types and scopes (via `alloc-meta` feature)
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

pub mod allocator;
pub mod block_header;
pub mod collections;
pub mod error;
pub mod memory_pool;
pub mod pool;

pub use allocator::LpAllocatorWrapper;
pub use collections::{
    print_memory_stats, print_memory_stats_with, LpBTreeMap, LpBox, LpString, LpVec, LpVecIter,
    LpVecIterMut,
};
pub use error::AllocError;
pub use memory_pool::{LpMemoryPool, PoolStats};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use core::ptr::NonNull;

    #[test]
    fn test_all_collections_together() {
        let mut memory = [0u8; 65536]; // Large pool for collections with 32-byte headers
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 65536).unwrap() };

        pool.run(|| {
            // Test LpVec
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;
            assert_eq!(vec.len(), 3);

            // Test LpString
            let mut s = LpString::new();
            s.try_push_str("hello")?;
            s.try_push_str(" world")?;
            assert_eq!(s.as_str(), "hello world");

            // Test LpBox
            let boxed = LpBox::try_new(42i32)?;
            assert_eq!(*boxed, 42);

            // Note: LpBTreeMap test skipped here due to high memory overhead
            // BTreeMap has its own comprehensive tests in collections::btree::map

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_allocator_wrapper_with_collections() {
        use allocator_api2::alloc::Allocator;

        let mut memory = [0u8; 32768]; // Increased for larger headers
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 32768).unwrap() };

        pool.run(|| {
            let allocator = LpAllocatorWrapper;
            let layout = core::alloc::Layout::from_size_align(64, 8).unwrap();

            // Allocate using allocator-api2
            let ptr = Allocator::allocate(&allocator, layout).unwrap();
            assert_eq!(ptr.len(), 64); // requested size

            // Deallocate
            unsafe {
                Allocator::deallocate(
                    &allocator,
                    NonNull::new(ptr.as_ptr() as *mut u8).unwrap(),
                    layout,
                );
            }

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_memory_usage_tracking() {
        let mut memory = [0u8; 32768]; // Increased for larger headers
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 32768).unwrap() };

        let before = pool.used_bytes().unwrap();
        assert_eq!(before, 0);

        pool.run(|| {
            let before_inner = pool.used_bytes().unwrap();

            let _vec = LpVec::<i32>::new();
            let _boxed = LpBox::try_new(42i32)?;

            let during = pool.used_bytes().unwrap();
            assert!(during > before_inner);

            // Collections are dropped here when closure returns
            Ok::<(), AllocError>(())
        })
        .unwrap();

        // After dropping, memory should be freed
        let after = pool.used_bytes().unwrap();
        assert_eq!(after, before);
    }

    // === Integration & Stress Tests ===

    #[test]
    fn test_stress_allocate_until_oom() {
        let mut memory = [0u8; 8192];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 8192).unwrap() };

        pool.run(|| {
            let mut boxes = alloc::vec::Vec::new();

            // Allocate until OOM
            loop {
                match LpBox::try_new([0u8; 64]) {
                    Ok(b) => boxes.push(b),
                    Err(AllocError::OutOfMemory { .. }) | Err(AllocError::PoolExhausted) => break,
                    Err(e) => panic!("Unexpected error: {:?}", e),
                }
            }

            // Should have allocated multiple boxes
            assert!(boxes.len() > 5, "Should allocate at least 5 boxes");

            // Free half of them
            boxes.truncate(boxes.len() / 2);

            // Should be able to allocate more
            let result = LpBox::try_new([0u8; 64]);
            assert!(result.is_ok(), "Should recover after freeing memory");

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_nested_collections_vec_of_strings() {
        let mut memory = [0u8; 32768];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 32768).unwrap() };

        pool.run(|| {
            let mut vec = LpVec::new();

            // Can't nest pool-allocated collections due to RefCell borrow
            // So we'll use standard Vec<String> inside LpBox
            let mut std_vec = alloc::vec::Vec::new();
            std_vec.push(alloc::string::String::from("hello"));
            std_vec.push(alloc::string::String::from("world"));

            let boxed = LpBox::try_new(std_vec)?;
            vec.try_push(boxed)?;

            assert_eq!(vec.len(), 1);
            assert_eq!(vec[0].len(), 2);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_stress_many_small_allocations() {
        let mut memory = [0u8; 32768];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 32768).unwrap() };

        pool.run(|| {
            let mut boxes = alloc::vec::Vec::new();

            // Allocate many small items
            for i in 0..100 {
                match LpBox::try_new(i) {
                    Ok(b) => boxes.push(b),
                    Err(AllocError::OutOfMemory { .. }) | Err(AllocError::PoolExhausted) => break,
                    Err(e) => panic!("Unexpected error: {:?}", e),
                }
            }

            // Should have allocated many
            assert!(boxes.len() > 50, "Should allocate at least 50 small boxes");

            // Verify values
            for (i, b) in boxes.iter().enumerate() {
                assert_eq!(**b, i);
            }

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_stress_vec_growth() {
        let mut memory = [0u8; 65536];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 65536).unwrap() };

        pool.run(|| {
            let mut vec = LpVec::new();

            // Push until we hit memory limit
            let mut count = 0;
            loop {
                match vec.try_push(count) {
                    Ok(_) => count += 1,
                    Err(AllocError::OutOfMemory { .. }) | Err(AllocError::PoolExhausted) => break,
                    Err(e) => panic!("Unexpected error: {:?}", e),
                }
            }

            // Should have pushed many items
            assert!(count > 100, "Should push at least 100 items, got {}", count);

            // Verify values
            for i in 0..count {
                assert_eq!(vec.get(i), Some(&i));
            }

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_stress_string_growth() {
        let mut memory = [0u8; 32768];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 32768).unwrap() };

        pool.run(|| {
            let mut s = LpString::new();

            // Build very long string
            let mut count = 0;
            loop {
                match s.try_push_str("0123456789") {
                    Ok(_) => count += 10,
                    Err(AllocError::OutOfMemory { .. }) | Err(AllocError::PoolExhausted) => break,
                    Err(e) => panic!("Unexpected error: {:?}", e),
                }
            }

            // Should have built a long string
            assert!(
                count > 1000,
                "Should build string of at least 1000 chars, got {}",
                count
            );
            // Verify actual string length (count might differ due to partial pushes)
            assert!(s.len() > 1000, "String should be at least 1000 bytes");
            assert!(s.len() <= count + 10, "String length should be reasonable");

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_multiple_vecs_simultaneously() {
        let mut memory = [0u8; 32768];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 32768).unwrap() };

        pool.run(|| {
            let mut vec1 = LpVec::new();
            let mut vec2 = LpVec::new();
            let mut vec3 = LpVec::new();

            // Push to multiple vecs
            for i in 0..10 {
                vec1.try_push(i)?;
                vec2.try_push(i * 2)?;
                vec3.try_push(i * 3)?;
            }

            assert_eq!(vec1.len(), 10);
            assert_eq!(vec2.len(), 10);
            assert_eq!(vec3.len(), 10);

            // Verify values
            for i in 0..10 {
                assert_eq!(vec1.get(i), Some(&i));
                assert_eq!(vec2.get(i), Some(&(i * 2)));
                assert_eq!(vec3.get(i), Some(&(i * 3)));
            }

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_pool_stats_accuracy() {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() };

        let initial = pool.stats().unwrap();
        assert_eq!(initial.used_bytes, 0);
        assert_eq!(initial.capacity, 16384);

        pool.run(|| {
            let _box1 = LpBox::try_new([0u8; 100])?;
            let during = pool.stats().unwrap();
            assert!(during.used_bytes > 0);
            assert!(during.used_bytes < during.capacity);

            let _box2 = LpBox::try_new([0u8; 100])?;
            let during2 = pool.stats().unwrap();
            assert!(during2.used_bytes > during.used_bytes);

            Ok::<(), AllocError>(())
        })
        .unwrap();

        let final_stats = pool.stats().unwrap();
        assert_eq!(final_stats.used_bytes, 0);
    }

    #[test]
    fn test_usage_ratio() {
        let mut memory = [0u8; 8192];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 8192).unwrap() };

        assert_eq!(pool.usage_ratio().unwrap(), 0.0);

        pool.run(|| {
            let mut boxes = alloc::vec::Vec::new();

            // Allocate until about 50% full
            while pool.usage_ratio().unwrap() < 0.5 {
                match LpBox::try_new([0u8; 64]) {
                    Ok(b) => boxes.push(b),
                    Err(AllocError::OutOfMemory { .. }) | Err(AllocError::PoolExhausted) => break,
                    Err(e) => panic!("Unexpected error: {:?}", e),
                }
            }

            let ratio = pool.usage_ratio().unwrap();
            assert!(
                ratio >= 0.4 && ratio <= 0.9,
                "Usage ratio should be in reasonable range, got {}",
                ratio
            );

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_realistic_usage_pattern() {
        // Simulate a realistic usage pattern: create data, process it, free it
        let mut memory = [0u8; 32768];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 32768).unwrap() };

        pool.run(|| {
            for _ in 0..10 {
                // Create temporary data structures
                let mut vec = LpVec::new();
                for i in 0..20 {
                    vec.try_push(i)?;
                }

                let mut s = LpString::new();
                s.try_push_str("Processing...")?;

                // Process (just verify)
                assert_eq!(vec.len(), 20);
                assert!(!s.is_empty());

                // vec and s dropped here, freeing memory for next iteration
            }

            // After 10 iterations, pool should be empty
            let used = pool.used_bytes().unwrap();
            assert_eq!(used, 0, "All temporary data should be freed");

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}

#[cfg(test)]
mod panic_safety_tests {
    use super::*;
    use core::ptr::NonNull;

    // Note: Most drop safety tests are in individual collection test modules
    // These tests focus on cross-collection drop order and panic safety

    #[test]
    fn test_drop_order_across_collections() {
        use core::sync::atomic::{AtomicUsize, Ordering};

        static CROSS_DROP_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

        struct CrossDropTracker(usize);

        impl Drop for CrossDropTracker {
            fn drop(&mut self) {
                let seq = CROSS_DROP_SEQUENCE.fetch_add(1, Ordering::SeqCst);
                // LIFO: last created should drop first
                assert_eq!(
                    seq, self.0,
                    "Drop order incorrect: expected {}, got {}",
                    self.0, seq
                );
            }
        }

        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() };

        CROSS_DROP_SEQUENCE.store(0, Ordering::SeqCst);

        pool.run(|| {
            let _box1 = LpBox::try_new(CrossDropTracker(2))?;
            let _box2 = LpBox::try_new(CrossDropTracker(1))?;
            let _box3 = LpBox::try_new(CrossDropTracker(0))?;

            Ok::<(), AllocError>(())
        })
        .unwrap();

        assert_eq!(CROSS_DROP_SEQUENCE.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_allocation_failure_leaves_consistent_state() {
        let mut memory = [0u8; 1024]; // Small pool to trigger OOM
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 1024).unwrap() };

        pool.run(|| {
            let mut vec = LpVec::new();

            // Push until OOM
            loop {
                match vec.try_push(0u8) {
                    Ok(_) => {}
                    Err(AllocError::OutOfMemory { .. }) | Err(AllocError::PoolExhausted) => break,
                    Err(e) => panic!("Unexpected error: {:?}", e),
                }
            }

            // Vec should still be valid and usable
            let len_before = vec.len();
            assert!(len_before > 0);

            // Can still access elements
            assert_eq!(vec.get(0), Some(&0u8));

            // Clear should work
            vec.clear();
            assert_eq!(vec.len(), 0);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_memory_cleanup_on_early_return() {
        let mut memory = [0u8; 8192];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 8192).unwrap() };

        let before = pool.used_bytes().unwrap();

        let result: Result<(), AllocError> = pool.run(|| {
            let _box1 = LpBox::try_new([0u8; 100])?;
            let _box2 = LpBox::try_new([0u8; 100])?;

            // Early return with error
            Err(AllocError::InvalidLayout)
        });

        assert!(result.is_err());

        // Memory should be cleaned up even though we returned early
        let after = pool.used_bytes().unwrap();
        assert_eq!(after, before, "Memory should be cleaned up on early return");
    }
}
