use core::alloc::Layout;
use allocator_api2::alloc::{Allocator, AllocError as ApiAllocError};
use crate::error::AllocError;
use crate::memory_pool::with_active_pool;

/// Wrapper that implements `Allocator` trait for the thread-local pool
/// 
/// This wrapper allows the pool allocator to be used with collections that support
/// the `allocator-api2` trait, such as `alloc::vec::Vec` with custom allocators.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use lp_pool::{LpMemoryPool, LpAllocatorWrapper};
/// use core::ptr::NonNull;
/// use allocator_api2::alloc::Allocator;
/// 
/// let mut memory = [0u8; 4096];
/// let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
/// let pool = unsafe { LpMemoryPool::new(memory_ptr, 4096, 64).unwrap() };
/// 
/// pool.run(|| {
///     let allocator = LpAllocatorWrapper;
///     let layout = core::alloc::Layout::from_size_align(32, 8).unwrap();
///     let ptr = allocator.allocate(layout)?;
///     // Use ptr...
///     unsafe {
///         allocator.deallocate(NonNull::new(ptr.as_ptr() as *mut u8).unwrap(), layout);
///     }
///     Ok::<(), lp_pool::AllocError>(())
/// }).unwrap();
/// ```
pub struct LpAllocatorWrapper;

unsafe impl Allocator for LpAllocatorWrapper {
    fn allocate(&self, layout: Layout) -> Result<core::ptr::NonNull<[u8]>, ApiAllocError> {
        with_active_pool(|pool| {
            let result = pool.allocate(layout);
            Ok(result)
        })
        .and_then(|inner| inner)
        .map_err(|_| ApiAllocError)
    }

    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: Layout) {
        let _ = with_active_pool(|pool| {
            pool.deallocate(ptr, layout);
            Ok::<(), AllocError>(())
        });
    }

    unsafe fn grow(
        &self,
        ptr: core::ptr::NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, ApiAllocError> {
        with_active_pool(|pool| {
            let result = pool.grow(ptr, old_layout, new_layout.size());
            Ok(result)
        })
        .and_then(|inner| inner)
        .map_err(|_| ApiAllocError)
    }

    unsafe fn shrink(
        &self,
        ptr: core::ptr::NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, ApiAllocError> {
        with_active_pool(|pool| {
            let result = pool.shrink(ptr, old_layout, new_layout.size());
            Ok(result)
        })
        .and_then(|inner| inner)
        .map_err(|_| ApiAllocError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::LpMemoryPool;
    use core::ptr::NonNull;
    use core::alloc::Layout;
    
    fn setup_pool() -> LpMemoryPool {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        unsafe {
            LpMemoryPool::new(memory_ptr, 16384, 128).unwrap()
        }
    }
    
    #[test]
    fn test_allocator_allocate() {
        let pool = setup_pool();
        let allocator = LpAllocatorWrapper;
        
        pool.run(|| {
            let layout = Layout::from_size_align(64, 8).unwrap();
            let ptr = allocator.allocate(layout);
            assert!(ptr.is_ok());
            Ok(())
        }).unwrap();
    }
    
    #[test]
    fn test_allocator_deallocate() {
        let pool = setup_pool();
        let allocator = LpAllocatorWrapper;
        
        pool.run(|| {
            let layout = Layout::from_size_align(64, 8).unwrap();
            let ptr = allocator.allocate(layout).unwrap();
            let ptr_u8 = NonNull::new(ptr.as_ptr() as *mut u8).unwrap();
            unsafe {
                allocator.deallocate(ptr_u8, layout);
            }
            Ok(())
        }).unwrap();
    }
    
    #[test]
    fn test_allocator_grow() {
        let pool = setup_pool();
        let allocator = LpAllocatorWrapper;
        
        pool.run(|| {
            let old_layout = Layout::from_size_align(32, 8).unwrap();
            let new_layout = Layout::from_size_align(64, 8).unwrap();
            let ptr = allocator.allocate(old_layout).unwrap();
            let ptr_u8 = NonNull::new(ptr.as_ptr() as *mut u8).unwrap();
            let grown = unsafe {
                allocator.grow(ptr_u8, old_layout, new_layout)
            };
            assert!(grown.is_ok());
            Ok(())
        }).unwrap();
    }
    
    #[test]
    fn test_allocator_shrink() {
        let pool = setup_pool();
        let allocator = LpAllocatorWrapper;
        
        pool.run(|| {
            let old_layout = Layout::from_size_align(64, 8).unwrap();
            let new_layout = Layout::from_size_align(32, 8).unwrap();
            let ptr = allocator.allocate(old_layout).unwrap();
            let ptr_u8 = NonNull::new(ptr.as_ptr() as *mut u8).unwrap();
            let shrunk = unsafe {
                allocator.shrink(ptr_u8, old_layout, new_layout)
            };
            assert!(shrunk.is_ok());
            Ok(())
        }).unwrap();
    }
}
