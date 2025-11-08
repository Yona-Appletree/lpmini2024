use core::alloc::Layout;
use core::ptr::NonNull;

#[cfg(feature = "alloc-meta")]
use super::alloc_meta::{record_allocation_meta, remove_allocation_meta, AllocationMeta};
use crate::error::AllocError;
use crate::memory_pool::with_active_pool;

/// Pool-backed Box
pub struct LpBox<T> {
    ptr: NonNull<T>,
    #[cfg(feature = "alloc-meta")]
    meta: AllocationMeta,
}

impl<T> LpBox<T> {
    /// Create a new LpBox with optional scope for metadata tracking
    pub fn try_new(value: T) -> Result<Self, AllocError> {
        Self::try_new_with_scope(value, None)
    }

    /// Create a new LpBox with a scope identifier for metadata tracking
    pub fn try_new_with_scope(value: T, scope: Option<&'static str>) -> Result<Self, AllocError> {
        let layout = Layout::new::<T>();

        #[cfg(feature = "alloc-meta")]
        let meta = AllocationMeta {
            type_name: core::any::type_name::<T>(),
            scope,
        };
        #[cfg(not(feature = "alloc-meta"))]
        let _ = scope; // Suppress unused warning

        let ptr = with_active_pool(|pool| {
            let allocated = pool.allocate(layout)?;
            let ptr = NonNull::new(allocated.as_ptr() as *mut T).unwrap();

            // Write value to allocated memory
            unsafe {
                core::ptr::write(ptr.as_ptr(), value);
            }

            #[cfg(feature = "alloc-meta")]
            {
                record_allocation_meta(meta, layout.size());
            }

            Ok(ptr)
        })?;

        Ok(LpBox {
            ptr,
            #[cfg(feature = "alloc-meta")]
            meta,
        })
    }

    #[allow(clippy::should_implement_trait)]
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.ptr.as_ptr() }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr.as_ptr() }
    }
}

impl<T> core::ops::Deref for LpBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> core::ops::DerefMut for LpBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: Clone> Clone for LpBox<T> {
    fn clone(&self) -> Self {
        // SAFETY: We know the pointer is valid
        let value_ref = unsafe { &*self.ptr.as_ptr() };
        let cloned_value = value_ref.clone();

        // try_new will allocate new memory and copy the cloned value
        #[cfg(feature = "alloc-meta")]
        let scope = self.meta.scope;
        #[cfg(not(feature = "alloc-meta"))]
        let scope = None;

        Self::try_new_with_scope(cloned_value, scope)
            .expect("Failed to clone LpBox: pool exhausted")
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for LpBox<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Delegate to T's Debug implementation
        core::fmt::Debug::fmt(self.as_ref(), f)
    }
}

