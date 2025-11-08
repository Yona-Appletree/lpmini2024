use core::ptr::NonNull;

use crate::*;

// Note: Most drop safety tests are in individual collection test modules
// These tests focus on cross-collection drop order and panic safety

#[test]
fn test_drop_order_across_collections() {
    use core::sync::atomic::{AtomicUsize, Ordering};

    static CROSS_DROP_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

    struct CrossDropTracker(usize);

    impl Drop for CrossDropTracker {
        fn drop(&mut self) {
            let seq = CROSS_DROP_SEQUENCE.fetch_add(1, Ordering::SeqCst);
            // LIFO: last created should drop first
            assert_eq!(
                seq, self.0,
                "Drop order incorrect: expected {}, got {}",
                self.0, seq
            );
        }
    }

    let mut memory = [0u8; 16384];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    let pool = unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() };

    CROSS_DROP_SEQUENCE.store(0, Ordering::SeqCst);

    pool.run(|| {
        let _box1 = LpBox::try_new(CrossDropTracker(2))?;
        let _box2 = LpBox::try_new(CrossDropTracker(1))?;
        let _box3 = LpBox::try_new(CrossDropTracker(0))?;

        Ok::<(), AllocError>(())
    })
    .unwrap();

    assert_eq!(CROSS_DROP_SEQUENCE.load(Ordering::SeqCst), 3);
}

#[test]
fn test_allocation_failure_leaves_consistent_state() {
    let mut memory = [0u8; 1024]; // Small pool to trigger OOM
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    let pool = unsafe { LpMemoryPool::new(memory_ptr, 1024).unwrap() };

    pool.run(|| {
        let mut vec = LpVec::new();

        // Push until OOM
        loop {
            match vec.try_push(0u8) {
                Ok(_) => {}
                Err(AllocError::OutOfMemory { .. }) | Err(AllocError::PoolExhausted) => break,
                Err(e) => panic!("Unexpected error: {:?}", e),
            }
        }

        // Vec should still be valid and usable
        let len_before = vec.len();
        assert!(len_before > 0);

        // Can still access elements
        assert_eq!(vec.get(0), Some(&0u8));

        // Clear should work
        vec.clear();
        assert_eq!(vec.len(), 0);

        Ok::<(), AllocError>(())
    })
    .unwrap();
}

#[test]
fn test_memory_cleanup_on_early_return() {
    let mut memory = [0u8; 8192];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    let pool = unsafe { LpMemoryPool::new(memory_ptr, 8192).unwrap() };

    let before = pool.used_bytes().unwrap();

    let result: Result<(), AllocError> = pool.run(|| {
        let _box1 = LpBox::try_new([0u8; 100])?;
        let _box2 = LpBox::try_new([0u8; 100])?;

        // Early return with error
        Err(AllocError::InvalidLayout)
    });

    assert!(result.is_err());

    // Memory should be cleaned up even though we returned early
    let after = pool.used_bytes().unwrap();
    assert_eq!(after, before, "Memory should be cleaned up on early return");
}
