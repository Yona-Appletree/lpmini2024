use core::mem;

/// Magic number to validate block headers
const BLOCK_MAGIC: u32 = 0xDEADBEEF;

/// Minimum block size to accommodate header + free list pointer
pub const MIN_BLOCK_SIZE: usize = mem::size_of::<BlockHeader>() + mem::size_of::<*mut u8>();

/// Header stored at the beginning of each block (allocated or free)
/// Padded to 32 bytes to ensure data area is aligned to at least 8 bytes
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct BlockHeader {
    /// Magic number for validation
    magic: u32,
    /// Total size of this block including the header
    pub size: usize,
    /// Whether this block is currently allocated
    pub is_allocated: bool,
    /// Offset from block start to data area (for alignment padding)
    pub data_offset: u16,
    /// Padding to reach 32 bytes (ensures data area is well-aligned)
    _padding: [u8; 6],
}

impl BlockHeader {
    /// Create a new block header
    #[inline]
    pub fn new(size: usize, is_allocated: bool) -> Self {
        BlockHeader {
            magic: BLOCK_MAGIC,
            size,
            is_allocated,
            data_offset: mem::size_of::<BlockHeader>() as u16,
            _padding: [0; 6],
        }
    }

    /// Create a new block header with custom data offset (for alignment padding)
    #[inline]
    pub fn new_with_offset(size: usize, is_allocated: bool, data_offset: u16) -> Self {
        BlockHeader {
            magic: BLOCK_MAGIC,
            size,
            is_allocated,
            data_offset,
            _padding: [0; 6],
        }
    }

