use crate::error::AllocError;
use super::vec::PoolVec;

/// Pool-backed String
pub struct PoolString {
    vec: PoolVec<u8>,
}

impl PoolString {
    pub fn new() -> Self {
        PoolString {
            vec: PoolVec::new(),
        }
    }
    
    pub fn try_push_str(&mut self, s: &str) -> Result<(), AllocError> {
        for byte in s.bytes() {
            self.vec.try_push(byte)?;
        }
        Ok(())
    }
    
    pub fn as_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(self.vec.as_raw_slice())
        }
    }
    
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl Default for PoolString {
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
    fn test_string_new() {
        let pool = setup_pool();
        pool.run(|| {
            let s = PoolString::new();
            assert_eq!(s.len(), 0);
            assert_eq!(s.as_str(), "");
            Ok(())
        }).unwrap();
    }
    
    #[test]
    fn test_string_push_str() {
        let pool = setup_pool();
        pool.run(|| {
            let mut s = PoolString::new();
            s.try_push_str("hello")?;
            assert_eq!(s.len(), 5);
            assert_eq!(s.as_str(), "hello");
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[test]
    fn test_string_multiple_push() {
        let pool = setup_pool();
        pool.run(|| {
            let mut s = PoolString::new();
            s.try_push_str("hello")?;
            s.try_push_str(" ")?;
            s.try_push_str("world")?;
            assert_eq!(s.as_str(), "hello world");
            assert_eq!(s.len(), 11);
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[test]
    fn test_string_empty() {
        let pool = setup_pool();
        pool.run(|| {
            let mut s = PoolString::new();
            s.try_push_str("")?;
            assert_eq!(s.len(), 0);
            assert_eq!(s.as_str(), "");
            Ok::<(), AllocError>(())
        }).unwrap();
    }
}

