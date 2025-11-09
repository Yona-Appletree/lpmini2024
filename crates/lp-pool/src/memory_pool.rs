use core::ptr::NonNull;

use crate::allocator_store;
use crate::error::AllocError;
#[cfg(any(feature = "std", test))]
use crate::guarded_alloc::ScopedGlobalAllocGuard;
use crate::pool::LpAllocator;

/// Main memory pool interface with thread-local allocator
pub struct LpMemoryPool;

impl LpMemoryPool {
    /// Create a new memory pool
    ///
    /// # Safety
    /// - `memory` must point to a valid memory region of at least `size` bytes
    /// - Memory must remain valid for the lifetime of the pool
    pub unsafe fn new(memory: NonNull<u8>, size: usize) -> Result<Self, AllocError> {
        let root_pool = LpAllocator::new(memory, size)?;
        allocator_store::set_allocator(root_pool);
        Ok(LpMemoryPool)
    }

    /// Get access to the global default pool (only available with `default_pool` feature)
    ///
    /// This pool is automatically initialized on first use with default settings.
    #[cfg(feature = "default_pool")]
    pub fn global() -> Self {
        LpMemoryPool
    }

    /// Execute a closure while temporarily allowing global allocations
    ///
    /// This allows code inside `run()` to use the standard allocator (e.g., `Vec`, `String`)
    /// without panicking. The allowance is only active for the duration of the closure.
    #[cfg(any(feature = "std", test))]
    pub fn with_global_alloc<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _allow = allocator_store::enter_global_alloc_allowance();
        f()
    }

    /// Execute a closure with the pool active
    pub fn run<F, R, E>(&self, f: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
        E: From<AllocError>,
    {
        if !allocator_store::allocator_exists() {
            return Err(E::from(AllocError::PoolExhausted));
        }
        #[cfg(any(feature = "std", test))]
        let _guard = ScopedGlobalAllocGuard::enter();
        // Execute closure - it will access pool via with_active_pool()
        f()
    }

    /// Get current memory usage statistics
    pub fn stats(&self) -> Result<PoolStats, AllocError> {
        allocator_store::with_allocator(|pool| {
            Ok(PoolStats {
                used_bytes: pool.used_bytes(),
                capacity: pool.capacity(),
                used_blocks: pool.used_blocks(),
                free_blocks: pool.free_blocks(),
            })
        })
    }

    /// Get current used bytes
    pub fn used_bytes(&self) -> Result<usize, AllocError> {
        self.stats().map(|stats| stats.used_bytes)
    }

    /// Get total capacity in bytes
    pub fn capacity(&self) -> Result<usize, AllocError> {
        self.stats().map(|stats| stats.capacity)
    }

    /// Get available bytes (capacity - used)
    pub fn available_bytes(&self) -> Result<usize, AllocError> {
        let stats = self.stats()?;
        Ok(stats.capacity - stats.used_bytes)
    }

    /// Get usage percentage (0.0 to 1.0)
    pub fn usage_ratio(&self) -> Result<f32, AllocError> {
        let stats = self.stats()?;
        if stats.capacity == 0 {
            return Ok(0.0);
        }
        Ok(stats.used_bytes as f32 / stats.capacity as f32)
    }
}

/// Statistics about pool usage
#[derive(Debug, Clone, Copy)]
pub struct PoolStats {
    pub used_bytes: usize,
    pub capacity: usize,
    pub used_blocks: usize,
    pub free_blocks: usize,
}

/// Execute a closure with access to the active pool
pub(crate) fn with_active_pool<F, R>(f: F) -> Result<R, AllocError>
where
    F: FnOnce(&mut LpAllocator) -> Result<R, AllocError>,
{
    allocator_store::with_allocator_mut(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool_creation() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let pool = LpMemoryPool::new(memory_ptr, 1024).unwrap();
            let stats = pool.stats().unwrap();
            assert_eq!(stats.capacity, 1024);
            assert_eq!(stats.used_bytes, 0);
            assert_eq!(pool.available_bytes().unwrap(), 1024);
            assert_eq!(pool.usage_ratio().unwrap(), 0.0);
        }
    }

    #[test]
    fn test_run() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let pool = LpMemoryPool::new(memory_ptr, 1024).unwrap();
            let result = pool.run(|| Ok::<i32, AllocError>(42)).unwrap();
            assert_eq!(result, 42);
        }
    }

    #[test]
    fn test_usage_meta() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let pool = LpMemoryPool::new(memory_ptr, 1024).unwrap();

            // Initially empty
            assert_eq!(pool.used_bytes().unwrap(), 0);
            assert_eq!(pool.available_bytes().unwrap(), 1024);

            // Allocate something and keep it alive
            use crate::collections::LpBox;
            let _boxed = pool
                .run(|| LpBox::try_new_with_scope(42i32, Some("test_scope")))
                .unwrap();

            // Should have used some memory (boxed is still alive)
            let used = pool.used_bytes().unwrap();
            assert!(used > 0);
            assert_eq!(pool.available_bytes().unwrap(), 1024 - used);

            // Drop it
            drop(_boxed);

            // Memory should be freed
            assert_eq!(pool.used_bytes().unwrap(), 0);
        }
    }

    #[test]
    fn test_global_allocations_panic_inside_run() {
        use std::panic;

        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let pool = LpMemoryPool::new(memory_ptr, 1024).unwrap();

            let panic_result = panic::catch_unwind(|| {
                let _ = pool.run(|| {
                    let mut vec = alloc::vec::Vec::new();
                    vec.push(1);
                    Ok::<(), AllocError>(())
                });
            });

            assert!(
                panic_result.is_err(),
                "global allocation should panic while guard active"
            );

            // Guard must be released even after panic so we can run again
            let rerun = pool.run(|| Ok::<(), AllocError>(()));
            assert!(rerun.is_ok(), "pool.run should succeed after guard panic");
        }
    }

    #[test]
    fn test_nested_run_guard_depth() {
        let mut memory = [0u8; 2048];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let pool = LpMemoryPool::new(memory_ptr, 2048).unwrap();

            let result = pool.run(|| {
                pool.run(|| {
                    let mut vec = crate::LpVec::new();
                    vec.try_push(1)?;
                    vec.try_push(2)?;
                    Ok::<(), AllocError>(())
                })?;
                Ok::<(), AllocError>(())
            });

            assert!(result.is_ok());
        }
    }
}

