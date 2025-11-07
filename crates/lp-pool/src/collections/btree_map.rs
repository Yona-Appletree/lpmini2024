use alloc::collections::BTreeMap;
use crate::error::AllocError;

/// Pool-backed BTreeMap
/// 
/// Note: Currently wraps `alloc::collections::BTreeMap` which uses the global allocator.
/// Metadata tracking will not be accurate until a custom pool-backed implementation is added.
pub struct PoolBTreeMap<K, V> {
    map: BTreeMap<K, V>,
    #[cfg(feature = "alloc-meta")]
    scope: Option<&'static str>,
}

impl<K, V> PoolBTreeMap<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        PoolBTreeMap {
            map: BTreeMap::new(),
            #[cfg(feature = "alloc-meta")]
            scope: None,
        }
    }
    
    /// Create a new PoolBTreeMap with a scope identifier for metadata tracking
    /// 
    /// Note: Metadata tracking is not currently accurate as this wraps the global allocator.
    /// Accurate tracking will be available when a custom pool-backed implementation is added.
    #[cfg(feature = "alloc-meta")]
    pub fn new_with_scope(scope: Option<&'static str>) -> Self {
        PoolBTreeMap {
            map: BTreeMap::new(),
            scope,
        }
    }
    
    /// Create a new PoolBTreeMap with a scope identifier for metadata tracking
    #[cfg(not(feature = "alloc-meta"))]
    pub fn new_with_scope(_scope: Option<&'static str>) -> Self {
        Self::new()
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    
    #[test]
    fn test_btree_map_new() {
        let map = PoolBTreeMap::<i32, i32>::new();
        assert_eq!(map.len(), 0);
    }
    
    #[test]
    fn test_btree_map_insert() {
        let mut map = PoolBTreeMap::new();
        assert_eq!(map.try_insert(1, 10).unwrap(), None);
        assert_eq!(map.try_insert(2, 20).unwrap(), None);
        assert_eq!(map.len(), 2);
    }
    
    #[test]
    fn test_btree_map_insert_replace() {
        let mut map = PoolBTreeMap::new();
        map.try_insert(1, 10).unwrap();
        assert_eq!(map.try_insert(1, 100).unwrap(), Some(10));
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1), Some(&100));
    }
    
    #[test]
    fn test_btree_map_get() {
        let mut map = PoolBTreeMap::new();
        map.try_insert(1, 10).unwrap();
        map.try_insert(2, 20).unwrap();
        
        assert_eq!(map.get(&1), Some(&10));
        assert_eq!(map.get(&2), Some(&20));
        assert_eq!(map.get(&3), None);
    }
    
    #[test]
    fn test_btree_map_string_keys() {
        let mut map = PoolBTreeMap::new();
        map.try_insert(String::from("a"), 1).unwrap();
        map.try_insert(String::from("b"), 2).unwrap();
        
        assert_eq!(map.get(&String::from("a")), Some(&1));
        assert_eq!(map.get(&String::from("b")), Some(&2));
    }
    
    #[cfg(feature = "alloc-meta")]
    #[test]
    fn test_btree_map_with_scope() {
        let mut map = PoolBTreeMap::new_with_scope(Some("test_scope"));
        map.try_insert(1, 10).unwrap();
        assert_eq!(map.get(&1), Some(&10));
    }
}

