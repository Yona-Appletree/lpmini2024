use core::alloc::Layout;
use core::ptr::NonNull;

use crate::error::AllocError;

use super::allocator::LpAllocator;

impl LpAllocator {
    /// Grow an allocation to a new size
    ///
    /// Allocates a new block, copies data from the old block, and deallocates the old block.
    ///
    /// # Safety
    /// - `ptr` must have been allocated by this allocator (data pointer)
    /// - `old_layout` must match the layout used to allocate `ptr`
    /// - `new_size` must be greater than `old_layout.size()`
    pub unsafe fn grow(
        &mut self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_size: usize,
    ) -> Result<NonNull<[u8]>, AllocError> {
        if new_size <= old_layout.size() {
            return Err(AllocError::InvalidLayout);
        }

        // Allocate new block
        let new_layout = Layout::from_size_align(new_size, old_layout.align())
            .map_err(|_| AllocError::InvalidLayout)?;
        let new_block = self.allocate(new_layout)?;
        let new_ptr = NonNull::new(new_block.as_ptr() as *mut u8).unwrap();

        // Copy data from old block to new block
        let copy_size = old_layout.size().min(new_size);
        core::ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr(), copy_size);

        // Deallocate old block
        self.deallocate(ptr, old_layout);

        Ok(new_block)
    }

    /// Shrink an allocation to a new size
    ///
    /// Allocates a new block, copies data from the old block, and deallocates the old block.
    ///
    /// # Safety
    /// - `ptr` must have been allocated by this allocator (data pointer)
    /// - `old_layout` must match the layout used to allocate `ptr`
    /// - `new_size` must be less than `old_layout.size()`
    pub unsafe fn shrink(
        &mut self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_size: usize,
    ) -> Result<NonNull<[u8]>, AllocError> {
        if new_size >= old_layout.size() {
            return Err(AllocError::InvalidLayout);
        }

        // Allocate new block
        let new_layout = Layout::from_size_align(new_size, old_layout.align())
            .map_err(|_| AllocError::InvalidLayout)?;
        let new_block = self.allocate(new_layout)?;
        let new_ptr = NonNull::new(new_block.as_ptr() as *mut u8).unwrap();

        // Copy data from old block to new block (only new_size bytes)
        core::ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr(), new_size);

        // Deallocate old block
        self.deallocate(ptr, old_layout);

        Ok(new_block)
    }
}

#[cfg(test)]
mod tests {
    use core::alloc::Layout;
    use core::ptr::NonNull;

    use super::*;

    #[test]
    fn test_grow() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let old_layout = Layout::from_size_align(32, 8).unwrap();

            // Allocate initial block
            let old_block = pool.allocate(old_layout).unwrap();
            let old_ptr = NonNull::new(old_block.as_ptr() as *mut u8).unwrap();

            // Write some data
            let data: [u8; 32] = [0x42; 32];
            core::ptr::copy_nonoverlapping(data.as_ptr(), old_ptr.as_ptr(), 32);

            // Grow to larger size
            let new_size = 48;
            let new_block = pool.grow(old_ptr, old_layout, new_size).unwrap();
            let new_ptr = NonNull::new(new_block.as_ptr() as *mut u8).unwrap();

            // Verify data was copied
            for i in 0..32 {
                assert_eq!(*new_ptr.as_ptr().add(i), 0x42);
            }

