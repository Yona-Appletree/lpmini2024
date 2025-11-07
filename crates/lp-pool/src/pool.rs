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
        
        // Check alignment - block must be aligned enough
        // We'll align blocks to block_size boundary, so check that
        if layout.align() > self.block_size {
            return Err(AllocError::InvalidLayout);
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
}
