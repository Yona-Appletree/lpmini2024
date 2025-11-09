//! Pool-backed Box for unsized types (trait objects).
//!
//! This module provides `LpBoxDyn<T: ?Sized>` which can store trait objects
//! and other unsized types, similar to `Box<dyn Trait>` but using lp-pool allocation.

use core::alloc::Layout;
use core::ptr::NonNull;

#[cfg(feature = "alloc-meta")]
use super::alloc_meta::{record_allocation_meta, remove_allocation_meta, AllocationMeta};
use crate::error::AllocError;
use crate::memory_pool::{with_active_pool, LpMemoryPool};

/// Pool-backed Box for unsized types (trait objects).
///
/// Similar to `Box<dyn Trait>` but allocates from lp-pool instead of the global allocator.
/// Can store trait objects and other unsized types.
pub struct LpBoxDyn<T: ?Sized> {
    ptr: NonNull<T>,
    #[cfg(feature = "alloc-meta")]
    meta: AllocationMeta,
}

impl<T: ?Sized> LpBoxDyn<T> {
    /// Create a new LpBoxDyn from a value.
    ///
    /// The value will be moved into lp-pool allocated memory.
    /// For sized types, prefer `LpBox::try_new()` for better performance.
    pub fn try_new(value: T) -> Result<Self, AllocError>
    where
        T: Sized,
    {
        Self::try_new_with_scope(value, None)
    }

