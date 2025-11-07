use core::alloc::Layout;
use core::ptr::{NonNull, null_mut};
use crate::error::AllocError;

/// Pool allocator with fixed-size blocks and free list management
pub struct PoolAllocator {
    memory: NonNull<u8>,
    block_size: usize,
    block_count: usize,
    free_list: *mut u8,  // Head of free list (null if empty)
    used_blocks: usize,
    capacity: usize,
}

unsafe impl Send for PoolAllocator {}
unsafe impl Sync for PoolAllocator {}

impl PoolAllocator {
    /// Create a new pool allocator with the given memory region
    ///
    /// # Safety
    /// - `memory` must point to a valid memory region of at least `size` bytes
    /// - `size` must be large enough for at least one block
    /// - `block_size` must be at least `size_of::<*mut u8>()` (for free list pointer)
    /// - `memory` should be aligned to `block_size` for optimal performance (blocks will be aligned to block_size boundaries)
    pub unsafe fn new(memory: NonNull<u8>, size: usize, block_size: usize) -> Result<Self, AllocError> {
        let ptr_size = core::mem::size_of::<*mut u8>();
        if block_size < ptr_size {
            return Err(AllocError::InvalidLayout);
        }
        
        if size < block_size {
            return Err(AllocError::InvalidLayout);
        }
        
        let block_count = size / block_size;
        if block_count == 0 {
            return Err(AllocError::InvalidLayout);
        }
        
        // Verify memory is aligned to block_size for proper block alignment
        let memory_addr = memory.as_ptr() as usize;
        if memory_addr % block_size != 0 {
            // Memory is not aligned - blocks won't be aligned either
            // This is a limitation: we can't guarantee alignment without aligned memory
            // For now, we'll allow it but document the limitation
        }
        
        // Initialize free list - each free block stores a pointer to the next free block
        let mut free_list: *mut u8 = null_mut();
        let base_ptr = memory.as_ptr();
        
        // Build free list in reverse order (so first allocation gets first block)
        for i in (0..block_count).rev() {
            let block_ptr = base_ptr.add(i * block_size);
            // Store pointer to next free block at start of this block
            let next_ptr = block_ptr as *mut *mut u8;
            core::ptr::write(next_ptr, free_list);
            free_list = block_ptr;
        }
        
        Ok(PoolAllocator {
            memory,
            block_size,
            block_count,
            free_list,
            used_blocks: 0,
            capacity: block_count * block_size,
        })
    }
    
    /// Create a pool allocator from a memory region (sub-region of parent)
    ///
    /// # Safety
    /// Same as `new()` - memory must be valid and large enough
    pub unsafe fn from_region(memory: NonNull<u8>, size: usize, block_size: usize) -> Result<Self, AllocError> {
        Self::new(memory, size, block_size)
    }
    
    /// Allocate a block
    pub fn allocate(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // Check if layout fits in a block
        if layout.size() > self.block_size {
            return Err(AllocError::OutOfMemory {
                requested: layout.size(),
                available: self.block_size,
            });
        }
        
        // Get block from free list
        if self.free_list.is_null() {
            return Err(AllocError::PoolExhausted);
        }
        
        // Remove from free list
        let block_ptr = self.free_list;
        // Read the next pointer from the block
        let next_ptr = block_ptr as *mut *mut u8;
        let next = unsafe { core::ptr::read(next_ptr) };
        self.free_list = next;
        self.used_blocks += 1;
        
        // Check if block is actually aligned to the requested alignment
        let block_addr = block_ptr as usize;
        if block_addr % layout.align() != 0 {
            // Block is not aligned - we can't satisfy this request
            // Put block back on free list
            unsafe {
                core::ptr::write(next_ptr, self.free_list);
            }
            self.free_list = block_ptr;
            self.used_blocks -= 1;
            return Err(AllocError::InvalidLayout);
        }
        
        // Return as NonNull<[u8]>
        Ok(unsafe {
            NonNull::slice_from_raw_parts(
                NonNull::new_unchecked(block_ptr),
                self.block_size,
            )
        })
    }
    
    /// Deallocate a block
    ///
    /// # Safety
    /// - `ptr` must have been allocated by this allocator
    /// - `ptr` must not be deallocated twice
    /// - `ptr` must point to the start of a block
    pub unsafe fn deallocate(&mut self, ptr: NonNull<u8>, _layout: Layout) {
        // Add back to free list
        let block_ptr = ptr.as_ptr();
        let next_ptr = block_ptr as *mut *mut u8;
        core::ptr::write(next_ptr, self.free_list);
        self.free_list = block_ptr;
        self.used_blocks -= 1;
    }
    