#[cfg(all(test, feature = "default_pool"))]
mod default_pool_tests {
    use super::*;
    use alloc::vec::Vec;
    use core::ptr::NonNull;
    use std::panic;

    #[test]
    fn default_pool_runs_without_manual_initialization() {
        let pool = LpMemoryPool::global();
        let result = pool.run(|| Ok::<(), AllocError>(()));
        assert!(result.is_ok(), "default pool should allow running closures");
    }

    #[test]
    fn global_allocations_require_with_global_alloc() {
        let pool = LpMemoryPool::global();

        let panic_result = panic::catch_unwind(|| {
            let _ = pool.run(|| {
                let mut host_vec = Vec::new();
                host_vec.push(42u8);
                Ok::<(), AllocError>(())
            });
        });

        assert!(
            panic_result.is_err(),
            "host allocations without with_global_alloc should panic"
        );

        let ok_result = pool.run(|| {
            LpMemoryPool::with_global_alloc(|| {
                let mut host_vec = Vec::new();
                host_vec.push(7u8);
            });
            Ok::<(), AllocError>(())
        });

        assert!(
            ok_result.is_ok(),
            "with_global_alloc should permit host allocations while pool is active"
        );
    }

    #[test]
    fn with_global_alloc_allows_println() {
        #[cfg(any(feature = "std", test))]
        {
            let mut memory = [0u8; 1024];
            let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

            unsafe {
                let pool = LpMemoryPool::new(memory_ptr, 1024).unwrap();

                let result = pool.run(|| {
                    LpMemoryPool::with_global_alloc(|| {
                        // println! allocates internally, this should work
                        std::println!("Test message");
                    });
                    Ok::<(), AllocError>(())
                });

                assert!(
                    result.is_ok(),
                    "with_global_alloc should allow println! allocations"
                );
            }
        }
    }

    #[test]
    fn with_global_alloc_allows_string_operations() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let pool = LpMemoryPool::new(memory_ptr, 1024).unwrap();

            let result = pool.run(|| {
                LpMemoryPool::with_global_alloc(|| {
                    // String::repeat allocates, this should work
                    let indent = "  ".repeat(5);
                    assert_eq!(indent.len(), 10);
                });
                Ok::<(), AllocError>(())
            });

            assert!(
                result.is_ok(),
                "with_global_alloc should allow String operations"
            );
        }
    }

    #[test]
    fn with_global_alloc_allows_nested_allocations() {
        // Test that allocations work in nested function calls
        fn helper_function(indent: usize) {
            let indent_str = "  ".repeat(indent);
            std::println!("{}Nested allocation test", indent_str);
        }

        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let pool = LpMemoryPool::new(memory_ptr, 1024).unwrap();

            let result = pool.run(|| {
                LpMemoryPool::with_global_alloc(|| {
                    // Call a function that allocates internally
                    helper_function(3);
                });
                Ok::<(), AllocError>(())
            });

            assert!(
                result.is_ok(),
                "with_global_alloc should allow allocations in nested functions"
            );
        }
    }

    #[test]
    fn with_global_alloc_allows_immediate_println() {
        // Test that println! works as the first call in with_global_alloc
        // This reproduces the lp-data test scenario
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let pool = LpMemoryPool::new(memory_ptr, 1024).unwrap();

            let result = pool.run(|| {
                LpMemoryPool::with_global_alloc(|| {
                    // First allocation - println! should work
                    std::println!("First message");
                    // Then string operations
                    let indent = "  ".repeat(2);
                    std::println!("{}Second message", indent);
                });
                Ok::<(), AllocError>(())
            });

            assert!(
                result.is_ok(),
                "with_global_alloc should allow println! as first operation"
            );
        }
    }
}
