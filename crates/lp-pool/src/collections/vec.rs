use core::alloc::Layout;
use core::ptr::NonNull;
use crate::error::AllocError;
use crate::memory_pool::with_active_pool;

/// Pool-backed Vec
pub struct PoolVec<T> {
    data: NonNull<u8>,
    len: usize,
    capacity: usize,
    _marker: core::marker::PhantomData<T>,
}

impl<T> PoolVec<T> {
    pub fn new() -> Self {
        PoolVec {
            data: NonNull::dangling(),
            len: 0,
            capacity: 0,
            _marker: core::marker::PhantomData,
        }
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
        
        let layout = Layout::array::<T>(new_cap)
            .map_err(|_| AllocError::InvalidLayout)?;
        
        with_active_pool(|pool| {
            let new_data = pool.allocate(layout)?;
            let new_ptr = NonNull::new(new_data.as_ptr() as *mut u8).unwrap();
            
            // Copy old data if any
            if self.len > 0 {
                let old_ptr = self.data.as_ptr() as *const T;
                let new_data_ptr = new_ptr.as_ptr() as *mut T;
                unsafe {
                    core::ptr::copy_nonoverlapping(old_ptr, new_data_ptr, self.len);
                }
            }
            
            // Deallocate old data if any
            if self.capacity > 0 {
                let old_layout = Layout::array::<T>(self.capacity).unwrap();
                unsafe {
                    pool.deallocate(self.data, old_layout);
                }
            }
            
            self.data = new_ptr;
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

impl<T> Default for PoolVec<T> {
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
            let vec = PoolVec::<i32>::new();
            assert_eq!(vec.len(), 0);
            assert_eq!(vec.capacity(), 0);
            Ok(())
        }).unwrap();
    }
    
    #[test]
    fn test_vec_push() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = PoolVec::new();
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
            let mut vec = PoolVec::new();
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
            let mut vec = PoolVec::new();
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
            let mut vec = PoolVec::new();
            
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
            let mut vec = PoolVec::new();
            vec.try_reserve(20)?;
            assert!(vec.capacity() >= 20);
            
            // Should be able to push without reallocating
            let old_cap = vec.capacity();
            vec.try_push(1)?;
            assert_eq!(vec.capacity(), old_cap);
            
            Ok::<(), AllocError>(())
        }).unwrap();
    }
}

