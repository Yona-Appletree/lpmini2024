#[cfg(any(feature = "std", test))]
use core::cell::RefCell;
#[cfg(any(feature = "std", test))]
use std::thread_local;

#[cfg(all(not(feature = "std"), not(test)))]
use spin::Mutex;

use crate::error::AllocError;
use crate::pool::LpAllocator;

#[cfg(any(feature = "std", test))]
thread_local! {
    static ALLOCATOR: RefCell<Option<LpAllocator>> = const { RefCell::new(None) };
}

#[cfg(all(not(feature = "std"), not(test)))]
static ALLOCATOR: Mutex<Option<LpAllocator>> = Mutex::new(None);

pub(crate) fn set_allocator(allocator: LpAllocator) {
    #[cfg(any(feature = "std", test))]
    {
        ALLOCATOR.with(|cell| {
            *cell.borrow_mut() = Some(allocator);
        });
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        *ALLOCATOR.lock() = Some(allocator);
    }
    #[cfg(feature = "alloc-meta")]
    {
        super::clear_meta();
    }
}

pub(crate) fn allocator_exists() -> bool {
    #[cfg(any(feature = "std", test))]
    {
        ALLOCATOR.with(|cell| {
            if cell.borrow().is_some() {
                return true;
            }
            #[cfg(feature = "default_pool")]
            {
                ensure_default_pool().is_ok()
            }
            #[cfg(not(feature = "default_pool"))]
            {
                false
            }
        })
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        ALLOCATOR.lock().is_some()
    }
}

#[cfg(all(feature = "default_pool", any(feature = "std", test)))]
fn ensure_default_pool() -> Result<(), AllocError> {
    use alloc::boxed::Box;
    use alloc::vec;
    use core::ptr::NonNull;

    ALLOCATOR.with(|cell| {
        if cell.borrow().is_some() {
            return Ok(());
        }

        const DEFAULT_POOL_SIZE: usize = 512 * 1024;
        let mut boxed = vec![0u8; DEFAULT_POOL_SIZE].into_boxed_slice();

        match NonNull::new(boxed.as_mut_ptr()) {
            Some(ptr) => {
                match unsafe { LpAllocator::new(ptr, DEFAULT_POOL_SIZE) } {
                    Ok(allocator) => {
                        // Leak the boxed slice so the memory stays valid
                        Box::leak(boxed);
                        *cell.borrow_mut() = Some(allocator);
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
            None => Err(AllocError::PoolExhausted),
        }
    })
}

pub(crate) fn with_allocator<F, R>(f: F) -> Result<R, AllocError>
where
    F: FnOnce(&LpAllocator) -> Result<R, AllocError>,
{
    #[cfg(any(feature = "std", test))]
    {
        ALLOCATOR.with(|cell| {
            if cell.borrow().is_none() {
                #[cfg(feature = "default_pool")]
                {
                    if let Err(err) = ensure_default_pool() {
                        return Err(err);
                    }
                }
                #[cfg(not(feature = "default_pool"))]
                {
                    return Err(AllocError::PoolExhausted);
                }
            }
            let borrow = cell.borrow();
            let allocator = borrow.as_ref().ok_or(AllocError::PoolExhausted)?;
            f(allocator)
        })
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        let guard = ALLOCATOR.lock();
        let allocator = guard.as_ref().ok_or(AllocError::PoolExhausted)?;
        f(allocator)
    }
}

pub(crate) fn with_allocator_mut<F, R>(f: F) -> Result<R, AllocError>
where
    F: FnOnce(&mut LpAllocator) -> Result<R, AllocError>,
{
    #[cfg(any(feature = "std", test))]
    {
        ALLOCATOR.with(|cell| {
            if cell.borrow().is_none() {
                #[cfg(feature = "default_pool")]
                {
                    if let Err(err) = ensure_default_pool() {
                        return Err(err);
                    }
                }
                #[cfg(not(feature = "default_pool"))]
                {
                    return Err(AllocError::PoolExhausted);
                }
            }
            let mut borrow = cell.borrow_mut();
            let allocator = borrow.as_mut().ok_or(AllocError::PoolExhausted)?;
            f(allocator)
        })
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        let mut guard = ALLOCATOR.lock();
        let allocator = guard.as_mut().ok_or(AllocError::PoolExhausted)?;
        f(allocator)
    }
}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use super::*;

    #[test]
    fn test_set_and_get_allocator() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let allocator = unsafe { LpAllocator::new(memory_ptr, 1024).unwrap() };

        set_allocator(allocator);
        assert!(allocator_exists());

        with_allocator(|alloc| {
            assert_eq!(alloc.capacity(), 1024);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}
