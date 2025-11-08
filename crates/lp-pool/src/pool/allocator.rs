use core::ptr::{null_mut, NonNull};

use crate::block_header::{BlockHeader, MIN_BLOCK_SIZE};
use crate::error::AllocError;

/// Pool allocator with variable-size blocks and free list management
pub struct LpAllocator {
    pub(crate) memory: NonNull<u8>,
    pub(crate) capacity: usize,
    pub(crate) free_list: *mut u8, // Head of free list (null if empty)
    pub(crate) used_bytes: usize,  // Track actual bytes used instead of blocks
}

unsafe impl Send for LpAllocator {}
unsafe impl Sync for LpAllocator {}

impl LpAllocator {
    /// Create a new pool allocator with the given memory region
    ///
    /// # Safety
    /// - `memory` must point to a valid memory region of at least `size` bytes
    /// - `size` must be large enough for at least one minimal block (header + free pointer)
    /// - Memory must remain valid for the lifetime of the allocator
    pub unsafe fn new(memory: NonNull<u8>, size: usize) -> Result<Self, AllocError> {
        if size < MIN_BLOCK_SIZE {
            return Err(AllocError::InvalidLayout);
        }

        let base_ptr = memory.as_ptr();

        // Initialize the entire memory region as one large free block
        let initial_header = BlockHeader::new(size, false);
        BlockHeader::write(base_ptr, initial_header);
        BlockHeader::write_next_free(base_ptr, null_mut());

        Ok(LpAllocator {
            memory,
            capacity: size,
            free_list: base_ptr,
            used_bytes: 0,
        })
    }

    /// Create a pool allocator from a memory region
    ///
    /// This is equivalent to `new()` and exists for API consistency.
    ///
    /// # Safety
    /// Same as `new()` - memory must be valid and large enough
    pub unsafe fn from_region(memory: NonNull<u8>, size: usize) -> Result<Self, AllocError> {
        Self::new(memory, size)
    }

    /// Reset the pool (free all blocks)
    pub fn reset(&mut self) {
        unsafe {
            let base_ptr = self.memory.as_ptr();

            // Initialize the entire memory region as one large free block
            let initial_header = BlockHeader::new(self.capacity, false);
            BlockHeader::write(base_ptr, initial_header);
            BlockHeader::write_next_free(base_ptr, null_mut());

            self.free_list = base_ptr;
            self.used_bytes = 0;
        }
    }

    /// Get a pointer to the underlying memory (for sub-region allocation)
    pub fn memory_ptr(&self) -> NonNull<u8> {
        self.memory
    }

    /// Get the size of the memory region
    pub fn size(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use super::*;

    #[test]
    fn test_pool_creation() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            assert_eq!(pool.capacity(), 1024);
            assert_eq!(pool.used_bytes(), 0);
        }
    }

    #[test]
    fn test_reset() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let layout = core::alloc::Layout::from_size_align(32, 8).unwrap();

            let _ptr1 = pool.allocate(layout).unwrap();
            let _ptr2 = pool.allocate(layout).unwrap();
            assert_eq!(pool.used_blocks(), 2);

            pool.reset();
            assert_eq!(pool.used_blocks(), 0);

            // Should be able to allocate again
            let _ptr3 = pool.allocate(layout).unwrap();
            assert_eq!(pool.used_blocks(), 1);
        }
    }
}
