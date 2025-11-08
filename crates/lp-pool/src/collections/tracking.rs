#[cfg(feature = "alloc-tracking")]
use alloc::collections::BTreeMap as TrackingMap;
#[cfg(feature = "alloc-tracking")]
use thread_local::ThreadLocal;
#[cfg(feature = "alloc-tracking")]
use core::cell::RefCell;
use alloc::format;

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
static TYPE_STATS: ThreadLocal<RefCell<TrackingMap<AllocationKey, TypeStats>>> = ThreadLocal::new();

#[cfg(feature = "alloc-tracking")]
pub(crate) fn track_allocation(key: AllocationKey, size: usize) {
    let mut stats_ref = TYPE_STATS.get_or(|| RefCell::new(TrackingMap::new())).borrow_mut();
    let entry = stats_ref.entry(key).or_insert(TypeStats {
        count: 0,
        total_bytes: 0,
    });
    entry.count += 1;
    entry.total_bytes += size;
}

#[cfg(feature = "alloc-tracking")]
pub(crate) fn track_deallocation(key: AllocationKey, size: usize) {
    let mut stats_ref = TYPE_STATS.get_or(|| RefCell::new(TrackingMap::new())).borrow_mut();
    if let Some(entry) = stats_ref.get_mut(&key) {
        entry.count = entry.count.saturating_sub(1);
        entry.total_bytes = entry.total_bytes.saturating_sub(size);
        if entry.count == 0 {
            stats_ref.remove(&key);
        }
    }
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
    let stats_ref = TYPE_STATS.get_or(|| RefCell::new(TrackingMap::new())).borrow();
    print("Memory Statistics by Type and Scope:");
    print("----------------------------------------------------------------------------------------");
    print(&format!("{:<40} {:<20} {:>10} {:>10}", "Type", "Scope", "Count", "Bytes"));
    print("----------------------------------------------------------------------------------------");
    
    let mut total_bytes = 0;
    let mut total_count = 0;
    
    for (key, stat) in stats_ref.iter() {
        let scope_str = key.scope.unwrap_or("(none)");
        print(&format!("{:<40} {:<20} {:>10} {:>10}", key.type_name, scope_str, stat.count, stat.total_bytes));
        total_bytes += stat.total_bytes;
        total_count += stat.count;
    }
    
    print("----------------------------------------------------------------------------------------");
    print(&format!("{:<62} {:>10} {:>10}", "TOTAL", total_count, total_bytes));
}

#[cfg(feature = "alloc-tracking")]
pub fn print_memory_stats() {
    // Default implementation - users should use print_memory_stats_with for no_std
    #[cfg(feature = "std")]
    {
        print_memory_stats_with(|s| println!("{}", s));
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

