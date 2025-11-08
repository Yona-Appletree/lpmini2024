use core::ptr::NonNull;

use crate::error::AllocError;
use crate::pool::LpAllocator;

#[cfg(feature = "std")]
mod storage {
    use std::cell::RefCell;
    use std::thread_local;

    use super::*;

    thread_local! {
        static ROOT_POOL: RefCell<Option<LpAllocator>> = const { RefCell::new(None) };
    }

    pub(super) fn set(pool: LpAllocator) {
        ROOT_POOL.with(|cell| {
            cell.replace(Some(pool));
        });
    }

    pub(super) fn exists() -> bool {
        ROOT_POOL.with(|cell| cell.borrow().is_some())
    }

    pub(super) fn with_ref<R, F>(f: F) -> Result<R, AllocError>
    where
        F: FnOnce(&LpAllocator) -> Result<R, AllocError>,
    {
        ROOT_POOL.with(|cell| {
            let borrow = cell.borrow();
            let pool = borrow.as_ref().ok_or(AllocError::PoolExhausted)?;
            f(pool)
        })
    }

    pub(super) fn with_mut<R, F>(f: F) -> Result<R, AllocError>
    where
        F: FnOnce(&mut LpAllocator) -> Result<R, AllocError>,
    {
        ROOT_POOL.with(|cell| {
            let mut borrow = cell.borrow_mut();
            let pool = borrow.as_mut().ok_or(AllocError::PoolExhausted)?;
            f(pool)
        })
    }
}

#[cfg(not(feature = "std"))]
mod storage {
    use spin::Mutex;

    use super::*;

    static ROOT_POOL: Mutex<Option<LpAllocator>> = Mutex::new(None);

    pub(super) fn set(pool: LpAllocator) {
        let mut guard = ROOT_POOL.lock();
        *guard = Some(pool);
    }

    pub(super) fn exists() -> bool {
        ROOT_POOL.lock().is_some()
    }

    pub(super) fn with_ref<R, F>(f: F) -> Result<R, AllocError>
    where
        F: FnOnce(&LpAllocator) -> Result<R, AllocError>,
    {
        let guard = ROOT_POOL.lock();
        let pool = guard.as_ref().ok_or(AllocError::PoolExhausted)?;
        f(pool)
    }

    pub(super) fn with_mut<R, F>(f: F) -> Result<R, AllocError>
    where
        F: FnOnce(&mut LpAllocator) -> Result<R, AllocError>,
    {
        let mut guard = ROOT_POOL.lock();
        let pool = guard.as_mut().ok_or(AllocError::PoolExhausted)?;
        f(pool)
    }
}

/// Main memory pool interface with thread-local allocator
pub struct LpMemoryPool;

impl LpMemoryPool {
    /// Create a new memory pool
    ///
    /// # Safety
    /// - `memory` must point to a valid memory region of at least `size` bytes
    /// - Memory must remain valid for the lifetime of the pool
    pub unsafe fn new(memory: NonNull<u8>, size: usize) -> Result<Self, AllocError> {
        let root_pool = LpAllocator::new(memory, size)?;
        storage::set(root_pool);
        Ok(LpMemoryPool)
    }

    /// Execute a closure with the pool active
    pub fn run<F, R, E>(&self, f: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
        E: From<AllocError>,
    {
        if !storage::exists() {
            return Err(E::from(AllocError::PoolExhausted));
        }
        // Execute closure - it will access pool via with_active_pool()
        f()
    }

    /// Get current memory usage statistics
    pub fn stats(&self) -> Result<PoolStats, AllocError> {
        storage::with_ref(|pool| {
            Ok(PoolStats {
                used_bytes: pool.used_bytes(),
                capacity: pool.capacity(),
                used_blocks: pool.used_blocks(),
                free_blocks: pool.free_blocks(),
            })
        })
    }

    /// Get current used bytes
    pub fn used_bytes(&self) -> Result<usize, AllocError> {
        self.stats().map(|stats| stats.used_bytes)
    }

    /// Get total capacity in bytes
    pub fn capacity(&self) -> Result<usize, AllocError> {
        self.stats().map(|stats| stats.capacity)
    }

    /// Get available bytes (capacity - used)
    pub fn available_bytes(&self) -> Result<usize, AllocError> {
        let stats = self.stats()?;
        Ok(stats.capacity - stats.used_bytes)
    }

    /// Get usage percentage (0.0 to 1.0)
    pub fn usage_ratio(&self) -> Result<f32, AllocError> {
        let stats = self.stats()?;
        if stats.capacity == 0 {
            return Ok(0.0);
        }
        Ok(stats.used_bytes as f32 / stats.capacity as f32)
    }
}

/// Statistics about pool usage
#[derive(Debug, Clone, Copy)]
pub struct PoolStats {
    pub used_bytes: usize,
    pub capacity: usize,
    pub used_blocks: usize,
    pub free_blocks: usize,
}

/// Execute a closure with access to the active pool
pub(crate) fn with_active_pool<F, R>(f: F) -> Result<R, AllocError>
where
    F: FnOnce(&mut LpAllocator) -> Result<R, AllocError>,
{
    storage::with_mut(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool_creation() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let pool = LpMemoryPool::new(memory_ptr, 1024).unwrap();
            let stats = pool.stats().unwrap();
            assert_eq!(stats.capacity, 1024);
            assert_eq!(stats.used_bytes, 0);
            assert_eq!(pool.available_bytes().unwrap(), 1024);
            assert_eq!(pool.usage_ratio().unwrap(), 0.0);
        }
    }

    #[test]
    fn test_run() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let pool = LpMemoryPool::new(memory_ptr, 1024).unwrap();
            let result = pool.run(|| Ok::<i32, AllocError>(42)).unwrap();
            assert_eq!(result, 42);
        }
    }

    #[test]
    fn test_usage_meta() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

        unsafe {
            let pool = LpMemoryPool::new(memory_ptr, 1024).unwrap();

            // Initially empty
            assert_eq!(pool.used_bytes().unwrap(), 0);
            assert_eq!(pool.available_bytes().unwrap(), 1024);

            // Allocate something and keep it alive
            use crate::collections::LpBox;
            let _boxed = pool
                .run(|| LpBox::try_new_with_scope(42i32, Some("test_scope")))
                .unwrap();

            // Should have used some memory (boxed is still alive)
            let used = pool.used_bytes().unwrap();
            assert!(used > 0);
            assert_eq!(pool.available_bytes().unwrap(), 1024 - used);

            // Drop it
            drop(_boxed);

            // Memory should be freed
            assert_eq!(pool.used_bytes().unwrap(), 0);
        }
    }
}
