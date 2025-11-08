use crate::block_header::BlockHeader;

use super::allocator::LpAllocator;

impl LpAllocator {
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
}

#[cfg(test)]
mod tests {
    use core::alloc::Layout;
    use core::ptr::NonNull;

    use super::*;

    #[test]
    fn test_stats() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            assert_eq!(pool.capacity(), 1024);
            assert_eq!(pool.used_bytes(), 0);
            assert_eq!(pool.used_blocks(), 0);
            assert_eq!(pool.free_blocks(), 1); // One large free block

            let layout = Layout::from_size_align(32, 8).unwrap();
            let _ptr1 = pool.allocate(layout).unwrap();
            assert_eq!(pool.used_blocks(), 1);
            assert!(pool.used_bytes() > 0);

            let _ptr2 = pool.allocate(layout).unwrap();
            assert_eq!(pool.used_blocks(), 2);
        }
    }
}
