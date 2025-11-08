use core::ptr::NonNull;

use super::map_get::{get_node, get_node_mut};
use super::map_insert::insert_node;
use super::map_remove::remove_node;
use super::node::Node;

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
    #[allow(dead_code)]
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

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn clear(&mut self) {
        if let Some(root) = self.root.take() {
            unsafe {
                Self::drop_tree(root);
            }
        }
        self.len = 0;
    }

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

// Re-export insert/get/remove methods
impl<K, V> LpBTreeMap<K, V>
where
    K: Ord,
{
    pub fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, crate::error::AllocError> {
        if let Some(root) = self.root {
            match insert_node(root, key, value)? {
                (None, increment) => {
                    if increment {
                        self.len += 1;
                    }
                    Ok(None)
                }
                (Some(old_value), _) => Ok(Some(old_value)),
            }
        } else {
            let node = Node::allocate(key, value)?;
            self.root = Some(node);
            self.len = 1;
            Ok(None)
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(root) = self.root {
            unsafe {
                let node = &*root.as_ptr();
                get_node(node, key)
            }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if let Some(root) = self.root {
            unsafe {
                let node = &mut *root.as_ptr();
                get_node_mut(node, key)
            }
        } else {
            None
        }
    }

    pub fn try_remove(&mut self, key: &K) -> Result<Option<V>, crate::error::AllocError> {
        if let Some(root) = self.root {
            unsafe {
                match remove_node(&mut self.root, root, key)? {
                    Some(value) => {
                        self.len -= 1;
                        Ok(Some(value))
                    }
                    None => Ok(None),
                }
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use super::*;
    use crate::allow_global_alloc;
    use crate::error::AllocError;
    use crate::memory_pool::LpMemoryPool;

    fn setup_pool() -> LpMemoryPool {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() }
    }

    #[test]
    fn test_btree_map_new() {
        let pool = setup_pool();
        pool.run(|| {
            let map = LpBTreeMap::<i32, i32>::new();
            assert_eq!(map.len(), 0);
            assert!(map.is_empty());
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_is_empty() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = LpBTreeMap::new();
            assert!(map.is_empty());

            map.try_insert(1, 10)?;
            assert!(!map.is_empty());

            map.try_remove(&1)?;
            assert!(map.is_empty());

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_contains_key() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = LpBTreeMap::new();
            assert!(!map.contains_key(&1));

            map.try_insert(1, 10)?;
            assert!(map.contains_key(&1));
            assert!(!map.contains_key(&2));

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_clear() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = LpBTreeMap::new();
            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;
            map.try_insert(3, 30)?;

            assert_eq!(map.len(), 3);
            map.clear();
            assert_eq!(map.len(), 0);
            assert!(map.is_empty());

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_drop_order() {
        use core::sync::atomic::{AtomicUsize, Ordering};

        static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

        struct DropCounter;
        impl Drop for DropCounter {
            fn drop(&mut self) {
                DROP_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }

        let pool = setup_pool();
        DROP_COUNT.store(0, Ordering::SeqCst);

        pool.run(|| {
            {
                let mut map = LpBTreeMap::new();
                map.try_insert(1, DropCounter)?;
                map.try_insert(2, DropCounter)?;
                map.try_insert(3, DropCounter)?;
            }

            assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 3);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_clear_verify_memory() {
        let pool = setup_pool();
        let before = pool.used_bytes().unwrap();

        pool.run(|| {
            let mut map = LpBTreeMap::new();
            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;
            map.try_insert(3, 30)?;

            map.clear();

            let after = pool.used_bytes().unwrap();
            assert_eq!(after, before, "Memory should be freed after clear");

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_reinsert_after_clear() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = LpBTreeMap::new();
            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;
            map.clear();

            map.try_insert(3, 30)?;
            map.try_insert(4, 40)?;
            assert_eq!(map.len(), 2);
            assert_eq!(map.get(&3), Some(&30));
            assert_eq!(map.get(&4), Some(&40));

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}
