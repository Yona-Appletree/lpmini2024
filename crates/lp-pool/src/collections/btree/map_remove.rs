use core::ptr::NonNull;

use super::node::Node;
use crate::error::AllocError;

pub(super) unsafe fn remove_node<K, V>(
    root_ref: &mut Option<NonNull<Node<K, V>>>,
    node_ptr: NonNull<Node<K, V>>,
    key: &K,
) -> Result<Option<V>, AllocError>
where
    K: Ord,
{
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
                    let successor = find_min(right_node);
                    let successor_node = &*successor.as_ptr();

                    // Replace key and value with successor's
                    let old_value = core::ptr::read(node.value());
                    core::ptr::write(node.key_mut(), core::ptr::read(successor_node.key()));
                    core::ptr::write(node.value_mut(), core::ptr::read(successor_node.value()));

                    // Remove successor from right subtree
                    remove_node(&mut Some(node_ptr), right_node, successor_node.key())?;

                    Ok(Some(old_value))
                }
            }
        }
        core::cmp::Ordering::Less => {
            if let Some(left) = node.left() {
                let mut left_ref = node.left();
                let result = remove_node(&mut left_ref, left, key)?;
                node.set_left(left_ref);
                Ok(result)
            } else {
                Ok(None)
            }
        }
        core::cmp::Ordering::Greater => {
            if let Some(right) = node.right() {
                let mut right_ref = node.right();
                let result = remove_node(&mut right_ref, right, key)?;
                node.set_right(right_ref);
                Ok(result)
            } else {
                Ok(None)
            }
        }
    }
}

pub(super) unsafe fn find_min<K, V>(mut node_ptr: NonNull<Node<K, V>>) -> NonNull<Node<K, V>>
where
    K: Ord,
{
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

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use super::*;
    use crate::error::AllocError;
    use crate::memory_pool::LpMemoryPool;

    fn setup_pool() -> LpMemoryPool {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() }
    }

    #[test]
    fn test_btree_map_remove() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();
            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;
            map.try_insert(3, 30)?;

            assert_eq!(map.try_remove(&2)?, Some(20));
            assert_eq!(map.len(), 2);
            assert_eq!(map.get(&2), None);
            assert_eq!(map.get(&1), Some(&10));
            assert_eq!(map.get(&3), Some(&30));

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_remove_all_verify_memory() {
        let pool = setup_pool();
        let before = pool.used_bytes().unwrap();

        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();
            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;
            map.try_insert(3, 30)?;

            map.try_remove(&1)?;
            map.try_remove(&2)?;
            map.try_remove(&3)?;

            let after = pool.used_bytes().unwrap();
            assert_eq!(
                after, before,
                "Memory should be freed after removing all nodes"
            );

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_remove_from_various_positions() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();
            map.try_insert(5, 50)?;
            map.try_insert(3, 30)?;
            map.try_insert(7, 70)?;
            map.try_insert(1, 10)?;
            map.try_insert(9, 90)?;
            map.try_insert(4, 40)?;
            map.try_insert(6, 60)?;

            // Remove leaf
            assert_eq!(map.try_remove(&1)?, Some(10));
            // Remove node with one child
            assert_eq!(map.try_remove(&9)?, Some(90));
            // Remove node with two children
            assert_eq!(map.try_remove(&5)?, Some(50));

            assert_eq!(map.len(), 4);
            assert_eq!(map.get(&3), Some(&30));
            assert_eq!(map.get(&7), Some(&70));
            assert_eq!(map.get(&4), Some(&40));
            assert_eq!(map.get(&6), Some(&60));

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_empty_operations() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::<i32, i32>::new();
            assert_eq!(map.try_remove(&1)?, None);
            assert_eq!(map.len(), 0);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_single_element_operations() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();
            map.try_insert(1, 10)?;

            assert_eq!(map.try_remove(&1)?, Some(10));
            assert_eq!(map.len(), 0);
            assert!(map.is_empty());

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_interleaved_insert_remove() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();

            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;
            map.try_remove(&1)?;
            map.try_insert(3, 30)?;
            map.try_remove(&2)?;
            map.try_insert(4, 40)?;

            assert_eq!(map.len(), 2);
            assert_eq!(map.get(&3), Some(&30));
            assert_eq!(map.get(&4), Some(&40));

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_complex_keys_with_drop() {
        use core::sync::atomic::{AtomicUsize, Ordering};

        static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

        struct DropCounter(i32);
        impl Drop for DropCounter {
            fn drop(&mut self) {
                DROP_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }

        let pool = setup_pool();
        DROP_COUNT.store(0, Ordering::SeqCst);

        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();
            map.try_insert(1, DropCounter(10))?;
            map.try_insert(2, DropCounter(20))?;
            map.try_insert(3, DropCounter(30))?;

            let removed = map.try_remove(&2)?;
            assert!(removed.is_some());
            // removed drops here implicitly at end of scope
            Ok::<(), AllocError>(())
        })
        .unwrap();

        // Verify at least one drop was called (the removed one)
        // Note: Other drops happen when map is dropped, but we can't check that
        // without causing allocation issues
        assert!(DROP_COUNT.load(Ordering::SeqCst) >= 1);
    }
}
