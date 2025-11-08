use crate::error::AllocError;
use crate::memory_pool::with_active_pool;
use core::alloc::Layout;
use core::ptr::NonNull;

/// Iterator over LpVec
pub struct LpVecIter<'a, T> {
    vec: &'a LpVec<T>,
    start: usize,
    end: usize,
}

impl<'a, T> Iterator for LpVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let item = self.vec.get(self.start);
            self.start += 1;
            item
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.start);
        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for LpVecIter<'a, T> {
    fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

impl<'a, T> DoubleEndedIterator for LpVecIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= 1;
            self.vec.get(self.end)
        } else {
            None
        }
    }
}

/// Mutable iterator over LpVec
pub struct LpVecIterMut<'a, T> {
    vec: *mut LpVec<T>,
    index: usize,
    len: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for LpVecIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            unsafe {
                let vec = &mut *self.vec;
                let item = vec.get_mut(self.index);
                self.index += 1;
                // Safety: We have exclusive access to vec, and we're returning non-overlapping refs
                item.map(|r| &mut *(r as *mut T))
            }
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for LpVecIterMut<'a, T> {
    fn len(&self) -> usize {
        self.len.saturating_sub(self.index)
    }
}

#[cfg(feature = "alloc-meta")]
use super::alloc_meta::{record_allocation_meta, remove_allocation_meta, AllocationMeta};

/// Pool-backed Vec
pub struct LpVec<T> {
    data: NonNull<u8>,
    len: usize,
    capacity: usize,
    _marker: core::marker::PhantomData<T>,
    #[cfg(feature = "alloc-meta")]
    scope: Option<&'static str>,
}

impl<T> LpVec<T> {
    pub fn new() -> Self {
        LpVec {
            data: NonNull::dangling(),
            len: 0,
            capacity: 0,
            _marker: core::marker::PhantomData,
            #[cfg(feature = "alloc-meta")]
            scope: None,
        }
    }

    /// Create a new LpVec with a scope identifier for metadata tracking
    #[cfg(feature = "alloc-meta")]
    pub fn new_with_scope(scope: Option<&'static str>) -> Self {
        LpVec {
            data: NonNull::dangling(),
            len: 0,
            capacity: 0,
            _marker: core::marker::PhantomData,
            scope,
        }
    }

    /// Create a new LpVec with a scope identifier for metadata tracking
    #[cfg(not(feature = "alloc-meta"))]
    pub fn new_with_scope(_scope: Option<&'static str>) -> Self {
        Self::new()
    }

    pub fn try_push(&mut self, item: T) -> Result<(), AllocError> {
        if self.len >= self.capacity {
            // Need to grow
            let new_cap = if self.capacity == 0 {
                4
            } else {
                self.capacity * 2
            };
            self.try_reserve(new_cap)?;
        }

        unsafe {
            let ptr = self.data.as_ptr().add(self.len * core::mem::size_of::<T>()) as *mut T;
            core::ptr::write(ptr, item);
        }
        self.len += 1;
        Ok(())
    }