impl<T> Drop for LpBox<T> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();

        // CRITICAL: Drop the value BEFORE deallocating memory
        // Otherwise we're dropping from memory that's already been freed
        unsafe {
            core::ptr::drop_in_place(self.ptr.as_ptr());
        }

        // Deallocate
        let _ = with_active_pool(|pool| {
            unsafe {
                pool.deallocate(self.ptr.cast(), layout);
            }

            #[cfg(feature = "alloc-meta")]
            {
                remove_allocation_meta(self.meta, layout.size());
            }

            Ok::<(), AllocError>(())
        });
    }
}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use super::*;
    use crate::allow_global_alloc;
    use crate::memory_pool::LpMemoryPool;

    fn setup_pool() -> LpMemoryPool {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() }
    }

    #[test]
    fn test_pool_box_new() {
        let pool = setup_pool();
        let boxed = pool.run(|| LpBox::try_new(42i32)).unwrap();
        assert_eq!(*boxed, 42);
    }

    #[test]
    fn test_pool_box_deref() {
        let pool = setup_pool();
        let boxed = pool.run(|| LpBox::try_new(100i32)).unwrap();
        assert_eq!(*boxed, 100);
    }

    #[test]
    fn test_pool_box_deref_mut() {
        let pool = setup_pool();
        let mut boxed = pool.run(|| LpBox::try_new(50i32)).unwrap();
        *boxed = 200;
        assert_eq!(*boxed, 200);
    }

    #[test]
    fn test_pool_box_as_ref() {
        let pool = setup_pool();
        let boxed = pool.run(|| LpBox::try_new(99i32)).unwrap();
        let val = boxed.as_ref();
        assert_eq!(*val, 99);
    }

    #[test]
    fn test_pool_box_as_mut() {
        let pool = setup_pool();
        let mut boxed = pool.run(|| LpBox::try_new(1i32)).unwrap();
        *boxed.as_mut() = 999;
        assert_eq!(*boxed, 999);
    }

    #[test]
    fn test_pool_box_with_scope() {
        let pool = setup_pool();
        let boxed = pool
            .run(|| LpBox::try_new_with_scope(42i32, Some("test_scope")))
            .unwrap();
        assert_eq!(*boxed, 42);
    }

    #[test]
    fn test_pool_box_drop() {
        let pool = setup_pool();
        let before = pool.used_bytes().unwrap();

        {
            let _boxed = pool.run(|| LpBox::try_new(42i32)).unwrap();
            let during = pool.used_bytes().unwrap();
            assert!(during > before);
        }

        // After drop, memory should be freed
        let after = pool.used_bytes().unwrap();
        assert_eq!(after, before);
    }

    #[test]
    fn test_pool_box_string() {
        use alloc::string::String;
        let pool = setup_pool();
        let boxed = pool
            .run(|| {
                let value = allow_global_alloc(|| String::from("hello"));
                LpBox::try_new(value)
            })
            .unwrap();
        assert_eq!(*boxed, "hello");
    }

    #[test]
    fn test_drop_before_deallocate() {
        use core::sync::atomic::{AtomicBool, Ordering};

        static DROP_CALLED: AtomicBool = AtomicBool::new(false);

        /// Type that accesses its own data during drop
        /// This would cause UB if memory is freed before drop is called
        struct DropChecker {
            sentinel: u32,
        }

        impl Drop for DropChecker {
            fn drop(&mut self) {
                // CRITICAL: This accesses self.sentinel from memory
                // If memory was already freed, this is undefined behavior
                assert_eq!(
                    self.sentinel, 0xDEADBEEF,
                    "Drop accessed freed memory - sentinel corrupted!"
                );
                DROP_CALLED.store(true, Ordering::SeqCst);
            }
        }

        let pool = setup_pool();
        DROP_CALLED.store(false, Ordering::SeqCst);

        {
            let _boxed = pool
                .run(|| {
                    LpBox::try_new(DropChecker {
                        sentinel: 0xDEADBEEF,
                    })
                })
                .unwrap();
        }

        // Verify drop was called
        assert!(DROP_CALLED.load(Ordering::SeqCst), "Drop was not called!");
    }

    // === Comprehensive Edge Case Tests ===

    #[test]
    fn test_box_large_type() {
        #[derive(Clone, Copy)]
        struct Large([u64; 16]); // 128 bytes

        let pool = setup_pool();
        pool.run(|| {
            let boxed = LpBox::try_new(Large([0x42; 16]))?;
            assert_eq!(boxed.0[0], 0x42);
            assert_eq!(boxed.0[15], 0x42);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_box_zst() {
        // Zero-sized types
        let pool = setup_pool();
        pool.run(|| {
            let boxed = LpBox::try_new(())?;
            assert_eq!(*boxed, ());

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_box_aligned_type() {
        #[repr(align(8))]
        struct Aligned8(u64);

        let pool = setup_pool();
        pool.run(|| {
            let boxed = LpBox::try_new(Aligned8(42))?;
            assert_eq!(boxed.0, 42);

            // Verify alignment
            let ptr = &*boxed as *const Aligned8 as usize;
            assert_eq!(ptr % 8, 0, "Should be aligned to 8 bytes");

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_box_with_drop_order() {
        use core::sync::atomic::{AtomicUsize, Ordering};

        static DROP_ORDER: AtomicUsize = AtomicUsize::new(0);

        struct DropOrderChecker(usize);

        impl Drop for DropOrderChecker {
            fn drop(&mut self) {
                let order = DROP_ORDER.fetch_add(1, Ordering::SeqCst);
                assert_eq!(
                    order, self.0,
                    "Drop called out of order: expected {}, got {}",
                    self.0, order
                );
            }
        }

        let pool = setup_pool();
        DROP_ORDER.store(0, Ordering::SeqCst);

        pool.run(|| {
            let _box1 = LpBox::try_new(DropOrderChecker(2))?; // Dropped first (LIFO)
            let _box2 = LpBox::try_new(DropOrderChecker(1))?; // Dropped second
            let _box3 = LpBox::try_new(DropOrderChecker(0))?; // Dropped last

            Ok::<(), AllocError>(())
        })
        .unwrap();

        assert_eq!(DROP_ORDER.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_box_mutation() {
        let pool = setup_pool();
        pool.run(|| {
            let mut boxed = LpBox::try_new(10i32)?;

            assert_eq!(*boxed, 10);

            *boxed = 20;
            assert_eq!(*boxed, 20);

            *boxed += 5;
            assert_eq!(*boxed, 25);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_box_deref_coercion() {
        let pool = setup_pool();
        pool.run(|| {
            let boxed =
                LpBox::try_new(allow_global_alloc(|| alloc::string::String::from("hello")))?;

            // Deref coercion: LpBox<String> -> &str
            let s: &str = &boxed;
            assert_eq!(s, "hello");
            assert_eq!(s.len(), 5);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_box_as_ref_as_mut() {
        let pool = setup_pool();
        pool.run(|| {
            let mut boxed = LpBox::try_new(100i32)?;

            let r = boxed.as_ref();
            assert_eq!(*r, 100);

            let m = boxed.as_mut();
            *m = 200;
            assert_eq!(*boxed, 200);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_box_tuple() {
        // Box containing tuple
        let pool = setup_pool();
        pool.run(|| {
            let boxed = LpBox::try_new((1, 2, 3))?;
            assert_eq!(boxed.0, 1);
            assert_eq!(boxed.1, 2);
            assert_eq!(boxed.2, 3);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_box_multiple_allocations() {
        let pool = setup_pool();
        pool.run(|| {
            let box1 = LpBox::try_new(1i32)?;
            let box2 = LpBox::try_new(2i32)?;
            let box3 = LpBox::try_new(3i32)?;

            assert_eq!(*box1, 1);
            assert_eq!(*box2, 2);
            assert_eq!(*box3, 3);

            // Verify they're different allocations
            let ptr1 = &*box1 as *const i32;
            let ptr2 = &*box2 as *const i32;
            let ptr3 = &*box3 as *const i32;

            assert_ne!(ptr1, ptr2);
            assert_ne!(ptr2, ptr3);
            assert_ne!(ptr1, ptr3);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_box_with_scope() {
        let pool = setup_pool();
        pool.run(|| {
            let boxed = LpBox::try_new_with_scope(42i32, Some("test_scope"))?;
            assert_eq!(*boxed, 42);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_box_memory_is_freed() {
        let pool = setup_pool();
        let before = pool.used_bytes().unwrap();

        pool.run(|| {
            {
                let _box1 = LpBox::try_new([0u8; 64])?;
                let _box2 = LpBox::try_new([0u8; 64])?;
                // Both dropped here
            }

            let after = pool.used_bytes().unwrap();
            assert_eq!(after, before, "Memory should be freed after boxes drop");

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_box_array() {
        let pool = setup_pool();
        pool.run(|| {
            let boxed = LpBox::try_new([1, 2, 3, 4, 5])?;

            assert_eq!(boxed[0], 1);
            assert_eq!(boxed[4], 5);
            assert_eq!(boxed.len(), 5);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}