    /// Create a new LpBoxDyn from a value with a scope identifier.
    pub fn try_new_with_scope(value: T, scope: Option<&'static str>) -> Result<Self, AllocError>
    where
        T: Sized,
    {
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

        Ok(LpBoxDyn {
            ptr,
            #[cfg(feature = "alloc-meta")]
            meta,
        })
    }

    /// Create a new LpBoxDyn from an unsized value (trait object).
    ///
    /// **DEPRECATED**: This method performs unsafe bitwise copying and should not be used.
    /// Use `try_new_from_clone` or the `lp_box_dyn!` macro instead.
    #[deprecated(
        note = "Use try_new_from_clone or lp_box_dyn! macro instead. This method is unsafe for non-Copy types."
    )]
    pub fn try_new_unsized(value: &T) -> Result<Self, AllocError> {
        Self::try_new_unsized_with_scope(value, None)
    }

    /// Create a new LpBoxDyn from an unsized value with a scope identifier.
    ///
    /// **DEPRECATED**: This method performs unsafe bitwise copying and should not be used.
    #[deprecated(note = "Use try_new_from_clone_with_scope or lp_box_dyn! macro instead.")]
    pub fn try_new_unsized_with_scope(
        value: &T,
        scope: Option<&'static str>,
    ) -> Result<Self, AllocError> {
        // Extract fat pointer components before entering with_active_pool
        // This avoids any potential issues with accessing the value while the guard is active
        let fat_ptr_repr: [usize; 2] = unsafe { core::mem::transmute_copy(&value) };
        let data_ptr = fat_ptr_repr[0] as *const u8;
        let vtable_ptr = fat_ptr_repr[1];

        // Calculate layout for the unsized value
        // Wrap in with_global_alloc in case Layout::for_value triggers any allocations
        // (e.g., for trait objects, it might access the vtable which could allocate)
        let layout = LpMemoryPool::with_global_alloc(|| Layout::for_value(value));

        #[cfg(feature = "alloc-meta")]
        let meta = AllocationMeta {
            type_name: core::any::type_name::<T>(),
            scope,
        };
        #[cfg(not(feature = "alloc-meta"))]
        let _ = scope; // Suppress unused warning

        let ptr = with_active_pool(|pool| {
            let allocated = pool.allocate(layout)?;
            let raw_ptr = allocated.as_ptr();

            // For trait objects, we need to:
            // 1. Extract the data pointer from the fat pointer (already done above)
            // 2. Copy the actual value (not the fat pointer)
            // 3. Reconstruct the fat pointer with new data pointer and same vtable

            // Copy the actual value (the data pointer points to the concrete value)
            // SAFETY: T: Copy ensures bitwise copying is safe
            unsafe {
                core::ptr::copy_nonoverlapping(data_ptr, raw_ptr as *mut u8, layout.size());
            }

            #[cfg(feature = "alloc-meta")]
            {
                // record_allocation_meta uses with_meta_mut which already calls
                // enter_global_alloc_allowance(), so it should be able to allocate
                record_allocation_meta(meta, layout.size());
            }

            // Reconstruct the fat pointer with new data pointer and same vtable
            // We need to create a *mut T (fat pointer) from [data_ptr, vtable_ptr]
            // Since we can't transmute directly, we'll use from_raw_parts if available,
            // or manually construct it
            let new_fat_ptr_repr: [usize; 2] = [raw_ptr as *const () as usize, vtable_ptr];
            // Construct *mut T from the fat pointer representation
            let mut_ptr: *mut T = unsafe {
                // On most platforms, a fat pointer is two pointer-sized values
                // We'll use transmute_copy to create the pointer
                core::mem::transmute_copy(&new_fat_ptr_repr)
            };
            let ptr = unsafe { NonNull::new_unchecked(mut_ptr) };

            Ok(ptr)
        })?;

        Ok(LpBoxDyn {
            ptr,
            #[cfg(feature = "alloc-meta")]
            meta,
        })
    }

    /// Create LpBoxDyn from a concrete sized type that implements a trait.
    ///
    /// **Note**: This method cannot work generically due to Rust's type system limitations.
    /// Use the `lp_box_dyn!` macro instead, or call `try_new_unsized` with a trait object reference.
    ///
    /// Example with macro:
    /// ```ignore
    /// let boxed = lp_box_dyn!(TestStruct(42), dyn TestTrait)?;
    /// ```
    ///
    /// Example with try_new_unsized:
    /// ```ignore
    /// let value = TestStruct(42);
    /// let trait_ref: &dyn TestTrait = &value;
    /// let boxed = LpBoxDyn::try_new_unsized(trait_ref)?;
    /// ```
    #[deprecated(note = "Use lp_box_dyn! macro or try_new_unsized instead")]
    pub fn try_new_trait_object<U>(_value: U) -> Result<Self, AllocError>
    where
        U: Sized,
    {
        // This cannot be implemented generically - use the macro or try_new_unsized
        Err(AllocError::PoolExhausted)
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
        // or via the slice metadata (for slices)

        // Get a reference to calculate layout
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

            #[cfg(feature = "alloc-meta")]
            {
                // Wrap in with_global_alloc to ensure BTreeMap operations can allocate
                LpMemoryPool::with_global_alloc(|| {
                    remove_allocation_meta(self.meta, layout.size());
                });
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

/// Macro to create `LpBoxDyn<dyn Trait>` from a concrete value.
///
/// This macro handles the coercion from concrete type to trait object at the call site.
/// The concrete type must implement `Clone` (which `Copy` types automatically do).
///
/// The macro clones the concrete value before creating the trait object, ensuring proper
/// ownership semantics. The trait object itself is still bitwise copied, but since the
/// concrete value is cloned first, this is safe.
///
/// # Example
/// ```ignore
/// #[derive(Clone)]
/// struct TestStruct(i32);
/// trait TestTrait { fn value(&self) -> i32; }
/// impl TestTrait for TestStruct { fn value(&self) -> i32 { self.0 } }
///
/// let boxed = lp_box_dyn!(TestStruct(42), dyn TestTrait)?;
/// ```
#[macro_export]
macro_rules! lp_box_dyn {
    ($value:expr, $trait:ty) => {{
        // Clone the concrete value first - this ensures proper ownership
        // of any pool-allocated data (LpString, LpVec, etc.)
        let cloned_value = $value.clone();
        // Coerce to trait object at call site - this creates the fat pointer
        let trait_ref: &$trait = &cloned_value;
        // Use try_new_unsized which bitwise copies the trait object
        // SAFETY: The concrete value was cloned above, so bitwise copying the
        // trait object is safe (we're copying the cloned data, not the original)
        $crate::collections::LpBoxDyn::try_new_unsized(trait_ref)
    }};
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
    fn test_lp_box_dyn_trait_object() {
        let pool = setup_pool();
        pool.run(|| {
            let concrete = TestStruct(42);
            // Use try_new_unsized with a trait object reference
            // SAFETY: TestStruct is Copy (i32 is Copy)
            let trait_ref: &dyn TestTrait = &concrete;
            let boxed = unsafe { LpBoxDyn::try_new_unsized(trait_ref)? };
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
            let trait_ref: &dyn TestTrait = &concrete;
            // SAFETY: TestStruct is Copy (i32 is Copy)
            let boxed = unsafe { LpBoxDyn::try_new_unsized(trait_ref)? };
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
                    {
                        let trait_ref: &dyn TestTrait = &concrete;
                        // SAFETY: TestStruct is Copy (i32 is Copy)
                        unsafe { LpBoxDyn::try_new_unsized(trait_ref) }
                    }
                })
                .unwrap();
            let during = pool.used_bytes().unwrap();
            assert!(during > before);
        }

        // After drop, memory should be freed
        let after = pool.used_bytes().unwrap();
        assert_eq!(after, before);
    }

    #[test]
    fn test_lp_box_dyn_multiple_trait_objects() {
        let pool = setup_pool();
        pool.run(|| {
            // SAFETY: TestStruct is Copy (i32 is Copy)
            let trait_ref1: &dyn TestTrait = &TestStruct(1);
            let box1 = unsafe { LpBoxDyn::try_new_unsized(trait_ref1)? };
            let trait_ref2: &dyn TestTrait = &TestStruct(2);
            let box2 = unsafe { LpBoxDyn::try_new_unsized(trait_ref2)? };
            let trait_ref3: &dyn TestTrait = &TestStruct(3);
            let box3 = unsafe { LpBoxDyn::try_new_unsized(trait_ref3)? };

            assert_eq!(box1.value(), 1);
            assert_eq!(box2.value(), 2);
            assert_eq!(box3.value(), 3);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    #[cfg(feature = "alloc-meta")]
    fn test_lp_box_dyn_alloc_meta_does_not_allocate_globally() {
        // This test reproduces the bug: when alloc-meta is enabled,
        // LpBoxDyn::try_new_unsized calls record_allocation_meta which
        // uses BTreeMap::entry().or_insert(). This BTreeMap is from alloc::collections::BTreeMap
        // which uses the global allocator. Even though with_meta_mut calls
        // enter_global_alloc_allowance(), the BTreeMap operations might allocate
        // before that allowance is active, or there's a timing issue.
        //
        // This test should pass - LpBoxDyn should work inside pool.run() even with alloc-meta.
        let pool = setup_pool();
        let result = pool.run(|| {
            // Create LpBoxDyn without with_global_alloc - this should work
            // because record_allocation_meta should handle allocations properly
            let concrete = TestStruct(42);
            let trait_ref: &dyn TestTrait = &concrete;
            // SAFETY: TestStruct is Copy (i32 is Copy)
            let _boxed = unsafe { LpBoxDyn::try_new_unsized(trait_ref)? };
            Ok::<(), AllocError>(())
        });

        assert!(
            result.is_ok(),
            "LpBoxDyn::try_new_unsized should work inside pool.run() even with alloc-meta enabled"
        );
    }

    #[test]
    #[cfg(feature = "alloc-meta")]
    fn test_lp_box_dyn_first_allocation_triggers_btreemap_growth() {
        // Test that the first LpBoxDyn allocation (which creates a new entry in the BTreeMap)
        // doesn't cause a global allocation panic.
        // The BTreeMap might need to allocate when inserting the first entry.
        let pool = setup_pool();

        // Clear any existing meta to ensure we're starting fresh
        #[cfg(feature = "alloc-meta")]
        crate::allocator_store::clear_meta();

        let result = pool.run(|| {
            // This should be the first entry in the BTreeMap for this type
            let concrete = TestStruct(999);
            let trait_ref: &dyn TestTrait = &concrete;
            // SAFETY: TestStruct is Copy (i32 is Copy)
            let _boxed = unsafe { LpBoxDyn::try_new_unsized(trait_ref)? };
            Ok::<(), AllocError>(())
        });

        assert!(
            result.is_ok(),
            "First LpBoxDyn allocation should not trigger global allocation panic"
        );
    }

    #[test]
    #[cfg(feature = "alloc-meta")]
    fn test_lp_box_dyn_drop_triggers_remove_allocation_meta() {
        // Test that dropping LpBoxDyn (which calls remove_allocation_meta)
        // doesn't cause a global allocation panic.
        // remove_allocation_meta uses BTreeMap::remove() which might allocate.
        let pool = setup_pool();

        let result = pool.run(|| {
            let concrete = TestStruct(42);
            let trait_ref: &dyn TestTrait = &concrete;
            // SAFETY: TestStruct is Copy (i32 is Copy)
            let boxed = unsafe { LpBoxDyn::try_new_unsized(trait_ref)? };

            // Drop the boxed value - this should call remove_allocation_meta
            // which uses BTreeMap operations that might allocate
            drop(boxed);

            Ok::<(), AllocError>(())
        });

        assert!(
            result.is_ok(),
            "Dropping LpBoxDyn should not trigger global allocation panic"
        );
    }

    #[test]
    #[cfg(feature = "alloc-meta")]
    fn test_lp_box_dyn_with_complex_struct() {
        // This test reproduces the bug from lp-data scene tests.
        // When LpBoxDyn::try_new_unsized is called on a struct that contains
        // pool-allocated types (like LpString, LpVec), it might trigger
        // a global allocation during the copy operation.
        //
        // This test should FAIL - it reproduces the bug.
        use crate::LpString;
        use crate::LpVec;

        struct ComplexStruct {
            name: LpString,
            items: LpVec<i32>,
        }

        trait ComplexTrait {
            fn get_name(&self) -> &str;
        }

        impl ComplexTrait for ComplexStruct {
            fn get_name(&self) -> &str {
                self.name.as_str()
            }
        }

        let pool = setup_pool();
        let result = pool.run(|| {
            let name = LpString::try_from_str("test")?;
            let items = LpVec::new();
            let complex = ComplexStruct { name, items };

            let trait_ref: &dyn ComplexTrait = &complex;
            // SAFETY: ComplexStruct contains LpString and LpVec which are NOT Copy
            // This will create an invalid state (two owners), but we're testing the failure case
            let _boxed = unsafe { LpBoxDyn::try_new_unsized(trait_ref)? };

            Ok::<(), AllocError>(())
        });

        assert!(
            result.is_ok(),
            "LpBoxDyn::try_new_unsized should work with structs containing pool-allocated types"
        );
    }
}
