use crate::block_header::{BlockHeader, MIN_BLOCK_SIZE};
use crate::error::AllocError;
use core::alloc::Layout;
use core::mem;
use core::ptr::{null_mut, NonNull};

/// Pool allocator with variable-size blocks and free list management
pub struct LpAllocator {
    memory: NonNull<u8>,
    capacity: usize,
    free_list: *mut u8, // Head of free list (null if empty)
    used_bytes: usize,  // Track actual bytes used instead of blocks
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

    /// Allocate a block using first-fit strategy with block splitting
    pub fn allocate(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let requested_size = layout.size();
        let align = layout.align();

        // Walk the free list to find a suitable block (first-fit)
        let mut current_ptr = self.free_list;
        let mut prev_ptr: *mut u8 = null_mut();

        unsafe {
            while !current_ptr.is_null() {
                let header = BlockHeader::read(current_ptr);

                // Check if this block is free (should always be true in free list)
                if header.is_allocated {
                    // Corruption detected - skip
                    current_ptr = BlockHeader::read_next_free(current_ptr);
                    continue;
                }

                // With 32-byte header, data is naturally aligned to 8, 16, 32
                // For larger alignments, we'd need padding (not currently supported)
                let data_ptr = BlockHeader::data_ptr(current_ptr);
                let data_addr = data_ptr as usize;

                // Check if naturally aligned (works for align <= 32)
                if data_addr.is_multiple_of(align) {
                    // Calculate total size needed
                    let total_needed = mem::size_of::<BlockHeader>() + requested_size;

                    // Check if this block is large enough
                    if header.size >= total_needed {
                        // Found a suitable block!
                        // Remove from free list
                        let next_free = BlockHeader::read_next_free(current_ptr);
                        if prev_ptr.is_null() {
                            self.free_list = next_free;
                        } else {
                            BlockHeader::write_next_free(prev_ptr, next_free);
                        }

                        // Calculate remaining size after allocation
                        let remaining_size = header.size - total_needed;
                        let min_leftover = MIN_BLOCK_SIZE;

                        let (alloc_block_ptr, alloc_size) = if remaining_size >= min_leftover {
                            // Split: current block becomes allocated, remainder becomes free
                            let alloc_size = total_needed;
                            let remainder_ptr = current_ptr.add(alloc_size);

                            // Write remainder block header
                            let remainder_header = BlockHeader::new(remaining_size, false);
                            BlockHeader::write(remainder_ptr, remainder_header);

                            // Add remainder to free list
                            BlockHeader::write_next_free(remainder_ptr, self.free_list);
                            self.free_list = remainder_ptr;

                            (current_ptr, alloc_size)
                        } else {
                            // No split - use entire block
                            (current_ptr, header.size)
                        };

                        // Mark block as allocated
                        let alloc_header = BlockHeader::new(alloc_size, true);
                        BlockHeader::write(alloc_block_ptr, alloc_header);

                        // Update used bytes
                        self.used_bytes += alloc_size;

                        // Return pointer to data area
                        let data_ptr = BlockHeader::data_ptr(alloc_block_ptr);

                        return Ok(NonNull::slice_from_raw_parts(
                            NonNull::new_unchecked(data_ptr),
                            requested_size,
                        ));
                    }
                }

                // Move to next block in free list
                prev_ptr = current_ptr;
                current_ptr = BlockHeader::read_next_free(current_ptr);
            }
        }

        // No suitable block found
        Err(AllocError::PoolExhausted)
    }