    /// Validate that this header has the correct magic number
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.magic == BLOCK_MAGIC && self.size >= mem::size_of::<BlockHeader>()
    }

    /// Read a header from memory
    ///
    /// # Safety
    /// - `ptr` must point to a valid BlockHeader
    /// - The memory must be properly initialized
    #[inline]
    pub unsafe fn read(ptr: *const u8) -> Self {
        core::ptr::read(ptr as *const BlockHeader)
    }

    /// Write a header to memory
    ///
    /// # Safety
    /// - `ptr` must point to valid memory of at least size_of::<BlockHeader>()
    #[inline]
    pub unsafe fn write(ptr: *mut u8, header: BlockHeader) {
        core::ptr::write(ptr as *mut BlockHeader, header);
    }

    /// Get pointer to the data area (after the header)
    /// With 32-byte aligned header, this provides natural alignment up to 32 bytes
    ///
    /// # Safety
    /// - block_ptr must be a valid pointer to a block header
    #[inline]
    pub unsafe fn data_ptr(block_ptr: *mut u8) -> *mut u8 {
        block_ptr.add(mem::size_of::<BlockHeader>())
    }

    /// Get pointer to the block header from a data pointer
    ///
    /// # Safety
    /// - data_ptr must be a valid data pointer returned from allocate
    #[inline]
    pub unsafe fn block_ptr_from_data(data_ptr: *mut u8) -> *mut u8 {
        // With no padding support, data is always at header_size offset
        data_ptr.sub(mem::size_of::<BlockHeader>())
    }

    /// Get the size of the data area (excluding header)
    #[inline]
    pub fn data_size(&self) -> usize {
        self.size.saturating_sub(mem::size_of::<BlockHeader>())
    }

    /// Get the next free block pointer (only valid for free blocks)
    ///
    /// # Safety
    /// - This block must be free (is_allocated == false)
    /// - The block must be large enough to hold the header + pointer
    #[inline]
    pub unsafe fn read_next_free(block_ptr: *const u8) -> *mut u8 {
        let data_ptr = block_ptr.add(mem::size_of::<BlockHeader>());
        core::ptr::read(data_ptr as *const *mut u8)
    }

    /// Write the next free block pointer (only for free blocks)
    ///
    /// # Safety
    /// - This block must be free (is_allocated == false)
    /// - The block must be large enough to hold the header + pointer
    #[inline]
    pub unsafe fn write_next_free(block_ptr: *mut u8, next: *mut u8) {
        let data_ptr = block_ptr.add(mem::size_of::<BlockHeader>());
        core::ptr::write(data_ptr as *mut *mut u8, next);
    }

    /// Get the size needed for an allocation including header
    #[inline]
    pub fn total_size_for_allocation(data_size: usize) -> usize {
        mem::size_of::<BlockHeader>() + data_size
    }

    /// Align a size to the header size (ensures proper alignment for headers)
    #[inline]
    pub fn align_size(size: usize) -> usize {
        let align = mem::align_of::<BlockHeader>();
        (size + align - 1) & !(align - 1)
    }

    /// Get pointer to the next block in memory (not the free list)
    ///
    /// # Safety
    /// - The returned pointer may be out of bounds; caller must check
    #[inline]
    pub unsafe fn next_block_ptr(&self, current_ptr: *mut u8) -> *mut u8 {
        current_ptr.add(self.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_size() {
        // Verify header size is reasonable (with padding for alignment)
        let header_size = mem::size_of::<BlockHeader>();
        assert_eq!(header_size, 32, "Header size changed: {}", header_size);
    }

    #[test]
    fn test_header_read_write() {
        let mut memory = [0u8; 32];
        let ptr = memory.as_mut_ptr();

        let header = BlockHeader::new(32, false);
        unsafe {
            BlockHeader::write(ptr, header);
            let read_header = BlockHeader::read(ptr);
            assert_eq!(read_header.size, 32);
            assert!(!read_header.is_allocated);
        }
    }

    #[test]
    fn test_data_ptr() {
        let mut memory = [0u8; 64];
        let block_ptr = memory.as_mut_ptr();
        let data_ptr = unsafe { BlockHeader::data_ptr(block_ptr) };

        let offset = unsafe { data_ptr.offset_from(block_ptr) };
        assert_eq!(offset as usize, mem::size_of::<BlockHeader>());
    }

    #[test]
    fn test_block_ptr_from_data() {
        let mut memory = [0u8; 64];
        let block_ptr = memory.as_mut_ptr();

        // Write a header
        let header = BlockHeader::new(64, true);
        unsafe {
            BlockHeader::write(block_ptr, header);
            let data_ptr = BlockHeader::data_ptr(block_ptr);
            let recovered_ptr = BlockHeader::block_ptr_from_data(data_ptr);
            assert_eq!(block_ptr, recovered_ptr);
        }
    }

    #[test]
    fn test_next_free_read_write() {
        let mut memory = [0u8; 64];
        let block_ptr = memory.as_mut_ptr();

        let header = BlockHeader::new(64, false);
        unsafe {
            BlockHeader::write(block_ptr, header);

            let next_ptr = 0x1234 as *mut u8;
            BlockHeader::write_next_free(block_ptr, next_ptr);
            let read_next = BlockHeader::read_next_free(block_ptr);
            assert_eq!(read_next, next_ptr);
        }
    }

    #[test]
    fn test_data_size() {
        let header = BlockHeader::new(64, false);
        let data_size = header.data_size();
        assert_eq!(data_size, 64 - mem::size_of::<BlockHeader>());
    }

    #[test]
    fn test_total_size_for_allocation() {
        let data_size = 32;
        let total = BlockHeader::total_size_for_allocation(data_size);
        assert_eq!(total, mem::size_of::<BlockHeader>() + data_size);
    }

    #[test]
    fn test_align_size() {
        let align = mem::align_of::<BlockHeader>();
        assert_eq!(BlockHeader::align_size(0), 0);
        assert_eq!(BlockHeader::align_size(1), align);
        assert_eq!(BlockHeader::align_size(align), align);
        assert_eq!(BlockHeader::align_size(align + 1), align * 2);
    }

    #[test]
    fn test_min_block_size() {
        let min_size = MIN_BLOCK_SIZE;
        assert!(min_size >= mem::size_of::<BlockHeader>());
        assert!(min_size >= mem::size_of::<BlockHeader>() + mem::size_of::<*mut u8>());
    }
}
