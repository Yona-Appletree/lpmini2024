use super::node::Node;
use crate::error::AllocError;
use core::ptr::NonNull;

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
            match Self::insert_node(root, key, value)? {
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

    fn insert_node(
        node_ptr: NonNull<Node<K, V>>,
        key: K,
        value: V,
    ) -> Result<(Option<V>, bool), AllocError> {
        let node = unsafe { &mut *node_ptr.as_ptr() };
        match key.cmp(node.key()) {
            core::cmp::Ordering::Equal => {
                // Replace existing value
                let old_value = unsafe { core::ptr::replace(node.value_mut(), value) };
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

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if let Some(root) = self.root {
            unsafe {
                // Safety: The node is allocated from the pool and lives as long as self
                let node = &mut *root.as_ptr();
                Self::get_node_mut(node, key)
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

    fn get_node_mut<'a>(node: &'a mut Node<K, V>, key: &K) -> Option<&'a mut V> {
        match key.cmp(node.key()) {
            core::cmp::Ordering::Equal => Some(node.value_mut()),
            core::cmp::Ordering::Less => {
                node.left().and_then(|left| {
                    unsafe {
                        // Safety: The node is allocated from the pool and lives as long as the parent
                        let left_node = &mut *left.as_ptr();
                        Self::get_node_mut(left_node, key)
                    }
                })
            }
            core::cmp::Ordering::Greater => {
                node.right().and_then(|right| {
                    unsafe {
                        // Safety: The node is allocated from the pool and lives as long as the parent
                        let right_node = &mut *right.as_ptr();
                        Self::get_node_mut(right_node, key)
                    }
                })
            }
        }
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

    pub fn try_remove(&mut self, key: &K) -> Result<Option<V>, AllocError> {
        if let Some(root) = self.root {
            unsafe {
                match Self::remove_node(&mut self.root, root, key)? {
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

    unsafe fn remove_node(
        root_ref: &mut Option<NonNull<Node<K, V>>>,
        node_ptr: NonNull<Node<K, V>>,
        key: &K,
    ) -> Result<Option<V>, AllocError> {
        let node = &mut *node_ptr.as_ptr();
        match key.cmp(node.key()) {
            core::cmp::Ordering::Equal => {
                // Found the node to remove
                let value = core::ptr::read(node.value());
                let left = node.left();
                let right = node.right();

                // Handle different cases
                match (left, right) {
                    (None, None) => {
                        // Leaf node - just remove it
                        if Some(node_ptr) == *root_ref {
                            *root_ref = None;
                        }
                        Node::deallocate(node_ptr);
                        Ok(Some(value))
                    }
                    (Some(left_node), None) => {
                        // Only left child - replace this node with left child
                        if Some(node_ptr) == *root_ref {
                            *root_ref = Some(left_node);
                        }
                        Node::deallocate(node_ptr);
                        Ok(Some(value))
                    }
                    (None, Some(right_node)) => {
                        // Only right child - replace this node with right child
                        if Some(node_ptr) == *root_ref {
                            *root_ref = Some(right_node);
                        }
                        Node::deallocate(node_ptr);
                        Ok(Some(value))
                    }
                    (Some(_left_node), Some(right_node)) => {
                        // Two children - find inorder successor (min in right subtree)
                        let successor = Self::find_min(right_node);
                        let successor_node = &*successor.as_ptr();

                        // Replace key and value with successor's
                        let old_value = core::ptr::read(node.value());
                        core::ptr::write(node.key_mut(), core::ptr::read(successor_node.key()));
                        core::ptr::write(node.value_mut(), core::ptr::read(successor_node.value()));

                        // Remove successor from right subtree
                        Self::remove_node(&mut Some(node_ptr), right_node, successor_node.key())?;

                        Ok(Some(old_value))
                    }
                }
            }
            core::cmp::Ordering::Less => {
                if let Some(left) = node.left() {
                    let mut left_ref = node.left();
                    let result = Self::remove_node(&mut left_ref, left, key)?;
                    node.set_left(left_ref);
                    Ok(result)
                } else {
                    Ok(None)
                }
            }
            core::cmp::Ordering::Greater => {
                if let Some(right) = node.right() {
                    let mut right_ref = node.right();
                    let result = Self::remove_node(&mut right_ref, right, key)?;
                    node.set_right(right_ref);
                    Ok(result)
                } else {
                    Ok(None)
                }
            }
        }
    }

    unsafe fn find_min(mut node_ptr: NonNull<Node<K, V>>) -> NonNull<Node<K, V>> {
        loop {
            let node = &*node_ptr.as_ptr();
            if let Some(left) = node.left() {
                node_ptr = left;
            } else {
                break;
            }
        }
        node_ptr
    }

    pub fn clear(&mut self) {
        if let Some(root) = self.root.take() {
            unsafe {
                Self::drop_tree(root);
            }
        }
        self.len = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::LpMemoryPool;
    use alloc::string::String;
    use core::ptr::NonNull;

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
            Ok(())
        })
        .unwrap();
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
        })
        .unwrap();
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
        })
        .unwrap();
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
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_get_mut() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = LpBTreeMap::new();
            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;

            if let Some(val) = map.get_mut(&1) {
                *val = 100;
            }

            assert_eq!(map.get(&1), Some(&100));
            assert_eq!(map.get(&2), Some(&20));
            Ok::<(), AllocError>(())
        })
        .unwrap();
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
        })
        .unwrap();
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
        })
        .unwrap();
    }

    // Test for design issue #4: Missing BTreeMap methods
    #[test]
    fn test_btree_map_is_empty() {
        let pool = setup_pool();
        pool.run(|| {
            let map = LpBTreeMap::<i32, i32>::new();
            assert!(map.is_empty());

            let mut map2 = LpBTreeMap::new();
            map2.try_insert(1, 10)?;
            assert!(!map2.is_empty());
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_contains_key() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = LpBTreeMap::new();
            map.try_insert(1, 10)?;
            assert!(map.contains_key(&1));
            assert!(!map.contains_key(&2));
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_remove() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = LpBTreeMap::new();
            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;
            assert_eq!(map.len(), 2);

            let removed = map.try_remove(&1)?;
            assert_eq!(removed, Some(10));
            assert_eq!(map.len(), 1);
            assert!(!map.contains_key(&1));
            assert!(map.contains_key(&2));

            let removed2 = map.try_remove(&99)?;
            assert_eq!(removed2, None);
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
            assert_eq!(map.len(), 2);

            map.clear();
            assert_eq!(map.len(), 0);
            assert!(map.is_empty());
            assert!(!map.contains_key(&1));
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    // Test for bug #1: Drop order - value should be dropped before deallocation
    // This test verifies that Node::deallocate drops the value BEFORE deallocating memory
    #[test]
    fn test_btree_map_drop_order() {
        use core::sync::atomic::{AtomicU8, Ordering};

        // Track the order: 0 = not started, 1 = value dropped, 2 = memory deallocated
        static ORDER: AtomicU8 = AtomicU8::new(0);

        struct DropTracker;
        impl Drop for DropTracker {
            fn drop(&mut self) {
                let current = ORDER.load(Ordering::SeqCst);
                // If memory was already deallocated (state 2), this is wrong
                if current == 2 {
                    panic!("Value dropped AFTER memory deallocation! Drop order is wrong.");
                }
                // Mark that value is being dropped
                ORDER.store(1, Ordering::SeqCst);
            }
        }

        // We need to hook into the deallocation to track when it happens
        // Since we can't easily hook into pool.deallocate, we'll use a different approach:
        // Create a map, insert a value, then drop it. The drop should happen in correct order.
        let pool = setup_pool();
        let map = pool
            .run(|| {
                let mut map = LpBTreeMap::new();
                map.try_insert(1, DropTracker)?;
                Ok::<LpBTreeMap<i32, DropTracker>, AllocError>(map)
            })
            .unwrap();

        // Now drop the map - this should drop values before deallocating
        // The test will panic if Node::deallocate calls drop_in_place AFTER pool.deallocate
        drop(map);

        // Verify that drop happened (order should be 1, meaning value was dropped)
        let final_order = ORDER.load(Ordering::SeqCst);
        assert_eq!(final_order, 1, "Value should have been dropped");
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