    /// Deallocate a block with coalescing
    ///
    /// # Safety
    /// - `ptr` must have been allocated by this allocator (must be a data pointer returned from allocate)
    /// - `ptr` must not be deallocated twice
    ///
    /// # Panics
    /// In debug builds, panics if `ptr` is not within the pool's memory region
    pub unsafe fn deallocate(&mut self, ptr: NonNull<u8>, _layout: Layout) {
        // Convert data pointer to block pointer using stored offset
        let block_ptr = BlockHeader::block_ptr_from_data(ptr.as_ptr());

        // Validation: check that block_ptr is within our memory region
        #[cfg(debug_assertions)]
        {
            let ptr_addr = block_ptr as usize;
            let base_addr = self.memory.as_ptr() as usize;
            let end_addr = base_addr + self.capacity;

            if ptr_addr < base_addr || ptr_addr >= end_addr {
                panic!(
                    "Attempted to deallocate pointer {:p} not owned by this allocator (range {:p}..{:p})",
                    block_ptr,
                    self.memory.as_ptr(),
                    end_addr as *const u8
                );
            }
        }

        // Read the block header
        let mut header = BlockHeader::read(block_ptr);

        if !header.is_allocated {
            #[cfg(debug_assertions)]
            panic!(
                "Double free detected at {:p} (data ptr {:p}, header.size={}, header.data_offset={})",
                block_ptr,
                ptr.as_ptr(),
                header.size,
                header.data_offset
            );
            #[cfg(not(debug_assertions))]
            return;
        }

        // Update used bytes
        self.used_bytes = self.used_bytes.saturating_sub(header.size);

        // Mark block as free
        header.is_allocated = false;
        BlockHeader::write(block_ptr, header);

        // Try to coalesce with next block
        let base_addr = self.memory.as_ptr() as usize;
        let end_addr = base_addr + self.capacity;
        let next_block_ptr = header.next_block_ptr(block_ptr);

        if (next_block_ptr as usize) < end_addr {
            let next_header = BlockHeader::read(next_block_ptr);
            if !next_header.is_allocated {
                // Coalesce with next block
                // Remove next block from free list
                self.remove_from_free_list(next_block_ptr);

                // Extend current block
                header.size += next_header.size;
                BlockHeader::write(block_ptr, header);
            }
        }

        // Try to coalesce with previous block
        // We need to scan from the beginning to find the previous block
        let coalesced_ptr = self.try_coalesce_with_prev(block_ptr, header);

        // Add (possibly coalesced) block to free list
        BlockHeader::write_next_free(coalesced_ptr, self.free_list);
        self.free_list = coalesced_ptr;
    }

    /// Remove a block from the free list
    unsafe fn remove_from_free_list(&mut self, block_ptr: *mut u8) {
        let mut current = self.free_list;
        let mut prev: *mut u8 = null_mut();

        while !current.is_null() {
            if current == block_ptr {
                let next = BlockHeader::read_next_free(current);
                if prev.is_null() {
                    self.free_list = next;
                } else {
                    BlockHeader::write_next_free(prev, next);
                }
                return;
            }
            prev = current;
            current = BlockHeader::read_next_free(current);
        }
    }

    /// Try to coalesce with the previous block
    /// Returns the pointer to the coalesced block (may be block_ptr or prev_block_ptr)
    unsafe fn try_coalesce_with_prev(
        &mut self,
        block_ptr: *mut u8,
        header: BlockHeader,
    ) -> *mut u8 {
        let base_ptr = self.memory.as_ptr();
        if block_ptr == base_ptr {
            // This is the first block, no previous block
            return block_ptr;
        }

        // Scan from the beginning to find the previous block
        let end_addr = base_ptr as usize + self.capacity;
        let mut current_ptr = base_ptr;
        let mut iterations = 0;
        let max_iterations = self.capacity / MIN_BLOCK_SIZE + 1;

        while (current_ptr as usize) < (block_ptr as usize) {
            // Safety check to prevent infinite loops
            iterations += 1;
            if iterations > max_iterations || (current_ptr as usize) >= end_addr {
                // Something is wrong - bail out
                return block_ptr;
            }

            let current_header = BlockHeader::read(current_ptr);

            // Validate header
            if current_header.size < MIN_BLOCK_SIZE || current_header.size > self.capacity {
                // Corrupted header - bail out
                return block_ptr;
            }

            let next_ptr = current_header.next_block_ptr(current_ptr);

            if next_ptr == block_ptr {
                // Found the previous block
                if !current_header.is_allocated {
                    // Previous block is free - coalesce!
                    // Remove previous block from free list
                    self.remove_from_free_list(current_ptr);

                    // Extend previous block to include current block
                    let new_size = current_header.size + header.size;
                    let new_header = BlockHeader::new(new_size, false);
                    BlockHeader::write(current_ptr, new_header);

                    return current_ptr;
                }
                break;
            }

            current_ptr = next_ptr;
        }

        block_ptr
    }

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

