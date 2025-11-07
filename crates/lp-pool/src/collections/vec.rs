use core::alloc::Layout;
use core::ptr::NonNull;
use crate::error::AllocError;
use crate::memory_pool::with_active_pool;

#[cfg(feature = "alloc-meta")]
use super::alloc_meta::{AllocationMeta, record_allocation_meta, remove_allocation_meta};

/// Pool-backed Vec
pub struct LpVec<T> {
    data: NonNull<u8>,
    len: usize,
    capacity: usize,
    _marker: core::marker::PhantomData<T>,
    #[cfg(feature = "alloc-meta")]
    scope: Option<&'static str>,
}

impl<T> LpVec<T> {
    pub fn new() -> Self {
        LpVec {
            data: NonNull::dangling(),
            len: 0,
            capacity: 0,
            _marker: core::marker::PhantomData,
            #[cfg(feature = "alloc-meta")]
            scope: None,
        }
    }
    
    /// Create a new LpVec with a scope identifier for metadata tracking
    #[cfg(feature = "alloc-meta")]
    pub fn new_with_scope(scope: Option<&'static str>) -> Self {
        LpVec {
            data: NonNull::dangling(),
            len: 0,
            capacity: 0,
            _marker: core::marker::PhantomData,
            scope,
        }
    }
    
    /// Create a new LpVec with a scope identifier for metadata tracking
    #[cfg(not(feature = "alloc-meta"))]
    pub fn new_with_scope(_scope: Option<&'static str>) -> Self {
        Self::new()
    }
    
    pub fn try_push(&mut self, item: T) -> Result<(), AllocError> {
        if self.len >= self.capacity {
            // Need to grow
            let new_cap = if self.capacity == 0 { 4 } else { self.capacity * 2 };
            self.try_reserve(new_cap)?;
        }
        
        unsafe {
            let ptr = self.data.as_ptr().add(self.len * core::mem::size_of::<T>()) as *mut T;
            core::ptr::write(ptr, item);
        }
        self.len += 1;
        Ok(())
    }
    
    pub fn try_reserve(&mut self, new_cap: usize) -> Result<(), AllocError> {
        if new_cap <= self.capacity {
            return Ok(());
        }
        
        let new_layout = Layout::array::<T>(new_cap)
            .map_err(|_| AllocError::InvalidLayout)?;
        
        #[cfg(feature = "alloc-meta")]
        let meta = AllocationMeta {
            type_name: core::any::type_name::<T>(),
            scope: self.scope,
        };
        
        with_active_pool(|pool| {
            let new_data = if self.capacity > 0 {
                // Use grow to reallocate
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                let new_size = new_layout.size();
                
                #[cfg(feature = "alloc-meta")]
                {
                    remove_allocation_meta(meta, old_layout.size());
                }
                
                unsafe {
                    let grown = pool.grow(self.data, old_layout, new_size)?;
                    
                    #[cfg(feature = "alloc-meta")]
                    {
                        record_allocation_meta(meta, new_layout.size());
                    }
                    
                    grown
                }
            } else {
                // First allocation
                let allocated = pool.allocate(new_layout)?;
                
                #[cfg(feature = "alloc-meta")]
                {
                    record_allocation_meta(meta, new_layout.size());
                }
                
                allocated
            };
            
            self.data = NonNull::new(new_data.as_ptr() as *mut u8).unwrap();
            self.capacity = new_cap;
            Ok(())
        })
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        unsafe {
            let ptr = self.data.as_ptr().add(index * core::mem::size_of::<T>()) as *const T;
            Some(&*ptr)
        }
    }
    
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        unsafe {
            let ptr = self.data.as_ptr().add(index * core::mem::size_of::<T>()) as *mut T;
            Some(&mut *ptr)
        }
    }
    
    /// Get raw slice of the underlying data (for internal use)
    pub(crate) fn as_raw_slice(&self) -> &[T] {
        unsafe {
            core::slice::from_raw_parts(
                self.data.as_ptr() as *const T,
                self.len,
            )
        }
    }
}

impl<T> Drop for LpVec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            let layout = Layout::array::<T>(self.capacity).unwrap();
            
            #[cfg(feature = "alloc-meta")]
            {
                let meta = AllocationMeta {
                    type_name: core::any::type_name::<T>(),
                    scope: self.scope,
                };
                remove_allocation_meta(meta, layout.size());
            }
            
            let _ = with_active_pool(|pool| {
                unsafe {
                    pool.deallocate(self.data, layout);
                }
                Ok::<(), AllocError>(())
            });
        }
    }
}

impl<T> Default for LpVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::LpMemoryPool;
    use core::ptr::NonNull;
    
    fn setup_pool() -> LpMemoryPool {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        unsafe {
            LpMemoryPool::new(memory_ptr, 16384, 128).unwrap()
        }
    }
    
    #[test]
    fn test_vec_new() {
        let pool = setup_pool();
        pool.run(|| {
            let vec = LpVec::<i32>::new();
            assert_eq!(vec.len(), 0);
            assert_eq!(vec.capacity(), 0);
            Ok(())
        }).unwrap();
    }
    
    #[test]
    fn test_vec_push() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;
            assert_eq!(vec.len(), 3);
            assert!(vec.capacity() >= 3);
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[test]
    fn test_vec_get() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(10)?;
            vec.try_push(20)?;
            vec.try_push(30)?;
            
            assert_eq!(vec.get(0), Some(&10));
            assert_eq!(vec.get(1), Some(&20));
            assert_eq!(vec.get(2), Some(&30));
            assert_eq!(vec.get(3), None);
            
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[test]
    fn test_vec_get_mut() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(10)?;
            vec.try_push(20)?;
            
            if let Some(val) = vec.get_mut(0) {
                *val = 100;
            }
            
            assert_eq!(vec.get(0), Some(&100));
            assert_eq!(vec.get(1), Some(&20));
            
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[test]
    fn test_vec_growth() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            
            // Push more than initial capacity (4)
            for i in 0..10 {
                vec.try_push(i)?;
            }
            
            assert_eq!(vec.len(), 10);
            assert!(vec.capacity() >= 10);
            
            // Verify all values
            for i in 0..10 {
                assert_eq!(vec.get(i), Some(&i));
            }
            
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[test]
    fn test_vec_reserve() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_reserve(20)?;
            assert!(vec.capacity() >= 20);
            
            // Should be able to push without reallocating
            let old_cap = vec.capacity();
            vec.try_push(1)?;
            assert_eq!(vec.capacity(), old_cap);
            
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[cfg(feature = "alloc-meta")]
    #[test]
    fn test_vec_with_scope() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::<i32>::new_with_scope(Some("test_scope"));
            vec.try_push(1)?;
            vec.try_push(2)?;
            assert_eq!(vec.len(), 2);
            Ok::<(), AllocError>(())
        }).unwrap();
    }
}