    /// Grow an allocation to a new size
    ///
    /// Allocates a new block, copies data from the old block, and deallocates the old block.
    /// The new size must fit within a single block.
    ///
    /// # Safety
    /// - `ptr` must have been allocated by this allocator
    /// - `ptr` must point to the start of a block
    /// - `old_layout` must match the layout used to allocate `ptr`
    /// - `new_size` must be greater than `old_layout.size()`
    /// - `new_size` must fit within `self.block_size`
    pub unsafe fn grow(
        &mut self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_size: usize,
    ) -> Result<NonNull<[u8]>, AllocError> {
        if new_size <= old_layout.size() {
            return Err(AllocError::InvalidLayout);
        }
        
        if new_size > self.block_size {
            return Err(AllocError::OutOfMemory {
                requested: new_size,
                available: self.block_size,
            });
        }
        
        // Allocate new block
        let new_layout = Layout::from_size_align(new_size, old_layout.align())
            .map_err(|_| AllocError::InvalidLayout)?;
        let new_block = self.allocate(new_layout)?;
        let new_ptr = NonNull::new(new_block.as_ptr() as *mut u8).unwrap();
        
        // Copy data from old block to new block
        let copy_size = old_layout.size().min(new_size);
        core::ptr::copy_nonoverlapping(
            ptr.as_ptr(),
            new_ptr.as_ptr(),
            copy_size,
        );
        
        // Deallocate old block
        self.deallocate(ptr, old_layout);
        
        Ok(new_block)
    }
    
    /// Shrink an allocation to a new size
    ///
    /// Allocates a new block, copies data from the old block, and deallocates the old block.
    /// The new size must fit within a single block.
    ///
    /// # Safety
    /// - `ptr` must have been allocated by this allocator
    /// - `ptr` must point to the start of a block
    /// - `old_layout` must match the layout used to allocate `ptr`
    /// - `new_size` must be less than `old_layout.size()`
    /// - `new_size` must fit within `self.block_size`
    pub unsafe fn shrink(
        &mut self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_size: usize,
    ) -> Result<NonNull<[u8]>, AllocError> {
        if new_size >= old_layout.size() {
            return Err(AllocError::InvalidLayout);
        }
        
        if new_size > self.block_size {
            return Err(AllocError::OutOfMemory {
                requested: new_size,
                available: self.block_size,
            });
        }
        
        // Allocate new block
        let new_layout = Layout::from_size_align(new_size, old_layout.align())
            .map_err(|_| AllocError::InvalidLayout)?;
        let new_block = self.allocate(new_layout)?;
        let new_ptr = NonNull::new(new_block.as_ptr() as *mut u8).unwrap();
        
        // Copy data from old block to new block (only new_size bytes)
        core::ptr::copy_nonoverlapping(
            ptr.as_ptr(),
            new_ptr.as_ptr(),
            new_size,
        );
        
        // Deallocate old block
        self.deallocate(ptr, old_layout);
        
        Ok(new_block)
    }
    
    /// Get used bytes
    pub fn used_bytes(&self) -> usize {
        self.used_blocks * self.block_size
    }
    
    /// Get capacity in bytes
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Get number of used blocks
    pub fn used_blocks(&self) -> usize {
        self.used_blocks
    }
    
    /// Get number of free blocks
    pub fn free_blocks(&self) -> usize {
        self.block_count - self.used_blocks
    }
    
    /// Reset the pool (free all blocks)
    pub fn reset(&mut self) {
        // Rebuild free list
        let base_ptr = self.memory.as_ptr();
        let mut free_list: *mut u8 = null_mut();
        
        // Build free list in reverse order
        for i in (0..self.block_count).rev() {
            let block_ptr = unsafe { base_ptr.add(i * self.block_size) };
            let next_ptr = block_ptr as *mut *mut u8;
            unsafe {
                core::ptr::write(next_ptr, free_list);
            }
            free_list = block_ptr;
        }
        
        self.free_list = free_list;
        self.used_blocks = 0;
    }
    
    /// Get a pointer to the underlying memory (for sub-region allocation)
    pub fn memory_ptr(&self) -> NonNull<u8> {
        self.memory
    }
    
