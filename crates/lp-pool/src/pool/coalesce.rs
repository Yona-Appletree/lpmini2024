use core::ptr::null_mut;

use crate::block_header::{BlockHeader, MIN_BLOCK_SIZE};

use super::allocator::LpAllocator;

impl LpAllocator {
    /// Remove a block from the free list
    pub(crate) unsafe fn remove_from_free_list(&mut self, block_ptr: *mut u8) {
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
    pub(crate) unsafe fn try_coalesce_with_prev(
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
}

#[cfg(test)]
mod tests {
    use core::alloc::Layout;
    use core::ptr::NonNull;

    use super::*;

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
}
