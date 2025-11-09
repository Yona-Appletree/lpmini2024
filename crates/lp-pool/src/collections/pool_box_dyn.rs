//! Pool-backed Box for unsized types (trait objects).
//!
//! This module provides `LpBoxDyn<T: ?Sized>` which can store trait objects
//! and other unsized types, similar to `Box<dyn Trait>` but using lp-pool allocation.

use core::alloc::Layout;
use core::ptr::NonNull;

use crate::error::AllocError;
use crate::memory_pool::with_active_pool;

/// Macro to create `LpBoxDyn<dyn Trait>` from a concrete value.
///
/// This macro handles the coercion from concrete type to trait object.
/// The concrete type must implement the specified trait.
///
/// # Example
/// ```ignore
/// trait TestTrait {
///     fn value(&self) -> i32;
/// }
///
/// struct TestStruct(i32);
/// impl TestTrait for TestStruct {
///     fn value(&self) -> i32 { self.0 }
/// }
///
/// let concrete = TestStruct(42);
/// let boxed = lp_box_dyn!(concrete, dyn TestTrait)?;
/// ```
#[macro_export]
macro_rules! lp_box_dyn {
    ($value:expr, $trait:ty) => {{
        // Create a temporary reference to extract the vtable
        let temp_ref: &_ = &$value;
        // Coerce to trait object - this creates the fat pointer with vtable
        let trait_ref: &$trait = temp_ref;

        // Extract vtable from the fat pointer using a union
        union VtableExtractor {
            ptr: *const $trait,
            repr: [usize; 2],
        }
        let extractor = VtableExtractor {
            ptr: trait_ref as *const $trait,
        };
        let vtable_ptr = unsafe { extractor.repr }[1];

        // Now move the value and create LpBoxDyn
        // The return type is inferred from the trait type
        $crate::collections::LpBoxDyn::<$trait>::try_new_from_with_vtable($value, vtable_ptr)
    }};
}

/// Pool-backed Box for unsized types (trait objects).
///
/// Similar to `Box<dyn Trait>` but allocates from lp-pool instead of the global allocator.
/// Can store trait objects and other unsized types.
pub struct LpBoxDyn<T: ?Sized> {
    ptr: NonNull<T>,
}

