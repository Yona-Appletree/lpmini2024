#[cfg(feature = "alloc-meta")]
use alloc::collections::BTreeMap as MetaMap;
#[cfg(feature = "alloc-meta")]
use thread_local::ThreadLocal;
#[cfg(feature = "alloc-meta")]
use core::cell::RefCell;
#[cfg(feature = "alloc-meta")]
use alloc::format;

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
static ALLOCATION_META_STATS: ThreadLocal<RefCell<MetaMap<AllocationMeta, AllocationStats>>> = ThreadLocal::new();

#[cfg(feature = "alloc-meta")]
pub(crate) fn record_allocation_meta(meta: AllocationMeta, size: usize) {
    let mut stats_ref = ALLOCATION_META_STATS.get_or(|| RefCell::new(MetaMap::new())).borrow_mut();
    let entry = stats_ref.entry(meta).or_insert(AllocationStats {
        count: 0,
        total_bytes: 0,
    });
    entry.count += 1;
    entry.total_bytes += size;
}

#[cfg(feature = "alloc-meta")]
pub(crate) fn remove_allocation_meta(meta: AllocationMeta, size: usize) {
    let mut stats_ref = ALLOCATION_META_STATS.get_or(|| RefCell::new(MetaMap::new())).borrow_mut();
    if let Some(entry) = stats_ref.get_mut(&meta) {
        entry.count = entry.count.saturating_sub(1);
        entry.total_bytes = entry.total_bytes.saturating_sub(size);
        if entry.count == 0 {
            stats_ref.remove(&meta);
        }
    }
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
    let stats_ref = ALLOCATION_META_STATS.get_or(|| RefCell::new(MetaMap::new())).borrow();
    print("Memory Statistics by Type and Scope:");
    print("----------------------------------------------------------------------------------------");
    print(&format!("{:<40} {:<20} {:>10} {:>10}", "Type", "Scope", "Count", "Bytes"));
    print("----------------------------------------------------------------------------------------");
    
    let mut total_bytes = 0;
    let mut total_count = 0;
    
    for (meta, stat) in stats_ref.iter() {
        let scope_str = meta.scope.unwrap_or("(none)");
        print(&format!("{:<40} {:<20} {:>10} {:>10}", meta.type_name, scope_str, stat.count, stat.total_bytes));
        total_bytes += stat.total_bytes;
        total_count += stat.count;
    }
    
    print("----------------------------------------------------------------------------------------");
    print(&format!("{:<62} {:>10} {:>10}", "TOTAL", total_count, total_bytes));
}

#[cfg(feature = "alloc-meta")]
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