    pub fn try_reserve(&mut self, new_cap: usize) -> Result<(), AllocError> {
        if new_cap <= self.capacity {
            return Ok(());
        }

        let new_layout = Layout::array::<T>(new_cap).map_err(|_| AllocError::InvalidLayout)?;

        #[cfg(feature = "alloc-meta")]
        let meta = AllocationMeta {
            type_name: core::any::type_name::<T>(),
            scope: self.scope,
        };

        with_active_pool(|pool| {
            let new_data = if self.capacity > 0 {
                // Use grow to reallocate
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                let new_size = new_layout.size();

                #[cfg(feature = "alloc-meta")]
                {
                    remove_allocation_meta(meta, old_layout.size());
                }

                unsafe {
                    let grown = pool.grow(self.data, old_layout, new_size)?;

                    #[cfg(feature = "alloc-meta")]
                    {
                        record_allocation_meta(meta, new_layout.size());
                    }

                    grown
                }
            } else {
                // First allocation
                let allocated = pool.allocate(new_layout)?;

                #[cfg(feature = "alloc-meta")]
                {
                    record_allocation_meta(meta, new_layout.size());
                }

                allocated
            };

            self.data = NonNull::new(new_data.as_ptr() as *mut u8).unwrap();
            self.capacity = new_cap;
            Ok(())
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn last(&self) -> Option<&T> {
        if self.len > 0 {
            self.get(self.len - 1)
        } else {
            None
        }
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.len > 0 {
            let last_idx = self.len - 1;
            self.get_mut(last_idx)
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        unsafe {
            let ptr = self.data.as_ptr().add(self.len * core::mem::size_of::<T>()) as *mut T;
            Some(core::ptr::read(ptr))
        }
    }

    pub fn clear(&mut self) {
        // Drop all elements
        unsafe {
            for i in 0..self.len {
                let ptr = self.data.as_ptr().add(i * core::mem::size_of::<T>()) as *mut T;
                core::ptr::drop_in_place(ptr);
            }
        }
        self.len = 0;
    }

    pub fn iter(&self) -> LpVecIter<'_, T> {
        LpVecIter {
            vec: self,
            start: 0,
            end: self.len,
        }
    }

    pub fn iter_mut(&mut self) -> LpVecIterMut<'_, T> {
        LpVecIterMut {
            vec: self as *mut Self,
            index: 0,
            len: self.len,
            _marker: core::marker::PhantomData,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        unsafe {
            let ptr = self.data.as_ptr().add(index * core::mem::size_of::<T>()) as *const T;
            Some(&*ptr)
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        unsafe {
            let ptr = self.data.as_ptr().add(index * core::mem::size_of::<T>()) as *mut T;
            Some(&mut *ptr)
        }
    }

    /// Get raw slice of the underlying data (for internal use)
    ///
    /// # Safety
    /// This method assumes that `self.data` is properly aligned for type `T`.
    /// The caller must ensure that the pool allocator provides blocks aligned to
    /// at least `align_of::<T>()`. If alignment requirements are not met, this
    /// may result in undefined behavior.
    pub(crate) fn as_raw_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.data.as_ptr() as *const T, self.len) }
    }
}

impl<T> Drop for LpVec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            let layout = Layout::array::<T>(self.capacity).unwrap();

            // CRITICAL: Drop all elements BEFORE deallocating memory
            // Otherwise we're dropping from memory that's already been freed
            unsafe {
                for i in 0..self.len {
                    let ptr = self.data.as_ptr().add(i * core::mem::size_of::<T>()) as *mut T;
                    core::ptr::drop_in_place(ptr);
                }
            }

            #[cfg(feature = "alloc-meta")]
            {
                let meta = AllocationMeta {
                    type_name: core::any::type_name::<T>(),
                    scope: self.scope,
                };
                remove_allocation_meta(meta, layout.size());
            }

            let _ = with_active_pool(|pool| {
                unsafe {
                    pool.deallocate(self.data, layout);
                }
                Ok::<(), AllocError>(())
            });
        }
    }
}