            // Old block should be deallocated
            assert_eq!(pool.used_blocks(), 1);
        }
    }

    #[test]
    fn test_grow_same_block_size() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let old_layout = Layout::from_size_align(32, 8).unwrap();

            let old_block = pool.allocate(old_layout).unwrap();
            let old_ptr = NonNull::new(old_block.as_ptr() as *mut u8).unwrap();

            // Grow to block_size (64)
            let new_block = pool.grow(old_ptr, old_layout, 64).unwrap();
            assert_eq!(pool.used_blocks(), 1);
            assert_eq!(new_block.len(), 64);
        }
    }

    #[test]
    fn test_grow_multiple_times() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let mut layout = Layout::from_size_align(16, 8).unwrap();

            let mut block = pool.allocate(layout).unwrap();
            let mut ptr = NonNull::new(block.as_ptr() as *mut u8).unwrap();

            // Grow multiple times
            block = pool.grow(ptr, layout, 32).unwrap();
            ptr = NonNull::new(block.as_ptr() as *mut u8).unwrap();
            layout = Layout::from_size_align(32, 8).unwrap();

            block = pool.grow(ptr, layout, 48).unwrap();
            ptr = NonNull::new(block.as_ptr() as *mut u8).unwrap();
            layout = Layout::from_size_align(48, 8).unwrap();

            block = pool.grow(ptr, layout, 64).unwrap();
            assert_eq!(block.len(), 64);
            assert_eq!(pool.used_blocks(), 1);
        }
    }

    #[test]
    fn test_grow_error_cases() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let layout = Layout::from_size_align(32, 8).unwrap();

            let block = pool.allocate(layout).unwrap();
            let ptr = NonNull::new(block.as_ptr() as *mut u8).unwrap();

            // Grow to same size should fail
            assert!(matches!(
                pool.grow(ptr, layout, 32),
                Err(AllocError::InvalidLayout)
            ));

            // Grow to smaller size should fail
            assert!(matches!(
                pool.grow(ptr, layout, 16),
                Err(AllocError::InvalidLayout)
            ));

            // Grow beyond pool capacity should fail
            assert!(matches!(
                pool.grow(ptr, layout, 2000),
                Err(AllocError::PoolExhausted)
            ));
        }
    }

    #[test]
    fn test_shrink() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let old_layout = Layout::from_size_align(48, 8).unwrap();

            // Allocate initial block
            let old_block = pool.allocate(old_layout).unwrap();
            let old_ptr = NonNull::new(old_block.as_ptr() as *mut u8).unwrap();

            // Write some data
            let data: [u8; 48] = [0x42; 48];
            core::ptr::copy_nonoverlapping(data.as_ptr(), old_ptr.as_ptr(), 48);

            // Shrink to smaller size
            let new_size = 32;
            let new_block = pool.shrink(old_ptr, old_layout, new_size).unwrap();
            let new_ptr = NonNull::new(new_block.as_ptr() as *mut u8).unwrap();

            // Verify data was copied (only first 32 bytes)
            for i in 0..32 {
                assert_eq!(*new_ptr.as_ptr().add(i), 0x42);
            }

            // Old block should be deallocated
            assert_eq!(pool.used_blocks(), 1);
        }
    }

    #[test]
    fn test_shrink_multiple_times() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let mut layout = Layout::from_size_align(64, 8).unwrap();

            let mut block = pool.allocate(layout).unwrap();
            let mut ptr = NonNull::new(block.as_ptr() as *mut u8).unwrap();

            // Shrink multiple times
            block = pool.shrink(ptr, layout, 48).unwrap();
            ptr = NonNull::new(block.as_ptr() as *mut u8).unwrap();
            layout = Layout::from_size_align(48, 8).unwrap();

            block = pool.shrink(ptr, layout, 32).unwrap();
            ptr = NonNull::new(block.as_ptr() as *mut u8).unwrap();
            layout = Layout::from_size_align(32, 8).unwrap();

            block = pool.shrink(ptr, layout, 16).unwrap();
            // With variable-size allocator, returned slice has requested size
            assert_eq!(block.len(), 16); // requested size
            assert_eq!(pool.used_blocks(), 1);
        }
    }

    #[test]
    fn test_shrink_error_cases() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let layout = Layout::from_size_align(32, 8).unwrap();

            let block = pool.allocate(layout).unwrap();
            let ptr = NonNull::new(block.as_ptr() as *mut u8).unwrap();

            // Shrink to same size should fail
            assert!(matches!(
                pool.shrink(ptr, layout, 32),
                Err(AllocError::InvalidLayout)
            ));

            // Shrink to larger size should fail
            assert!(matches!(
                pool.shrink(ptr, layout, 48),
                Err(AllocError::InvalidLayout)
            ));
        }
    }

    #[test]
    fn test_grow_then_shrink() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let mut layout = Layout::from_size_align(16, 8).unwrap();

            let mut block = pool.allocate(layout).unwrap();
            let mut ptr = NonNull::new(block.as_ptr() as *mut u8).unwrap();

            // Grow
            block = pool.grow(ptr, layout, 48).unwrap();
            ptr = NonNull::new(block.as_ptr() as *mut u8).unwrap();
            layout = Layout::from_size_align(48, 8).unwrap();

            // Shrink back
            block = pool.shrink(ptr, layout, 32).unwrap();
            ptr = NonNull::new(block.as_ptr() as *mut u8).unwrap();
            layout = Layout::from_size_align(32, 8).unwrap();

            // Grow again
            block = pool.grow(ptr, layout, 64).unwrap();
            assert_eq!(block.len(), 64);
            assert_eq!(pool.used_blocks(), 1);
        }
    }

    #[test]
    fn test_grow_pool_exhausted() {
        let mut memory = [0u8; 1024]; // Larger pool for 32-byte headers
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let layout = Layout::from_size_align(32, 8).unwrap();

            // Fill most of the pool
            let _block1 = pool
                .allocate(Layout::from_size_align(400, 8).unwrap())
                .unwrap();
            let _block2 = pool
                .allocate(Layout::from_size_align(200, 8).unwrap())
                .unwrap();

            // Allocate a small block
            let block3 = pool.allocate(layout).unwrap();
            let ptr3 = NonNull::new(block3.as_ptr() as *mut u8).unwrap();

            // Try to grow to 600 bytes - should fail (not enough space)
            assert!(matches!(
                pool.grow(ptr3, layout, 600),
                Err(AllocError::PoolExhausted)
            ));
        }
    }

    #[test]
    fn test_data_preservation_during_grow() {
        let mut memory = [0u8; 4096];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 4096).unwrap();

            let old_layout = Layout::from_size_align(32, 8).unwrap();
            let ptr = pool.allocate(old_layout).unwrap();
            let data_ptr = NonNull::new(ptr.as_ptr() as *mut u8).unwrap();

            // Write test data
            let test_data: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
            core::ptr::copy_nonoverlapping(test_data.as_ptr(), data_ptr.as_ptr(), test_data.len());

            // Grow allocation
            let grown = pool.grow(data_ptr, old_layout, 64).unwrap();
            let grown_ptr = grown.as_ptr() as *const u8;

            // Verify data was preserved
            for (i, &expected_val) in test_data.iter().enumerate() {
                assert_eq!(
                    *grown_ptr.add(i),
                    expected_val,
                    "Data corrupted at byte {}",
                    i
                );
            }

            // Clean up
            let new_layout = Layout::from_size_align(64, 8).unwrap();
            pool.deallocate(NonNull::new(grown_ptr as *mut u8).unwrap(), new_layout);
        }
    }

    #[test]
    fn test_data_preservation_during_shrink() {
        let mut memory = [0u8; 4096];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 4096).unwrap();

            let old_layout = Layout::from_size_align(128, 8).unwrap();
            let ptr = pool.allocate(old_layout).unwrap();
            let data_ptr = NonNull::new(ptr.as_ptr() as *mut u8).unwrap();

            // Write test data (more than new size to test partial copy)
            let test_data: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            core::ptr::copy_nonoverlapping(test_data.as_ptr(), data_ptr.as_ptr(), test_data.len());

            // Shrink allocation
            let shrunk = pool.shrink(data_ptr, old_layout, 16).unwrap();
            let shrunk_ptr = shrunk.as_ptr() as *const u8;

            // Verify data was preserved (up to new size)
            for (i, &expected_val) in test_data.iter().enumerate().take(16) {
                assert_eq!(
                    *shrunk_ptr.add(i),
                    expected_val,
                    "Data corrupted at byte {}",
                    i
                );
            }

            // Clean up
            let new_layout = Layout::from_size_align(16, 8).unwrap();
            pool.deallocate(NonNull::new(shrunk_ptr as *mut u8).unwrap(), new_layout);
        }
    }

    #[test]
    fn test_invalid_grow_same_size() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();

            let layout = Layout::from_size_align(32, 8).unwrap();
            let ptr = pool.allocate(layout).unwrap();
            let data_ptr = NonNull::new(ptr.as_ptr() as *mut u8).unwrap();

            // Try to "grow" to same size
            let result = pool.grow(data_ptr, layout, 32);
            assert!(result.is_err(), "Growing to same size should fail");

            // Clean up
            pool.deallocate(data_ptr, layout);
        }
    }

    #[test]
    fn test_invalid_shrink_same_size() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();

            let layout = Layout::from_size_align(32, 8).unwrap();
            let ptr = pool.allocate(layout).unwrap();
            let data_ptr = NonNull::new(ptr.as_ptr() as *mut u8).unwrap();

            // Try to "shrink" to same size
            let result = pool.shrink(data_ptr, layout, 32);
            assert!(result.is_err(), "Shrinking to same size should fail");

            // Clean up
            pool.deallocate(data_ptr, layout);
        }
    }
}
