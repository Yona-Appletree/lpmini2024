use core::alloc::Layout;
use core::mem;
use core::ptr::NonNull;

use super::allocator::LpAllocator;
use crate::block_header::BlockHeader;

impl LpAllocator {
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
        let block_ptr =
            BlockHeader::block_ptr_from_data(ptr.as_ptr(), self.memory.as_ptr(), self.capacity);

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
        header.data_offset = mem::size_of::<BlockHeader>() as u16;
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
}

#[cfg(test)]
mod tests {
    use core::alloc::Layout;
    use core::ptr::NonNull;

    use super::*;

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
    fn test_block_header_validation() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();

            let layout = Layout::from_size_align(64, 8).unwrap();
            let ptr = pool.allocate(layout).unwrap();

            // Get block pointer and verify header is valid
            let block_ptr = BlockHeader::block_ptr_from_data(
                ptr.as_ptr() as *mut u8,
                pool.memory.as_ptr(),
                pool.capacity,
            );
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
