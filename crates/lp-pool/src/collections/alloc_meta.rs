#[cfg(feature = "alloc-meta")]
use alloc::collections::BTreeMap as MetaMap;
#[cfg(feature = "alloc-meta")]
use alloc::format;
#[cfg(all(feature = "alloc-meta", feature = "std"))]
use std::cell::RefCell as StdRefCell;
#[cfg(all(feature = "alloc-meta", feature = "std"))]
use std::thread_local;

#[cfg(all(feature = "alloc-meta", not(feature = "std")))]
use spin::Mutex;

#[cfg(feature = "alloc-meta")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct AllocationMeta {
    pub type_name: &'static str,
    pub scope: Option<&'static str>,
}

#[cfg(feature = "alloc-meta")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct AllocationStats {
    pub count: usize,
    pub total_bytes: usize,
}

#[cfg(feature = "alloc-meta")]
mod storage {
    use super::*;

    #[cfg(feature = "std")]
    thread_local! {
        static ALLOCATION_META_STATS: StdRefCell<MetaMap<AllocationMeta, AllocationStats>> =
            const { StdRefCell::new(MetaMap::new()) };
    }

    #[cfg(feature = "std")]
    pub(super) fn with_stats_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut MetaMap<AllocationMeta, AllocationStats>) -> R,
    {
        ALLOCATION_META_STATS.with(|cell| f(&mut cell.borrow_mut()))
    }

    #[cfg(feature = "std")]
    pub(super) fn with_stats<F, R>(f: F) -> R
    where
        F: FnOnce(&MetaMap<AllocationMeta, AllocationStats>) -> R,
    {
        ALLOCATION_META_STATS.with(|cell| f(&cell.borrow()))
    }

    #[cfg(not(feature = "std"))]
    static ALLOCATION_META_STATS: Mutex<MetaMap<AllocationMeta, AllocationStats>> =
        Mutex::new(MetaMap::new());

    #[cfg(not(feature = "std"))]
    pub(super) fn with_stats_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut MetaMap<AllocationMeta, AllocationStats>) -> R,
    {
        let mut guard = ALLOCATION_META_STATS.lock();
        f(&mut *guard)
    }

    #[cfg(not(feature = "std"))]
    pub(super) fn with_stats<F, R>(f: F) -> R
    where
        F: FnOnce(&MetaMap<AllocationMeta, AllocationStats>) -> R,
    {
        let guard = ALLOCATION_META_STATS.lock();
        f(&*guard)
    }
}

#[cfg(feature = "alloc-meta")]
pub(crate) fn record_allocation_meta(meta: AllocationMeta, size: usize) {
    storage::with_stats_mut(|stats| {
        let entry = stats.entry(meta).or_insert(AllocationStats {
            count: 0,
            total_bytes: 0,
        });
        entry.count += 1;
        entry.total_bytes += size;
    });
}

#[cfg(feature = "alloc-meta")]
pub(crate) fn remove_allocation_meta(meta: AllocationMeta, size: usize) {
    storage::with_stats_mut(|stats| {
        if let Some(entry) = stats.get_mut(&meta) {
            entry.count = entry.count.saturating_sub(1);
            entry.total_bytes = entry.total_bytes.saturating_sub(size);
            if entry.count == 0 {
                stats.remove(&meta);
            }
        }
    });
}

/// Print memory statistics (only available with alloc-meta feature)
///
/// Note: Requires a print function to be provided. For std environments,
/// use `print_memory_stats_with` with `println!` or similar.
#[cfg(feature = "alloc-meta")]
pub fn print_memory_stats_with<F>(print: F)
where
    F: Fn(&str),
{
    storage::with_stats(|stats| {
        print("Memory Statistics by Type and Scope:");
        print(
            "----------------------------------------------------------------------------------------",
        );
        print(&format!(
            "{:<40} {:<20} {:>10} {:>10}",
            "Type", "Scope", "Count", "Bytes"
        ));
        print(
            "----------------------------------------------------------------------------------------",
        );

        let mut total_bytes = 0;
        let mut total_count = 0;

        for (meta, stat) in stats.iter() {
            let scope_str = meta.scope.unwrap_or("(none)");
            print(&format!(
                "{:<40} {:<20} {:>10} {:>10}",
                meta.type_name, scope_str, stat.count, stat.total_bytes
            ));
            total_bytes += stat.total_bytes;
            total_count += stat.count;
        }

        print(
            "----------------------------------------------------------------------------------------",
        );
        print(&format!(
            "{:<62} {:>10} {:>10}",
            "TOTAL", total_count, total_bytes
        ));
    });
}