    /// Get the size of the memory region
    pub fn size(&self) -> usize {
        self.block_count * self.block_size
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
            let pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
            assert_eq!(pool.capacity(), 1024);
            assert_eq!(pool.used_bytes(), 0);
        }
    }
    
    #[test]
    fn test_allocate_deallocate() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        
        unsafe {
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
            let layout = Layout::from_size_align(32, 8).unwrap();
            
            let ptr1 = pool.allocate(layout).unwrap();
            assert_eq!(pool.used_blocks(), 1);
            
            let ptr2 = pool.allocate(layout).unwrap();
            assert_eq!(pool.used_blocks(), 2);
            
            pool.deallocate(unsafe { NonNull::new(ptr1.as_ptr() as *mut u8).unwrap() }, layout);
            assert_eq!(pool.used_blocks(), 1);
            
            pool.deallocate(unsafe { NonNull::new(ptr2.as_ptr() as *mut u8).unwrap() }, layout);
            assert_eq!(pool.used_blocks(), 0);
        }
    }
    
    #[test]
    fn test_pool_exhausted() {
        let mut memory = [0u8; 128];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        
        unsafe {
            let mut pool = PoolAllocator::new(memory_ptr, 128, 64).unwrap();
            let layout = Layout::from_size_align(32, 8).unwrap();
            
            // Should get 2 blocks (128 / 64 = 2)
            let _ptr1 = pool.allocate(layout).unwrap();
            let _ptr2 = pool.allocate(layout).unwrap();
            
            // Third allocation should fail
            assert!(matches!(pool.allocate(layout), Err(AllocError::PoolExhausted)));
        }
    }
    
    #[test]
    fn test_reset() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        
        unsafe {
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
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
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
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
                assert_eq!(unsafe { *new_ptr.as_ptr().add(i) }, 0x42);
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
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
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
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
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
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
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
            
            // Grow beyond block size should fail
            assert!(matches!(
                pool.grow(ptr, layout, 128),
                Err(AllocError::OutOfMemory { .. })
            ));
        }
    }
    
    #[test]
    fn test_shrink() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        
        unsafe {
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
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
                assert_eq!(unsafe { *new_ptr.as_ptr().add(i) }, 0x42);
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
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
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
            // Block always has block_size length, but we shrunk to 16 bytes
            assert_eq!(block.len(), 64); // block_size
            assert_eq!(pool.used_blocks(), 1);
        }
    }
    
    #[test]
    fn test_shrink_error_cases() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        
        unsafe {
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
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
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
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
        let mut memory = [0u8; 256];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        
        unsafe {
            let mut pool = PoolAllocator::new(memory_ptr, 256, 64).unwrap();
            let layout = Layout::from_size_align(32, 8).unwrap();
            
            // Allocate all blocks except one
            let _block1 = pool.allocate(layout).unwrap();
            let _block2 = pool.allocate(layout).unwrap();
            let _block3 = pool.allocate(layout).unwrap();
            
            // Last block
            let block4 = pool.allocate(layout).unwrap();
            let ptr4 = NonNull::new(block4.as_ptr() as *mut u8).unwrap();
            
            // Grow should fail (no free blocks)
            assert!(matches!(
                pool.grow(ptr4, layout, 48),
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
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
            
            // This should work: alignment 8, size 32, block_size 64
            // But current code rejects it if align() > block_size, which is wrong
            let layout = Layout::from_size_align(32, 8).unwrap();
            assert!(layout.align() <= 8); // 8 <= 64, should work
            let result = pool.allocate(layout);
            // This should succeed, but current buggy code might reject it
            // Actually wait, 8 <= 64, so it should pass the check...
            // The bug is that we check align() > block_size, but we should check
            // if the block is actually aligned to the required alignment
            assert!(result.is_ok(), "Valid allocation with align=8, block_size=64 should succeed");
        }
    }
    
    // Test for bug #3: Blocks should be properly aligned
    #[test]
    fn test_block_alignment() {
        // Create memory that might not be aligned - use an offset to force misalignment
        let mut memory = [0u8; 1024 + 1];
        // Start at offset 1 to force potential misalignment
        let memory_ptr = NonNull::new(unsafe { memory.as_mut_ptr().add(1) }).unwrap();
        
        unsafe {
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
            
            // Request alignment of 16 - this should fail if blocks aren't properly aligned
            let layout = Layout::from_size_align(32, 16).unwrap();
            let block = pool.allocate(layout).unwrap();
            let block_ptr = block.as_ptr() as *const u8 as usize;
            
            // Block should be aligned to requested alignment (16)
            // This will fail if the allocator doesn't ensure alignment
            assert_eq!(block_ptr % 16, 0, "Block should be aligned to 16 bytes, got ptr at offset {}", block_ptr % 16);
            
            // Also test with alignment 8
            let layout2 = Layout::from_size_align(32, 8).unwrap();
            let block2 = pool.allocate(layout2).unwrap();
            let block2_ptr = block2.as_ptr() as *const u8 as usize;
            assert_eq!(block2_ptr % 8, 0, "Block should be aligned to 8 bytes, got ptr at offset {}", block2_ptr % 8);
        }
    }
    
    // Test for bug #2: Alignment check should verify actual alignment, not compare to block_size
    #[test]
    fn test_alignment_check_should_verify_alignment() {
        // Test with a case where align() < block_size but block might not be aligned
        let mut memory = [0u8; 1024 + 1];
        let memory_ptr = NonNull::new(unsafe { memory.as_mut_ptr().add(1) }).unwrap();
        
        unsafe {
            let mut pool = PoolAllocator::new(memory_ptr, 1024, 64).unwrap();
            
            // Request alignment of 16, block_size is 64
            // align() = 16, block_size = 64, so 16 <= 64 passes the current check
            // But if the block isn't actually aligned to 16, this is wrong
            let layout = Layout::from_size_align(32, 16).unwrap();
            let result = pool.allocate(layout);
            
            // Should succeed AND the returned block should be aligned
            let block = result.expect("Allocation should succeed");
            let block_ptr = block.as_ptr() as *const u8 as usize;
            assert_eq!(block_ptr % 16, 0, "Returned block must be aligned to requested alignment");
        }
    }
}
