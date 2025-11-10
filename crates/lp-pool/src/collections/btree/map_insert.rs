use core::ptr::NonNull;

use super::node::Node;
use crate::error::AllocError;

pub(super) fn insert_node<K, V>(
    node_ptr: NonNull<Node<K, V>>,
    key: K,
    value: V,
) -> Result<(Option<V>, bool), AllocError>
where
    K: Ord,
{
    let node = unsafe { &mut *node_ptr.as_ptr() };
    match key.cmp(node.key()) {
        core::cmp::Ordering::Equal => {
            // Replace existing value
            let old_value = unsafe { core::ptr::replace(node.value_mut(), value) };
            Ok((Some(old_value), false))
        }
        core::cmp::Ordering::Less => {
            if let Some(left) = node.left() {
                insert_node(left, key, value)
            } else {
                let new_node = Node::allocate(key, value)?;
                node.set_left(Some(new_node));
                Ok((None, true))
            }
        }
        core::cmp::Ordering::Greater => {
            if let Some(right) = node.right() {
                insert_node(right, key, value)
            } else {
                let new_node = Node::allocate(key, value)?;
                node.set_right(Some(new_node));
                Ok((None, true))
            }
        }
    }
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
    fn test_btree_map_insert() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();
            assert_eq!(map.try_insert(1, 10)?, None);
            assert_eq!(map.len(), 1);
            assert_eq!(map.get(&1), Some(&10));

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_insert_replace() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();
            assert_eq!(map.try_insert(1, 10)?, None);
            assert_eq!(map.try_insert(1, 20)?, Some(10));
            assert_eq!(map.len(), 1);
            assert_eq!(map.get(&1), Some(&20));

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_string_keys() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();
            let k1 = crate::allow_global_alloc(|| alloc::string::String::from("key1"));
            let k2 = crate::allow_global_alloc(|| alloc::string::String::from("key2"));

            let k1_clone = crate::allow_global_alloc(|| k1.clone());
            let k2_clone = crate::allow_global_alloc(|| k2.clone());
            map.try_insert(k1_clone, 10)?;
            map.try_insert(k2_clone, 20)?;

            let k1_ref = crate::allow_global_alloc(|| k1.clone());
            let k2_ref = crate::allow_global_alloc(|| k2.clone());
            assert_eq!(map.get(&k1_ref), Some(&10));
            assert_eq!(map.get(&k2_ref), Some(&20));

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[cfg(feature = "alloc-meta")]
    #[test]
    fn test_btree_map_with_scope() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new_with_scope(Some("test_scope"));
            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;
            assert_eq!(map.len(), 2);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_duplicate_key_replacement() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();
            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;
            map.try_insert(3, 30)?;

            let old = map.try_insert(2, 200)?;
            assert_eq!(old, Some(20));
            assert_eq!(map.get(&2), Some(&200));
            assert_eq!(map.len(), 3);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_many_insertions() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();

            for i in 0..100 {
                map.try_insert(i, i * 10)?;
            }

            assert_eq!(map.len(), 100);
            for i in 0..100 {
                assert_eq!(map.get(&i), Some(&(i * 10)));
            }

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_degenerate_sorted_insertion() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();

            // Insert in sorted order (creates degenerate tree)
            for i in 0..50 {
                map.try_insert(i, i)?;
            }

            assert_eq!(map.len(), 50);
            for i in 0..50 {
                assert_eq!(map.get(&i), Some(&i));
            }

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_reverse_sorted_insertion() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();

            // Insert in reverse sorted order (creates degenerate tree)
            for i in (0..50).rev() {
                map.try_insert(i, i)?;
            }

            assert_eq!(map.len(), 50);
            for i in 0..50 {
                assert_eq!(map.get(&i), Some(&i));
            }

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}