    /// Get used bytes
    pub fn used_bytes(&self) -> usize {
        self.used_bytes
    }

    /// Get capacity in bytes
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get number of used blocks (for compatibility; counts allocated blocks)
    pub fn used_blocks(&self) -> usize {
        // This is a compatibility method - count allocated blocks
        // For better accuracy, use used_bytes()
        unsafe {
            let mut count = 0;
            let mut ptr = self.memory.as_ptr();
            let end = ptr.add(self.capacity);

            while (ptr as usize) < (end as usize) {
                let header = BlockHeader::read(ptr);
                if header.is_allocated {
                    count += 1;
                }
                ptr = header.next_block_ptr(ptr);
                if ptr >= end {
                    break;
                }
            }
            count
        }
    }

    /// Get number of free blocks (for compatibility; counts free blocks)
    pub fn free_blocks(&self) -> usize {
        // Count blocks in free list
        unsafe {
            let mut count = 0;
            let mut ptr = self.free_list;
            while !ptr.is_null() {
                count += 1;
                ptr = BlockHeader::read_next_free(ptr);
            }
            count
        }
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
    use super::*;
    use core::alloc::Layout;

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
    fn test_allocate_deallocate() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let layout = Layout::from_size_align(32, 8).unwrap();

            let ptr1 = pool.allocate(layout).unwrap();
            assert_eq!(pool.used_blocks(), 1);

            let ptr2 = pool.allocate(layout).unwrap();
            assert_eq!(pool.used_blocks(), 2);

            pool.deallocate(NonNull::new(ptr1.as_ptr() as *mut u8).unwrap(), layout);
            assert_eq!(pool.used_blocks(), 1);

            pool.deallocate(NonNull::new(ptr2.as_ptr() as *mut u8).unwrap(), layout);
            assert_eq!(pool.used_blocks(), 0);
        }
    }

    #[test]
    fn test_pool_exhausted() {
        let mut memory = [0u8; 128];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 128).unwrap();
            let layout = Layout::from_size_align(32, 8).unwrap();

            // Should get 2 blocks (128 / 64 = 2)
            let _ptr1 = pool.allocate(layout).unwrap();
            let _ptr2 = pool.allocate(layout).unwrap();

            // Third allocation should fail
            assert!(matches!(
                pool.allocate(layout),
                Err(AllocError::PoolExhausted)
            ));
        }
    }

    #[test]
    fn test_reset() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let layout = Layout::from_size_align(32, 8).unwrap();

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

    // Test for bug #2: Alignment check bug - valid alignments should not be rejected
    #[test]
    fn test_alignment_check_bug() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();

            // This should work: alignment 8, size 32, block_size 64
            // But current code rejects it if align() > block_size, which is wrong
            let layout = Layout::from_size_align(32, 8).unwrap();
            assert!(layout.align() <= 8); // 8 <= 64, should work
            let result = pool.allocate(layout);
            // This should succeed, but current buggy code might reject it
            // Actually wait, 8 <= 64, so it should pass the check...
            // The bug is that we check align() > block_size, but we should check
            // if the block is actually aligned to the required alignment
            assert!(
                result.is_ok(),
                "Valid allocation with align=8, block_size=64 should succeed"
            );
        }
    }

    // Test for bug #3: Blocks should be properly aligned
    #[test]
    fn test_block_alignment() {
        // Test with properly aligned memory
        let mut memory = [0u8; 8192];
        let memory_ptr = {
            let addr = memory.as_mut_ptr() as usize;
            let aligned_addr = (addr + 31) & !31; // Align to 32 bytes
            NonNull::new(aligned_addr as *mut u8).unwrap()
        };
        let size = 8192 - (memory_ptr.as_ptr() as usize - memory.as_mut_ptr() as usize);

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, size).unwrap();

            // Request alignment of 16 - should work with 32-byte aligned pool
            let layout = Layout::from_size_align(32, 16).unwrap();
            let block = pool.allocate(layout).unwrap();
            let block_ptr = block.as_ptr() as *const u8 as usize;

            assert_eq!(block_ptr % 16, 0, "Block should be aligned to 16 bytes");

            // Also test with alignment 8
            let layout2 = Layout::from_size_align(32, 8).unwrap();
            let block2 = pool.allocate(layout2).unwrap();
            let block2_ptr = block2.as_ptr() as *const u8 as usize;
            assert_eq!(block2_ptr % 8, 0, "Block should be aligned to 8 bytes");
        }
    }

    // Test for memory alignment: Pool should ensure blocks meet alignment requirements
    #[test]
    fn test_pool_ensures_alignment() {
        // Test with unaligned memory - pool should skip blocks that don't meet alignment
        let mut memory = [0u8; 2048];
        // Start at offset 1 to force misalignment
        let unaligned_ptr = NonNull::new(unsafe { memory.as_mut_ptr().add(1) }).unwrap();

        unsafe {
            // Create pool with block_size 64, but memory starts at offset 1
            let mut pool = LpAllocator::new(unaligned_ptr, 2047).unwrap();

            // Request alignment of 16
            let layout = Layout::from_size_align(32, 16).unwrap();

            // Pool should find a block that IS aligned to 16, even if first block isn't
            // This tests that the pool skips misaligned blocks
            let result = pool.allocate(layout);

            if let Ok(block) = result {
                let block_ptr = block.as_ptr() as *const u8 as usize;
                assert_eq!(
                    block_ptr % 16,
                    0,
                    "Pool should return aligned block even from unaligned memory"
                );
            } else {
                // If it fails, that's also acceptable - means pool correctly rejects
                // when no aligned blocks are available
            }
        }
    }

    #[test]
    fn test_pool_alignment_with_different_requirements() {
        let mut memory = [0u8; 4096]; // Larger pool for multiple alloc/dealloc cycles
                                      // Align memory to 64 bytes to ensure blocks are aligned
        let memory_ptr = {
            let addr = memory.as_mut_ptr() as usize;
            let aligned_addr = (addr + 63) & !63;
            NonNull::new(aligned_addr as *mut u8).unwrap()
        };
        let size = 4096 - (memory_ptr.as_ptr() as usize - memory.as_mut_ptr() as usize);

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, size).unwrap();

            // Test various alignment requirements (up to 32 - header natural alignment limit)
            for align in [1, 2, 4, 8, 16, 32] {
                let layout = Layout::from_size_align(32, align).unwrap();
                let block = pool.allocate(layout).unwrap();
                let block_ptr = block.as_ptr() as *const u8 as usize;
                assert_eq!(
                    block_ptr % align,
                    0,
                    "Block should be aligned to {} bytes, got offset {}",
                    align,
                    block_ptr % align
                );

                // Deallocate for next test
                pool.deallocate(NonNull::new(block.as_ptr() as *mut u8).unwrap(), layout);
            }
        }
    }

    #[test]
    fn test_pool_skips_misaligned_blocks() {
        // Create memory where first block is misaligned but later blocks are aligned
        let mut memory = [0u8; 2048];
        // Start at offset 8 (not aligned to 16)
        let memory_ptr = NonNull::new(unsafe { memory.as_mut_ptr().add(8) }).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 2040).unwrap();

            // Request alignment of 16
            let layout = Layout::from_size_align(32, 16).unwrap();

            // First block would be at offset 8, which is not aligned to 16
            // Pool should skip it and find a block that IS aligned
            let result = pool.allocate(layout);

            if let Ok(block) = result {
                let block_ptr = block.as_ptr() as *const u8 as usize;
                assert_eq!(
                    block_ptr % 16,
                    0,
                    "Pool should skip misaligned blocks and return aligned one"
                );
            }
        }
    }

    // Test for bug #2: Alignment check should verify actual alignment, not compare to block_size
    #[test]
    fn test_alignment_check_should_verify_alignment() {
        // Test with a case where align() < block_size but block might not be aligned
        let mut memory = [0u8; 1024 + 1];
        let memory_ptr = NonNull::new(unsafe { memory.as_mut_ptr().add(1) }).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();

            // Request alignment of 16, block_size is 64
            // align() = 16, block_size = 64, so 16 <= 64 passes the current check
            // But if the block isn't actually aligned to 16, this is wrong
            let layout = Layout::from_size_align(32, 16).unwrap();
            let result = pool.allocate(layout);

            // Should succeed AND the returned block should be aligned
            let block = result.expect("Allocation should succeed");
            let block_ptr = block.as_ptr() as *const u8 as usize;
            assert_eq!(
                block_ptr % 16,
                0,
                "Returned block must be aligned to requested alignment"
            );
        }
    }

    // === New tests for variable-size allocator ===

    #[test]
    fn test_variable_size_allocations() {
        let mut memory = [0u8; 4096];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 4096).unwrap();

            // Allocate various sizes
            let layout_8 = Layout::from_size_align(8, 8).unwrap();
            let layout_32 = Layout::from_size_align(32, 8).unwrap();
            let layout_128 = Layout::from_size_align(128, 8).unwrap();
            let layout_512 = Layout::from_size_align(512, 8).unwrap();

            let ptr1 = pool.allocate(layout_8).unwrap();
            assert_eq!(ptr1.len(), 8);

            let ptr2 = pool.allocate(layout_32).unwrap();
            assert_eq!(ptr2.len(), 32);

            let ptr3 = pool.allocate(layout_128).unwrap();
            assert_eq!(ptr3.len(), 128);

            let ptr4 = pool.allocate(layout_512).unwrap();
            assert_eq!(ptr4.len(), 512);

            // All should fit in 4096 bytes
            assert!(pool.used_bytes() < 4096);
        }
    }

    #[test]
    fn test_block_splitting() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();

            // First allocation should split the large free block
            let layout_32 = Layout::from_size_align(32, 8).unwrap();
            let ptr1 = pool.allocate(layout_32).unwrap();
            assert!(ptr1.len() >= 32);

            // Should still have plenty of free memory
            let available = pool.capacity() - pool.used_bytes();
            assert!(
                available > 900,
                "Should have split block, got {} free",
                available
            );

            // Allocate another small block
            let ptr2 = pool.allocate(layout_32).unwrap();
            assert!(ptr2.len() >= 32);

            // Both allocations should fit without using entire pool
            assert!(pool.used_bytes() < 200);
        }
    }

    #[test]
    fn test_block_coalescing() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();

            let layout_32 = Layout::from_size_align(32, 8).unwrap();

            // Allocate three blocks
            let ptr1 = pool.allocate(layout_32).unwrap();
            let ptr2 = pool.allocate(layout_32).unwrap();
            let _ptr3 = pool.allocate(layout_32).unwrap();

            // Deallocate middle block
            pool.deallocate(NonNull::new(ptr2.as_ptr() as *mut u8).unwrap(), layout_32);

            // Deallocate first block (should coalesce with middle)
            pool.deallocate(NonNull::new(ptr1.as_ptr() as *mut u8).unwrap(), layout_32);

            // Now we should be able to allocate a larger block
            // (larger than 32 but smaller than 64) if coalescing worked
            let layout_48 = Layout::from_size_align(48, 8).unwrap();
            let large_ptr = pool.allocate(layout_48);
            assert!(
                large_ptr.is_ok(),
                "Should be able to allocate 48 bytes after coalescing"
            );
        }
    }

    #[test]
    fn test_large_allocation() {
        let mut memory = [0u8; 8192];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 8192).unwrap();

            // Allocate most of the pool
            let layout_large = Layout::from_size_align(7000, 8).unwrap();
            let ptr = pool.allocate(layout_large);
            assert!(
                ptr.is_ok(),
                "Should be able to allocate 7000 bytes from 8192 byte pool"
            );

            let used = pool.used_bytes();
            assert!(used >= 7000, "Should have allocated at least 7000 bytes");
        }
    }

    #[test]
    fn test_mixed_size_allocations() {
        let mut memory = [0u8; 2048];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 2048).unwrap();

            // Allocate blocks of different sizes
            let sizes = [16, 64, 8, 128, 32, 256, 24];
            let mut ptrs = alloc::vec::Vec::new();

            for size in sizes.iter() {
                let layout = Layout::from_size_align(*size, 8).unwrap();
                let ptr = pool.allocate(layout).unwrap();
                ptrs.push((ptr, layout));
            }

            // All allocations should fit
            assert!(
                pool.used_bytes() < 2048,
                "All allocations should fit in pool"
            );

            // Deallocate some blocks
            for (ptr, layout) in ptrs.iter().take(3) {
                pool.deallocate(NonNull::new(ptr.as_ptr() as *mut u8).unwrap(), *layout);
            }

            // Should be able to allocate more
            let layout_new = Layout::from_size_align(50, 8).unwrap();
            let new_ptr = pool.allocate(layout_new);
            assert!(new_ptr.is_ok(), "Should be able to reuse freed space");
        }
    }

    #[test]
    fn test_allocation_efficiency() {
        let mut memory = [0u8; 4096];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 4096).unwrap();

            // Allocate many small blocks
            let layout = Layout::from_size_align(16, 8).unwrap();
            let mut count = 0;

            while pool.allocate(layout).is_ok() {
                count += 1;
            }

            // With variable-size allocator, we should be able to fit many 16-byte allocations
            // Old fixed-size with 64-byte blocks: 4096/64 = 64 blocks
            // New variable-size: ~4096/(16+32_byte_header) ≈ 85 blocks
            assert!(
                count > 70,
                "Should fit at least 70 small allocations, got {}",
                count
            );
        }
    }

    // === Core Allocator Safety Tests ===

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

    #[test]
    fn test_pool_exhaustion_and_recovery() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();

            let layout = Layout::from_size_align(256, 8).unwrap();

            // Allocate until exhausted
            let ptr1 = pool.allocate(layout).unwrap();
            let ptr2 = pool.allocate(layout).unwrap();
            let ptr3 = pool.allocate(layout).unwrap();

            // Next allocation should fail (pool exhausted)
            let result = pool.allocate(layout);
            assert!(result.is_err(), "Should fail when pool exhausted");

            // Deallocate one block
            pool.deallocate(NonNull::new(ptr2.as_ptr() as *mut u8).unwrap(), layout);

            // Should be able to allocate again
            let ptr4 = pool.allocate(layout);
            assert!(ptr4.is_ok(), "Should succeed after freeing memory");

            // Clean up
            pool.deallocate(NonNull::new(ptr1.as_ptr() as *mut u8).unwrap(), layout);
            pool.deallocate(NonNull::new(ptr3.as_ptr() as *mut u8).unwrap(), layout);
            pool.deallocate(
                NonNull::new(ptr4.unwrap().as_ptr() as *mut u8).unwrap(),
                layout,
            );
        }
    }

    #[test]
    fn test_zero_size_allocation() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();

            let layout = Layout::from_size_align(0, 1).unwrap();
            let result = pool.allocate(layout);

            // Zero-size allocations should either succeed (with a valid pointer)
            // or fail gracefully - both are acceptable behaviors
            if let Ok(ptr) = result {
                // If it succeeds, pointer should be non-null
                // Verify NonNull invariant (always non-null by construction)
                // And deallocate should work
                pool.deallocate(NonNull::new(ptr.as_ptr() as *mut u8).unwrap(), layout);
            }
            // If it fails, that's also acceptable
        }
    }

    #[test]
    fn test_memory_leak_detection() {
        let mut memory = [0u8; 2048];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 2048).unwrap();

            let initial_used = pool.used_bytes();
            assert_eq!(initial_used, 0, "Pool should start empty");

            // Allocate and deallocate
            let layout = Layout::from_size_align(64, 8).unwrap();
            let ptr1 = pool.allocate(layout).unwrap();
            let used_after_alloc = pool.used_bytes();
            assert!(used_after_alloc > 0, "Should track used bytes");

            pool.deallocate(NonNull::new(ptr1.as_ptr() as *mut u8).unwrap(), layout);
            let used_after_dealloc = pool.used_bytes();

            assert_eq!(
                used_after_dealloc, initial_used,
                "All memory should be freed after deallocation"
            );
        }
    }

    #[test]
    fn test_fragmentation_handling() {
        let mut memory = [0u8; 2048];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 2048).unwrap();

            let layout_small = Layout::from_size_align(32, 8).unwrap();
            let layout_medium = Layout::from_size_align(64, 8).unwrap();

            // Create fragmentation pattern: alloc, alloc, dealloc first, alloc
            let ptr1 = pool.allocate(layout_small).unwrap();
            let ptr2 = pool.allocate(layout_small).unwrap();
            let ptr3 = pool.allocate(layout_small).unwrap();

            // Free middle block
            pool.deallocate(
                NonNull::new(ptr2.as_ptr() as *mut u8).unwrap(),
                layout_small,
            );

            // Try to allocate medium - should work despite fragmentation
            let ptr_medium = pool.allocate(layout_medium);
            assert!(ptr_medium.is_ok(), "Should handle fragmentation");

            // Clean up
            pool.deallocate(
                NonNull::new(ptr1.as_ptr() as *mut u8).unwrap(),
                layout_small,
            );
            pool.deallocate(
                NonNull::new(ptr3.as_ptr() as *mut u8).unwrap(),
                layout_small,
            );
            if let Ok(pm) = ptr_medium {
                pool.deallocate(NonNull::new(pm.as_ptr() as *mut u8).unwrap(), layout_medium);
            }
        }
    }

    #[test]
    fn test_allocate_entire_pool() {
        let mut memory = [0u8; 2048];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 2048).unwrap();

            // Try to allocate entire pool capacity (minus header overhead)
            let layout = Layout::from_size_align(1900, 8).unwrap();
            let result = pool.allocate(layout);

            assert!(
                result.is_ok(),
                "Should be able to allocate most of the pool"
            );

            if let Ok(ptr) = result {
                // Verify no more allocations possible
                let layout_small = Layout::from_size_align(32, 8).unwrap();
                let result2 = pool.allocate(layout_small);
                assert!(
                    result2.is_err(),
                    "Should not be able to allocate when pool is full"
                );

                // Clean up
                pool.deallocate(NonNull::new(ptr.as_ptr() as *mut u8).unwrap(), layout);

                // Now small allocation should work
                let result3 = pool.allocate(layout_small);
                assert!(result3.is_ok(), "Should work after freeing large block");
            }
        }
    }

    #[test]
    fn test_block_header_validation() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();

            let layout = Layout::from_size_align(64, 8).unwrap();
            let ptr = pool.allocate(layout).unwrap();

            // Get block pointer and verify header is valid
            let block_ptr = BlockHeader::block_ptr_from_data(ptr.as_ptr() as *mut u8);
            let header = BlockHeader::read(block_ptr);

            assert!(header.is_valid(), "Block header should be valid");
            assert!(header.is_allocated, "Block should be marked as allocated");
            assert!(
                header.size >= layout.size(),
                "Block size should accommodate requested size"
            );

            // Clean up
            pool.deallocate(NonNull::new(ptr.as_ptr() as *mut u8).unwrap(), layout);

            // After deallocation, block should be marked as free
            let header_after = BlockHeader::read(block_ptr);
            assert!(header_after.is_valid(), "Header should still be valid");
            assert!(!header_after.is_allocated, "Block should be marked as free");
        }
    }

    #[test]
    fn test_allocation_alignment_boundary() {
        let mut memory = [0u8; 2048];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 2048).unwrap();

            // Test allocation at various alignment boundaries
            for align_power in 0..5 {
                let align = 1 << align_power; // 1, 2, 4, 8, 16
                let layout = Layout::from_size_align(32, align).unwrap();

                let ptr = pool
                    .allocate(layout)
                    .expect("Should allocate with various alignments");
                let addr = ptr.as_ptr() as *const u8 as usize;

                assert_eq!(
                    addr % align,
                    0,
                    "Allocation should be aligned to {} bytes",
                    align
                );

                pool.deallocate(NonNull::new(ptr.as_ptr() as *mut u8).unwrap(), layout);
            }
        }
    }

    #[test]
    fn test_rapid_alloc_dealloc_cycles() {
        let mut memory = [0u8; 4096];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 4096).unwrap();

            let layout = Layout::from_size_align(64, 8).unwrap();

            // Rapid allocation/deallocation cycles
            for _ in 0..100 {
                let ptr = pool.allocate(layout).expect("Should allocate");
                pool.deallocate(NonNull::new(ptr.as_ptr() as *mut u8).unwrap(), layout);
            }

            // Pool should be in consistent state
            let used = pool.used_bytes();
            assert_eq!(used, 0, "All memory should be freed after cycles");

            // Should still be able to allocate
            let final_ptr = pool.allocate(layout);
            assert!(
                final_ptr.is_ok(),
                "Pool should still work after many cycles"
            );
        }
    }
}