impl<T> Default for LpVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> core::ops::Index<usize> for LpVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T> core::ops::IndexMut<usize> for LpVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::LpMemoryPool;
    use core::ptr::NonNull;

    fn setup_pool() -> LpMemoryPool {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() }
    }

    #[test]
    fn test_vec_new() {
        let pool = setup_pool();
        pool.run(|| {
            let vec = LpVec::<i32>::new();
            assert_eq!(vec.len(), 0);
            assert_eq!(vec.capacity(), 0);
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_push() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;
            assert_eq!(vec.len(), 3);
            assert!(vec.capacity() >= 3);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_get() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(10)?;
            vec.try_push(20)?;
            vec.try_push(30)?;

            assert_eq!(vec.get(0), Some(&10));
            assert_eq!(vec.get(1), Some(&20));
            assert_eq!(vec.get(2), Some(&30));
            assert_eq!(vec.get(3), None);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_get_mut() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(10)?;
            vec.try_push(20)?;

            if let Some(val) = vec.get_mut(0) {
                *val = 100;
            }

            assert_eq!(vec.get(0), Some(&100));
            assert_eq!(vec.get(1), Some(&20));

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_growth() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();

            // Push more than initial capacity (4)
            for i in 0..10 {
                vec.try_push(i)?;
            }

            assert_eq!(vec.len(), 10);
            assert!(vec.capacity() >= 10);

            // Verify all values
            for i in 0..10 {
                assert_eq!(vec.get(i), Some(&i));
            }

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_reserve() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_reserve(20)?;
            assert!(vec.capacity() >= 20);

            // Should be able to push without reallocating
            let old_cap = vec.capacity();
            vec.try_push(1)?;
            assert_eq!(vec.capacity(), old_cap);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[cfg(feature = "alloc-meta")]
    #[test]
    fn test_vec_with_scope() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::<i32>::new_with_scope(Some("test_scope"));
            vec.try_push(1)?;
            vec.try_push(2)?;
            assert_eq!(vec.len(), 2);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    // Test for design issue #6: Pointer arithmetic alignment
    // This test verifies that LpVec handles aligned types correctly
    // Note: This test may fail if the pool doesn't provide aligned blocks
    #[test]
    fn test_vec_aligned_types() {
        #[repr(align(16))]
        struct Aligned16(u64);

        // Use a pool with block_size that's a multiple of 16 to ensure alignment
        let mut memory = [0u8; 16384];
        // Align memory to 16 bytes
        let memory_ptr = {
            let addr = memory.as_mut_ptr() as usize;
            let aligned_addr = (addr + 15) & !15;
            NonNull::new(aligned_addr as *mut u8).unwrap()
        };
        let pool = unsafe {
            LpMemoryPool::new(
                memory_ptr,
                16384 - (memory_ptr.as_ptr() as usize - memory.as_mut_ptr() as usize),
            )
            .unwrap()
        };

        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(Aligned16(1))?;
            vec.try_push(Aligned16(2))?;
            vec.try_push(Aligned16(3))?;

            // Verify we can access the values correctly
            assert_eq!(vec.get(0).unwrap().0, 1);
            assert_eq!(vec.get(1).unwrap().0, 2);
            assert_eq!(vec.get(2).unwrap().0, 3);

            // Verify alignment is maintained (if allocation succeeded)
            let slice = vec.as_raw_slice();
            let ptr = slice.as_ptr() as usize;
            assert_eq!(ptr % 16, 0, "Vec data should be aligned to 16 bytes");

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    // Test for design issue #12: Missing is_empty method
    #[test]
    fn test_vec_is_empty() {
        let pool = setup_pool();
        pool.run(|| {
            let vec = LpVec::<i32>::new();
            assert!(vec.is_empty());

            let mut vec2 = LpVec::new();
            vec2.try_push(1)?;
            assert!(!vec2.is_empty());
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    // Test for indexing
    #[test]
    fn test_vec_index() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(10)?;
            vec.try_push(20)?;
            vec.try_push(30)?;

            assert_eq!(vec[0], 10);
            assert_eq!(vec[1], 20);
            assert_eq!(vec[2], 30);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_index_mut() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(10)?;
            vec.try_push(20)?;

            vec[0] = 100;
            vec[1] = 200;

            assert_eq!(vec[0], 100);
            assert_eq!(vec[1], 200);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    // Test for iteration
    #[test]
    fn test_vec_iter() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;

            let sum: i32 = vec.iter().sum();
            assert_eq!(sum, 6);

            let collected: alloc::vec::Vec<_> = vec.iter().collect();
            assert_eq!(collected, alloc::vec![&1, &2, &3]);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_iter_mut() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;

            for val in vec.iter_mut() {
                *val *= 10;
            }

            assert_eq!(vec[0], 10);
            assert_eq!(vec[1], 20);
            assert_eq!(vec[2], 30);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_iter_rev() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;

            let collected: alloc::vec::Vec<_> = vec.iter().rev().copied().collect();
            assert_eq!(collected, alloc::vec![3, 2, 1]);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    // Test for last() and last_mut()
    #[test]
    fn test_vec_last() {
        let pool = setup_pool();
        pool.run(|| {
            let vec = LpVec::<i32>::new();
            assert_eq!(vec.last(), None);

            let mut vec2 = LpVec::new();
            vec2.try_push(1)?;
            vec2.try_push(2)?;
            vec2.try_push(3)?;
            assert_eq!(vec2.last(), Some(&3));
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_last_mut() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;

            if let Some(last) = vec.last_mut() {
                *last = 99;
            }

            assert_eq!(vec[1], 99);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_drop_before_deallocate() {
        use core::sync::atomic::{AtomicUsize, Ordering};

        static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

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
                DROP_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }

        let pool = setup_pool();
        DROP_COUNT.store(0, Ordering::SeqCst);

        pool.run(|| {
            {
                let mut vec = LpVec::new();
                vec.try_push(DropChecker {
                    sentinel: 0xDEADBEEF,
                })?;
                vec.try_push(DropChecker {
                    sentinel: 0xDEADBEEF,
                })?;
                vec.try_push(DropChecker {
                    sentinel: 0xDEADBEEF,
                })?;
            }
            // Vec dropped here - all 3 elements should be dropped

            // Verify all drops were called
            assert_eq!(
                DROP_COUNT.load(Ordering::SeqCst),
                3,
                "All 3 elements should have been dropped!"
            );

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    // === Comprehensive Edge Case Tests ===

    #[test]
    fn test_vec_zst() {
        // Zero-sized types
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::<()>::new();
            vec.try_push(())?;
            vec.try_push(())?;
            vec.try_push(())?;

            assert_eq!(vec.len(), 3);
            assert_eq!(vec.get(0), Some(&()));
            assert_eq!(vec.get(1), Some(&()));
            assert_eq!(vec.get(2), Some(&()));

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_pop_empty() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::<i32>::new();

            // Pop from empty vec
            assert_eq!(vec.pop(), None);

            // Push one, pop one
            vec.try_push(42)?;
            assert_eq!(vec.pop(), Some(42));

            // Pop again from empty
            assert_eq!(vec.pop(), None);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_clear_with_drop() {
        use core::sync::atomic::{AtomicUsize, Ordering};

        static CLEAR_DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

        struct DropCounter;
        impl Drop for DropCounter {
            fn drop(&mut self) {
                CLEAR_DROP_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }

        let pool = setup_pool();
        CLEAR_DROP_COUNT.store(0, Ordering::SeqCst);

        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(DropCounter)?;
            vec.try_push(DropCounter)?;
            vec.try_push(DropCounter)?;
            vec.try_push(DropCounter)?;

            vec.clear();

            // All 4 should be dropped
            assert_eq!(CLEAR_DROP_COUNT.load(Ordering::SeqCst), 4);
            assert_eq!(vec.len(), 0);

            // Can still push after clear
            vec.try_push(DropCounter)?;
            assert_eq!(vec.len(), 1);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_nested_drop() {
        use alloc::string::String;
        let pool = setup_pool();

        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(String::from("hello"))?;
            vec.try_push(String::from("world"))?;
            vec.try_push(String::from("test"))?;

            // Drop happens here - should not leak strings
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_large_type() {
        #[derive(Clone, Copy)]
        struct Large([u64; 8]); // 64 bytes - smaller to fit in block

        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(Large([1; 8]))?;
            vec.try_push(Large([2; 8]))?;
            vec.try_push(Large([3; 8]))?;

            assert_eq!(vec.len(), 3);
            assert_eq!(vec.get(0).unwrap().0[0], 1);
            assert_eq!(vec.get(1).unwrap().0[0], 2);
            assert_eq!(vec.get(2).unwrap().0[0], 3);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_iterator_exactsize() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;

            let mut iter = vec.iter();
            assert_eq!(iter.len(), 3);

            iter.next();
            assert_eq!(iter.len(), 2);

            iter.next();
            assert_eq!(iter.len(), 1);

            iter.next();
            assert_eq!(iter.len(), 0);

            assert_eq!(iter.next(), None);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_iterator_double_ended() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;
            vec.try_push(4)?;

            let mut iter = vec.iter();

            // Alternate between next and next_back
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next_back(), Some(&4));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next_back(), Some(&3));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next_back(), None);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_iterator_empty() {
        let pool = setup_pool();
        pool.run(|| {
            let vec = LpVec::<i32>::new();
            let mut iter = vec.iter();

            assert_eq!(iter.len(), 0);
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next_back(), None);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_iterator_single_element() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(42)?;

            let mut iter = vec.iter();
            assert_eq!(iter.len(), 1);
            assert_eq!(iter.next(), Some(&42));
            assert_eq!(iter.len(), 0);
            assert_eq!(iter.next(), None);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_multiple_growth_cycles() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();

            // Capacity grows: 0 -> 4 -> 8 -> 16 -> 32
            for i in 0..50 {
                vec.try_push(i)?;
            }

            assert_eq!(vec.len(), 50);

            // Verify all values
            for i in 0..50 {
                assert_eq!(vec.get(i), Some(&i));
            }

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_memory_reuse() {
        let pool = setup_pool();
        let before = pool.used_bytes().unwrap();

        pool.run(|| {
            {
                let mut vec = LpVec::new();
                vec.try_push(1)?;
                vec.try_push(2)?;
                vec.try_push(3)?;
                // Vec dropped here
            }

            let after = pool.used_bytes().unwrap();
            assert_eq!(after, before, "Memory should be freed after vec drop");

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_as_raw_slice() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(10)?;
            vec.try_push(20)?;
            vec.try_push(30)?;

            let slice = vec.as_raw_slice();
            assert_eq!(slice.len(), 3);
            assert_eq!(slice[0], 10);
            assert_eq!(slice[1], 20);
            assert_eq!(slice[2], 30);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}