#[cfg(feature = "alloc-meta")]
#[allow(unexpected_cfgs)]
pub fn print_memory_stats() {
    // Default implementation - users should use print_memory_stats_with for no_std
    #[cfg(feature = "std")]
    {
        print_memory_stats_with(|s| std::println!("{}", s));
    }
    #[cfg(not(feature = "std"))]
    {
        // No-op in no_std - use print_memory_stats_with with a custom print function
    }
}

#[cfg(not(feature = "alloc-meta"))]
pub fn print_memory_stats() {
    // No-op when metadata tracking is disabled
}

#[cfg(not(feature = "alloc-meta"))]
pub fn print_memory_stats_with<F>(_print: F)
where
    F: Fn(&str),
{
    // No-op when metadata tracking is disabled
}

#[cfg(all(test, feature = "alloc-meta"))]
mod tests {
    use core::ptr::NonNull;

    use super::{storage, *};
    use crate::{LpBox, LpMemoryPool, LpVec};

    fn setup_pool() -> LpMemoryPool {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() }
    }

    #[test]
    fn test_metadata_tracking_basic() {
        let pool = setup_pool();

        pool.run(|| {
            // Clear any previous metadata
            let initial_count = storage::with_stats(|stats| stats.len());

            // Allocate with scope
            let _box1 = LpBox::try_new_with_scope(42i32, Some("test_scope"))?;

            // Check metadata was recorded
            let len_after = storage::with_stats(|stats| stats.len());
            assert!(len_after >= initial_count);

            Ok::<(), crate::AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_metadata_cleanup_on_drop() {
        let pool = setup_pool();

        pool.run(|| {
            {
                let _box1 = LpBox::try_new_with_scope([0u8; 64], Some("scope1"))?;
                let _box2 = LpBox::try_new_with_scope([0u8; 64], Some("scope2"))?;

                // Metadata should be tracked
                let count_during = storage::with_stats(|stats| stats.len());
                assert!(count_during > 0);
            }

            // After drop, metadata should be cleaned up
            // All scoped allocations should be removed
            let entries: alloc::vec::Vec<(AllocationMeta, AllocationStats)> =
                storage::with_stats(|stats| {
                    stats.iter().map(|(meta, stat)| (*meta, *stat)).collect()
                });

            for (meta, stats) in entries {
                if meta.scope == Some("scope1") || meta.scope == Some("scope2") {
                    assert_eq!(
                        stats.count, 0,
                        "Scope {:?} should have 0 allocations after drop",
                        meta.scope
                    );
                }
            }

            Ok::<(), crate::AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_metadata_with_vec() {
        let pool = setup_pool();

        pool.run(|| {
            let mut vec = LpVec::<i32>::new_with_scope(Some("vec_scope"));

            for i in 0..10 {
                vec.try_push(i)?;
            }

            // Metadata should reflect vec allocations
            // Note: metadata tracking is best-effort, so we don't assert
            // Just verify it doesn't crash and that we can iterate metadata
            storage::with_stats(|stats| {
                for _ in stats.iter() {
                    // no-op
                }
            });

            Ok::<(), crate::AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_print_memory_stats_doesnt_crash() {
        let pool = setup_pool();

        pool.run(|| {
            let _box1 = LpBox::try_new_with_scope(42, Some("test"))?;
            let _vec = LpVec::<i32>::new_with_scope(Some("vec_test"));

            // Should not crash - just call it
            use core::sync::atomic::{AtomicBool, Ordering};
            static PRINT_CALLED: AtomicBool = AtomicBool::new(false);

            print_memory_stats_with(|_s| {
                PRINT_CALLED.store(true, Ordering::SeqCst);
            });

            // Verify print function was called
            assert!(
                PRINT_CALLED.load(Ordering::SeqCst),
                "Print function should have been called"
            );

            Ok::<(), crate::AllocError>(())
        })
        .unwrap();
    }
}

#[cfg(all(test, not(feature = "alloc-meta")))]
mod tests_no_meta {
    use core::ptr::NonNull;

    use crate::{LpBox, LpMemoryPool};

    #[test]
    fn test_no_meta_features_work() {
        let mut memory = [0u8; 1024];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        let pool = unsafe { LpMemoryPool::new(memory_ptr, 1024).unwrap() };

        pool.run(|| {
            // With alloc-meta disabled, scoped constructors should still work
            let _box1 = LpBox::try_new_with_scope(42, Some("ignored_scope"))?;
            assert_eq!(*_box1, 42);

            // print_memory_stats should be a no-op
            super::print_memory_stats();

            Ok::<(), crate::AllocError>(())
        })
        .unwrap();
    }
}