impl<T: ?Sized> LpBoxDyn<T> {
    /// Internal helper to create LpBoxDyn from a concrete value and vtable pointer.
    ///
    /// This is used by the `lp_box_dyn!` macro after extracting the vtable.
    /// The value is moved into pool-allocated memory, preventing double free issues.
    #[doc(hidden)]
    pub fn try_new_from_with_vtable<U>(value: U, vtable_ptr: usize) -> Result<Self, AllocError>
    where
        U: Sized,
    {
        let layout = Layout::new::<U>();

        let ptr = with_active_pool(|pool| {
            let allocated = pool.allocate(layout)?;
            let concrete_ptr = NonNull::new(allocated.as_ptr() as *mut U).unwrap();

            // Move value into allocated memory
            unsafe {
                core::ptr::write(concrete_ptr.as_ptr(), value);
            }

            // Construct fat pointer with our data pointer and the vtable
            let data_ptr = concrete_ptr.as_ptr() as usize;

            // A fat pointer is represented as two usize values: [data_ptr, vtable_ptr]
            // We use a union to construct it from the representation
            // The union allows us to convert between the representation and the pointer type
            union FatPtrConstructor<T: ?Sized> {
                repr: [usize; 2],
                fat: *const T,
            }

            // Create the fat pointer representation
            let fat_ptr_repr = [data_ptr, vtable_ptr];

            // Convert to *const T using the union
            // Both are two words (two usize values), so this is safe
            let constructor = FatPtrConstructor::<T> { repr: fat_ptr_repr };
            let fat_ptr: *const T = unsafe { constructor.fat };

            let ptr = unsafe { NonNull::new_unchecked(fat_ptr as *mut T) };

            Ok(ptr)
        })?;

        Ok(LpBoxDyn { ptr })
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

impl<T: ?Sized> core::ops::Deref for LpBoxDyn<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: ?Sized> core::ops::DerefMut for LpBoxDyn<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: ?Sized> Drop for LpBoxDyn<T> {
    fn drop(&mut self) {
        // For unsized types, we need to get the layout from the value itself
        // We can use the pointer to get size/align via the vtable (for trait objects)
        let value_ref = unsafe { &*self.ptr.as_ptr() };
        let layout = Layout::for_value(value_ref);

        // CRITICAL: Drop the value BEFORE deallocating memory
        // For trait objects, this uses the vtable's drop function
        unsafe {
            core::ptr::drop_in_place(self.ptr.as_ptr());
        }

        // Deallocate
        let _ = with_active_pool(|pool| {
            unsafe {
                pool.deallocate(self.ptr.cast(), layout);
            }

            Ok::<(), AllocError>(())
        });
    }
}

impl<T: core::fmt::Debug + ?Sized> core::fmt::Debug for LpBoxDyn<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self.as_ref(), f)
    }
}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use super::*;
    use crate::memory_pool::LpMemoryPool;
    use crate::LpString;

    fn setup_pool() -> LpMemoryPool {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() }
    }

    trait TestTrait {
        fn value(&self) -> i32;
    }

    struct TestStruct(i32);

    impl TestTrait for TestStruct {
        fn value(&self) -> i32 {
            self.0
        }
    }

    #[test]
    fn test_lp_box_dyn_basic() {
        let pool = setup_pool();
        pool.run(|| {
            let concrete = TestStruct(42);
            let boxed = lp_box_dyn!(concrete, dyn TestTrait)?;
            assert_eq!(boxed.value(), 42);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_lp_box_dyn_deref() {
        let pool = setup_pool();
        pool.run(|| {
            let concrete = TestStruct(100);
            let boxed = lp_box_dyn!(concrete, dyn TestTrait)?;
            let trait_ref: &dyn TestTrait = &*boxed;
            assert_eq!(trait_ref.value(), 100);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_lp_box_dyn_drop() {
        let pool = setup_pool();
        let before = pool.used_bytes().unwrap();

        {
            let _boxed = pool
                .run(|| {
                    let concrete = TestStruct(42);
                    lp_box_dyn!(concrete, dyn TestTrait)
                })
                .unwrap();
            let during = pool.used_bytes().unwrap();
            assert!(during > before);
        }

        // After drop, memory should be freed
        let after = pool.used_bytes().unwrap();
        assert_eq!(after, before);
    }

    /// Test that demonstrates the double free problem would occur with copying.
    ///
    /// This test creates a struct containing pool-allocated types (LpString).
    /// If we were to bitwise copy this struct, both the original and copy would
    /// try to deallocate the same pool memory, causing a double free.
    ///
    /// With `try_new_from`, the value is moved (not copied), so only one drop occurs.
    #[test]
    fn test_double_free_prevention() {
        use core::sync::atomic::{AtomicUsize, Ordering};

        static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

        struct TestStructWithPoolData {
            name: LpString,
        }

        impl Drop for TestStructWithPoolData {
            fn drop(&mut self) {
                DROP_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }

        trait TestTraitWithPoolData {
            fn get_name(&self) -> &str;
        }

        impl TestTraitWithPoolData for TestStructWithPoolData {
            fn get_name(&self) -> &str {
                self.name.as_str()
            }
        }

        let pool = setup_pool();
        DROP_COUNT.store(0, Ordering::SeqCst);

        pool.run(|| {
            let name = LpString::try_from_str("test")?;
            let concrete = TestStructWithPoolData { name };

            // Move into LpBoxDyn - this should only result in one drop
            let _boxed = lp_box_dyn!(concrete, dyn TestTraitWithPoolData)?;

            // Verify the boxed value works
            assert_eq!(_boxed.get_name(), "test");

            // concrete is now moved, so it won't be dropped
            // Only _boxed will be dropped when it goes out of scope
            Ok::<(), AllocError>(())
        })
        .unwrap();

        // After the pool.run closure, _boxed is dropped
        // We should have exactly one drop (not two, which would indicate double free)
        let drop_count = DROP_COUNT.load(Ordering::SeqCst);
        assert_eq!(
            drop_count, 1,
            "Expected exactly one drop. Got {} drops, which indicates double free would occur with copying approach.",
            drop_count
        );
    }

    #[test]
    fn test_lp_box_dyn_multiple() {
        let pool = setup_pool();
        pool.run(|| {
            let box1 = lp_box_dyn!(TestStruct(1), dyn TestTrait)?;
            let box2 = lp_box_dyn!(TestStruct(2), dyn TestTrait)?;
            let box3 = lp_box_dyn!(TestStruct(3), dyn TestTrait)?;

            assert_eq!(box1.value(), 1);
            assert_eq!(box2.value(), 2);
            assert_eq!(box3.value(), 3);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}
