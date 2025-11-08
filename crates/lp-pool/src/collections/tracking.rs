#[cfg(feature = "alloc-tracking")]
use alloc::collections::BTreeMap as TrackingMap;
use alloc::format;
#[cfg(all(feature = "alloc-tracking", not(feature = "std")))]
use spin::Mutex;
#[cfg(all(feature = "alloc-tracking", feature = "std"))]
use std::cell::RefCell as StdRefCell;
#[cfg(all(feature = "alloc-tracking", feature = "std"))]
use std::thread_local;

#[cfg(feature = "alloc-tracking")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct AllocationKey {
    pub type_name: &'static str,
    pub scope: Option<&'static str>,
}

#[cfg(feature = "alloc-tracking")]
#[derive(Debug, Clone, Copy)]
pub(crate) struct TypeStats {
    pub count: usize,
    pub total_bytes: usize,
}

#[cfg(feature = "alloc-tracking")]
mod storage {
    use super::*;

    #[cfg(feature = "std")]
    thread_local! {
        static TYPE_STATS: StdRefCell<TrackingMap<AllocationKey, TypeStats>> =
            const { StdRefCell::new(TrackingMap::new()) };
    }

    #[cfg(feature = "std")]
    pub(super) fn with_stats_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut TrackingMap<AllocationKey, TypeStats>) -> R,
    {
        TYPE_STATS.with(|cell| f(&mut cell.borrow_mut()))
    }

    #[cfg(feature = "std")]
    pub(super) fn with_stats<F, R>(f: F) -> R
    where
        F: FnOnce(&TrackingMap<AllocationKey, TypeStats>) -> R,
    {
        TYPE_STATS.with(|cell| f(&cell.borrow()))
    }

    #[cfg(not(feature = "std"))]
    static TYPE_STATS: Mutex<TrackingMap<AllocationKey, TypeStats>> =
        Mutex::new(TrackingMap::new());

    #[cfg(not(feature = "std"))]
    pub(super) fn with_stats_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut TrackingMap<AllocationKey, TypeStats>) -> R,
    {
        let mut guard = TYPE_STATS.lock();
        f(&mut *guard)
    }

    #[cfg(not(feature = "std"))]
    pub(super) fn with_stats<F, R>(f: F) -> R
    where
        F: FnOnce(&TrackingMap<AllocationKey, TypeStats>) -> R,
    {
        let guard = TYPE_STATS.lock();
        f(&*guard)
    }
}

#[cfg(feature = "alloc-tracking")]
pub(crate) fn track_allocation(key: AllocationKey, size: usize) {
    storage::with_stats_mut(|stats| {
        let entry = stats.entry(key).or_insert(TypeStats {
            count: 0,
            total_bytes: 0,
        });
        entry.count += 1;
        entry.total_bytes += size;
    });
}

#[cfg(feature = "alloc-tracking")]
pub(crate) fn track_deallocation(key: AllocationKey, size: usize) {
    storage::with_stats_mut(|stats| {
        if let Some(entry) = stats.get_mut(&key) {
            entry.count = entry.count.saturating_sub(1);
            entry.total_bytes = entry.total_bytes.saturating_sub(size);
            if entry.count == 0 {
                stats.remove(&key);
            }
        }
    });
}

/// Print memory statistics (only available with alloc-tracking feature)
///
/// Note: Requires a print function to be provided. For std environments,
/// use `print_memory_stats_with` with `println!` or similar.
#[cfg(feature = "alloc-tracking")]
pub fn print_memory_stats_with<F>(print: F)
where
    F: Fn(&str),
{
    storage::with_stats(|stats| {
        print("Memory Statistics by Type and Scope:");
        print("----------------------------------------------------------------------------------------");
        print(&format!(
            "{:<40} {:<20} {:>10} {:>10}",
            "Type", "Scope", "Count", "Bytes"
        ));
        print("----------------------------------------------------------------------------------------");

        let mut total_bytes = 0;
        let mut total_count = 0;

        for (key, stat) in stats.iter() {
            let scope_str = key.scope.unwrap_or("(none)");
            print(&format!(
                "{:<40} {:<20} {:>10} {:>10}",
                key.type_name, scope_str, stat.count, stat.total_bytes
            ));
            total_bytes += stat.total_bytes;
            total_count += stat.count;
        }

        print("----------------------------------------------------------------------------------------");
        print(&format!(
            "{:<62} {:>10} {:>10}",
            "TOTAL", total_count, total_bytes
        ));
    });
}

#[cfg(feature = "alloc-tracking")]
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

#[cfg(not(feature = "alloc-tracking"))]
pub fn print_memory_stats() {
    // No-op when tracking is disabled
}

#[cfg(not(feature = "alloc-tracking"))]
pub fn print_memory_stats_with<F>(_print: F)
where
    F: Fn(&str),
{
    // No-op when tracking is disabled
}
