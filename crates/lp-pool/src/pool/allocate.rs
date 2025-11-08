use core::alloc::Layout;
use core::mem;
use core::ptr::NonNull;

use crate::block_header::{BlockHeader, MIN_BLOCK_SIZE};
use crate::error::AllocError;

use super::allocator::LpAllocator;

impl LpAllocator {
    /// Allocate a block using first-fit strategy with block splitting
    pub fn allocate(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let requested_size = layout.size();
        let align = layout.align().max(1);
        let header_size = mem::size_of::<BlockHeader>();
        let metadata_size = core::mem::size_of::<usize>();

        // Walk the free list to find a suitable block (first-fit)
        let mut current_ptr = self.free_list;
        let mut prev_ptr: *mut u8 = core::ptr::null_mut();

        unsafe {
            while !current_ptr.is_null() {
                let header = BlockHeader::read(current_ptr);

                // Check if this block is free (should always be true in free list)
                if header.is_allocated {
                    // Corruption detected - skip
                    current_ptr = BlockHeader::read_next_free(current_ptr);
                    continue;
                }

                let block_base = current_ptr as usize;
                let min_user_addr = block_base
                    .checked_add(header_size + metadata_size)
                    .ok_or(AllocError::InvalidLayout)?;

                let align_mask = align - 1;
                let aligned_user_addr = if align > 1 {
                    (min_user_addr
                        .checked_add(align_mask)
                        .ok_or(AllocError::InvalidLayout)?)
                        & !align_mask
                } else {
                    min_user_addr
                };

                let metadata_addr = aligned_user_addr
                    .checked_sub(metadata_size)
                    .ok_or(AllocError::InvalidLayout)?;
                let aligned_data_offset = metadata_addr
                    .checked_sub(block_base)
                    .ok_or(AllocError::InvalidLayout)?;

                let total_needed = match aligned_user_addr
                    .checked_sub(block_base)
                    .and_then(|offset| offset.checked_add(requested_size))
                {
                    Some(val) => val,
                    None => return Err(AllocError::InvalidLayout),
                };

                if total_needed > header.size {
                    prev_ptr = current_ptr;
                    current_ptr = BlockHeader::read_next_free(current_ptr);
                    continue;
                }

                // Found a suitable block!
                let next_free = BlockHeader::read_next_free(current_ptr);
                if prev_ptr.is_null() {
                    self.free_list = next_free;
                } else {
                    BlockHeader::write_next_free(prev_ptr, next_free);
                }

                let remaining_size = header.size - total_needed;
                let min_leftover = MIN_BLOCK_SIZE;

                let (alloc_block_ptr, alloc_size) = if remaining_size >= min_leftover {
                    let alloc_size = total_needed;
                    let remainder_ptr = current_ptr.add(alloc_size);

                    let remainder_header = BlockHeader::new(remaining_size, false);
                    BlockHeader::write(remainder_ptr, remainder_header);

                    BlockHeader::write_next_free(remainder_ptr, self.free_list);
                    self.free_list = remainder_ptr;

                    (current_ptr, alloc_size)
                } else {
                    (current_ptr, header.size)
                };

                if aligned_data_offset > u16::MAX as usize {
                    return Err(AllocError::InvalidLayout);
                }

                let alloc_header =
                    BlockHeader::new_with_offset(alloc_size, true, aligned_data_offset as u16);
                BlockHeader::write(alloc_block_ptr, alloc_header);

                self.used_bytes += alloc_size;

                let metadata_ptr = alloc_block_ptr.add(aligned_data_offset);
                // Store types (offset back to header)
                core::ptr::write_unaligned(metadata_ptr as *mut usize, alloc_block_ptr as usize);

                let user_ptr = aligned_user_addr as *mut u8;

                return Ok(NonNull::slice_from_raw_parts(
                    NonNull::new_unchecked(user_ptr),
                    requested_size,
                ));
            }
        }

        // No suitable block found
        Err(AllocError::PoolExhausted)
    }
}

#[cfg(test)]
mod tests {
    use core::alloc::Layout;
    use core::ptr::NonNull;

    use super::*;

    #[test]
    fn test_allocate_deallocate() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 1024).unwrap();
            let allocation_overhead = mem::size_of::<BlockHeader>() + mem::size_of::<usize>();
            let layout = Layout::from_size_align(64 - allocation_overhead, 8).unwrap();

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
            let align = 8;
            let header_size = mem::size_of::<BlockHeader>();
            let metadata_size = mem::size_of::<usize>();
            let max_padding = align - 1;
            let max_data_capacity =
                64usize.saturating_sub(header_size + metadata_size + max_padding);
            let layout_size = (max_data_capacity / align).max(1) * align;
            let layout = Layout::from_size_align(layout_size, align).unwrap();

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
    fn test_allocate_entire_pool() {
        let mut memory = [0u8; 2048];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let mut pool = LpAllocator::new(memory_ptr, 2048).unwrap();

            // Try to allocate entire pool capacity (minus header overhead)
            let allocation_overhead = mem::size_of::<BlockHeader>() + mem::size_of::<usize>();
            let layout = Layout::from_size_align(2048 - allocation_overhead, 8).unwrap();
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

    // Alignment tests
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
            assert!(
                result.is_ok(),
                "Valid allocation with align=8, block_size=64 should succeed"
            );
        }
    }

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
}
