use core::ptr::NonNull;
use crate::error::AllocError;
use super::node::Node;

#[cfg(feature = "alloc-meta")]
use super::super::alloc_meta::AllocationMeta;

/// Pool-backed BTreeMap implementation
/// 
/// Uses a binary search tree structure (simplified B-tree) with nodes allocated from the pool.
/// 
/// **Note**: This is a simplified implementation using a binary search tree, not a true B-tree.
/// For balanced performance, consider using a proper B-tree implementation. This implementation
/// maintains ordering but may degrade to O(n) performance with unbalanced data.
/// 
/// All nodes are allocated from the active memory pool via `with_active_pool()`.
pub struct LpBTreeMap<K, V>
where
    K: Ord,
{
    root: Option<NonNull<Node<K, V>>>,
    len: usize,
    #[cfg(feature = "alloc-meta")]
    scope: Option<&'static str>,
}

impl<K, V> LpBTreeMap<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        LpBTreeMap {
            root: None,
            len: 0,
            #[cfg(feature = "alloc-meta")]
            scope: None,
        }
    }
    
    /// Create a new LpBTreeMap with a scope identifier for metadata tracking
    #[cfg(feature = "alloc-meta")]
    pub fn new_with_scope(scope: Option<&'static str>) -> Self {
        LpBTreeMap {
            root: None,
            len: 0,
            scope,
        }
    }
    
    /// Create a new LpBTreeMap with a scope identifier for metadata tracking
    #[cfg(not(feature = "alloc-meta"))]
    pub fn new_with_scope(_scope: Option<&'static str>) -> Self {
        Self::new()
    }
    
    pub fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, AllocError> {
        if let Some(root) = self.root {
            unsafe {
                match Self::insert_node(root, key, value)? {
                    (None, increment) => {
                        if increment {
                            self.len += 1;
                        }
                        Ok(None)
                    }
                    (Some(old_value), _) => Ok(Some(old_value)),
                }
            }
        } else {
            let node = Node::allocate(key, value)?;
            self.root = Some(node);
            self.len = 1;
            Ok(None)
        }
    }
    
    unsafe fn insert_node(
        node_ptr: NonNull<Node<K, V>>,
        key: K,
        value: V,
    ) -> Result<(Option<V>, bool), AllocError> {
        let node = &mut *node_ptr.as_ptr();
        match key.cmp(node.key()) {
            core::cmp::Ordering::Equal => {
                // Replace existing value
                let old_value = core::ptr::replace(node.value_mut(), value);
                Ok((Some(old_value), false))
            }
            core::cmp::Ordering::Less => {
                if let Some(left) = node.left() {
                    Self::insert_node(left, key, value)
                } else {
                    let new_node = Node::allocate(key, value)?;
                    node.set_left(Some(new_node));
                    Ok((None, true))
                }
            }
            core::cmp::Ordering::Greater => {
                if let Some(right) = node.right() {
                    Self::insert_node(right, key, value)
                } else {
                    let new_node = Node::allocate(key, value)?;
                    node.set_right(Some(new_node));
                    Ok((None, true))
                }
            }
        }
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(root) = self.root {
            unsafe {
                // Safety: The node is allocated from the pool and lives as long as self
                let node = &*root.as_ptr();
                Self::get_node(node, key)
            }
        } else {
            None
        }
    }
    
    fn get_node<'a>(node: &'a Node<K, V>, key: &K) -> Option<&'a V> {
        match key.cmp(node.key()) {
            core::cmp::Ordering::Equal => Some(node.value()),
            core::cmp::Ordering::Less => {
                node.left().and_then(|left| {
                    unsafe {
                        // Safety: The node is allocated from the pool and lives as long as the parent
                        let left_node = &*left.as_ptr();
                        Self::get_node(left_node, key)
                    }
                })
            }
            core::cmp::Ordering::Greater => {
                node.right().and_then(|right| {
                    unsafe {
                        // Safety: The node is allocated from the pool and lives as long as the parent
                        let right_node = &*right.as_ptr();
                        Self::get_node(right_node, key)
                    }
                })
            }
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::LpMemoryPool;
    use core::ptr::NonNull;
    use alloc::string::String;
    
    fn setup_pool() -> LpMemoryPool {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        unsafe {
            LpMemoryPool::new(memory_ptr, 16384, 128).unwrap()
        }
    }
    
    #[test]
    fn test_btree_map_new() {
        let pool = setup_pool();
        pool.run(|| {
            let map = LpBTreeMap::<i32, i32>::new();
            assert_eq!(map.len(), 0);
            Ok(())
        }).unwrap();
    }
    
    #[test]
    fn test_btree_map_insert() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = LpBTreeMap::new();
            assert_eq!(map.try_insert(1, 10)?, None);
            assert_eq!(map.try_insert(2, 20)?, None);
            assert_eq!(map.len(), 2);
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[test]
    fn test_btree_map_insert_replace() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = LpBTreeMap::new();
            map.try_insert(1, 10)?;
            assert_eq!(map.try_insert(1, 100)?, Some(10));
            assert_eq!(map.len(), 1);
            assert_eq!(map.get(&1), Some(&100));
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[test]
    fn test_btree_map_get() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = LpBTreeMap::new();
            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;
            
            assert_eq!(map.get(&1), Some(&10));
            assert_eq!(map.get(&2), Some(&20));
            assert_eq!(map.get(&3), None);
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[test]
    fn test_btree_map_string_keys() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = LpBTreeMap::new();
            map.try_insert(String::from("a"), 1)?;
            map.try_insert(String::from("b"), 2)?;
            
            assert_eq!(map.get(&String::from("a")), Some(&1));
            assert_eq!(map.get(&String::from("b")), Some(&2));
            Ok::<(), AllocError>(())
        }).unwrap();
    }
    
    #[cfg(feature = "alloc-meta")]
    #[test]
    fn test_btree_map_with_scope() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = LpBTreeMap::new_with_scope(Some("test_scope"));
            map.try_insert(1, 10)?;
            assert_eq!(map.get(&1), Some(&10));
            Ok::<(), AllocError>(())
        }).unwrap();
    }
}

impl<K, V> Default for LpBTreeMap<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Drop for LpBTreeMap<K, V>
where
    K: Ord,
{
    fn drop(&mut self) {
        if let Some(root) = self.root {
            unsafe {
                Self::drop_tree(root);
            }
        }
    }
}

impl<K, V> LpBTreeMap<K, V>
where
    K: Ord,
{
    unsafe fn drop_tree(node_ptr: NonNull<Node<K, V>>) {
        let node = &*node_ptr.as_ptr();
        if let Some(left) = node.left() {
            Self::drop_tree(left);
        }
        if let Some(right) = node.right() {
            Self::drop_tree(right);
        }
        Node::deallocate(node_ptr);
    }
}

