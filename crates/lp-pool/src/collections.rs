use core::alloc::Layout;
use core::ptr::NonNull;
use alloc::collections::BTreeMap;
use crate::error::AllocError;
use crate::memory_pool::with_active_pool;

#[cfg(feature = "alloc-tracking")]
use alloc::collections::BTreeMap as TrackingMap;

#[cfg(feature = "alloc-tracking")]
use thread_local::ThreadLocal;

#[cfg(feature = "alloc-tracking")]
static TYPE_STATS: ThreadLocal<RefCell<TrackingMap<&'static str, TypeStats>>> = ThreadLocal::new();

#[cfg(feature = "alloc-tracking")]
use core::cell::RefCell;

#[cfg(feature = "alloc-tracking")]
#[derive(Debug, Clone, Copy)]
struct TypeStats {
    count: usize,
    total_bytes: usize,
}

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
}

impl<T> Default for PoolVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

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
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                self.vec.data.as_ptr(),
                self.vec.len,
            ))
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

/// Pool-backed BTreeMap
pub struct PoolBTreeMap<K, V> {
    map: BTreeMap<K, V>,
}

impl<K, V> PoolBTreeMap<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        PoolBTreeMap {
            map: BTreeMap::new(),
        }
    }
    
    pub fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, AllocError> {
        // BTreeMap uses global allocator, so we can't control it directly
        // For now, just use standard BTreeMap
        // TODO: Implement custom BTreeMap using pool allocator
        Ok(self.map.insert(key, value))
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
    
    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl<K, V> Default for PoolBTreeMap<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Pool-backed Box
pub struct PoolBox<T> {
    ptr: NonNull<T>,
    #[cfg(feature = "alloc-tracking")]
    type_name: &'static str,
}

impl<T> PoolBox<T> {
    pub fn try_new(value: T) -> Result<Self, AllocError> {
        let layout = Layout::new::<T>();
        
        let ptr = with_active_pool(|pool| {
            let allocated = pool.allocate(layout)?;
            let ptr = NonNull::new(allocated.as_ptr() as *mut T).unwrap();
            
            // Write value to allocated memory
            unsafe {
                core::ptr::write(ptr.as_ptr(), value);
            }
            
            #[cfg(feature = "alloc-tracking")]
            {
                let type_name = core::any::type_name::<T>();
                let mut stats_ref = TYPE_STATS.get_or(|| RefCell::new(TrackingMap::new())).borrow_mut();
                let entry = stats_ref.entry(type_name).or_insert(TypeStats {
                    count: 0,
                    total_bytes: 0,
                });
                entry.count += 1;
                entry.total_bytes += layout.size();
            }
            
            Ok(ptr)
        })?;
        
        Ok(PoolBox {
            ptr,
            #[cfg(feature = "alloc-tracking")]
            type_name: core::any::type_name::<T>(),
        })
    }
    
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.ptr.as_ptr() }
    }
    
    pub fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr.as_ptr() }
    }
}

impl<T> core::ops::Deref for PoolBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> core::ops::DerefMut for PoolBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T> Drop for PoolBox<T> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();
        
        // Deallocate
        let _ = with_active_pool(|pool| {
            unsafe {
                pool.deallocate(self.ptr.cast(), layout);
            }
            
            #[cfg(feature = "alloc-tracking")]
            {
                let mut stats_ref = TYPE_STATS.get_or(|| RefCell::new(TrackingMap::new())).borrow_mut();
                if let Some(entry) = stats_ref.get_mut(self.type_name) {
                    entry.count = entry.count.saturating_sub(1);
                    entry.total_bytes = entry.total_bytes.saturating_sub(layout.size());
                    if entry.count == 0 {
                        stats_ref.remove(self.type_name);
                    }
                }
            }
            
            Ok::<(), AllocError>(())
        });
        
        // Drop value
        unsafe {
            core::ptr::drop_in_place(self.ptr.as_ptr());
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
    print("Memory Statistics by Type:");
    print("------------------------------------------------------------");
    print(&format!("{:<40} {:>10} {:>10}", "Type", "Count", "Bytes"));
    print("------------------------------------------------------------");
    
    let mut total_bytes = 0;
    let mut total_count = 0;
    
    for (type_name, stat) in stats_ref.iter() {
        print(&format!("{:<40} {:>10} {:>10}", type_name, stat.count, stat.total_bytes));
        total_bytes += stat.total_bytes;
        total_count += stat.count;
    }
    
    print("------------------------------------------------------------");
    print(&format!("{:<40} {:>10} {:>10}", "TOTAL", total_count, total_bytes));
}

#[cfg(feature = "alloc-tracking")]
#[cfg(feature = "std")]
pub fn print_memory_stats() {
    print_memory_stats_with(|s| println!("{}", s));
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

