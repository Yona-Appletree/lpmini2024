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

    pub fn as_ref(&self) -> &T {
        unsafe { &*self.ptr.as_ptr() }
    }

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
        let boxed = pool.run(|| LpBox::try_new(String::from("hello"))).unwrap();
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
}
