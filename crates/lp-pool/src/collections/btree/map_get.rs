use super::node::Node;

pub(super) fn get_node<'a, K, V>(node: &'a Node<K, V>, key: &K) -> Option<&'a V>
where
    K: Ord,
{
    match key.cmp(node.key()) {
        core::cmp::Ordering::Equal => Some(node.value()),
        core::cmp::Ordering::Less => node.left().and_then(|left| unsafe {
            let left_node = &*left.as_ptr();
            get_node(left_node, key)
        }),
        core::cmp::Ordering::Greater => node.right().and_then(|right| unsafe {
            let right_node = &*right.as_ptr();
            get_node(right_node, key)
        }),
    }
}

pub(super) fn get_node_mut<'a, K, V>(node: &'a mut Node<K, V>, key: &K) -> Option<&'a mut V>
where
    K: Ord,
{
    match key.cmp(node.key()) {
        core::cmp::Ordering::Equal => Some(node.value_mut()),
        core::cmp::Ordering::Less => node.left().and_then(|left| unsafe {
            let left_node = &mut *left.as_ptr();
            get_node_mut(left_node, key)
        }),
        core::cmp::Ordering::Greater => node.right().and_then(|right| unsafe {
            let right_node = &mut *right.as_ptr();
            get_node_mut(right_node, key)
        }),
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
    fn test_btree_map_get() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();
            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;
            map.try_insert(3, 30)?;

            assert_eq!(map.get(&1), Some(&10));
            assert_eq!(map.get(&2), Some(&20));
            assert_eq!(map.get(&3), Some(&30));
            assert_eq!(map.get(&4), None);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_btree_map_get_mut() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();
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
    fn test_btree_map_get_mut_multiple() {
        let pool = setup_pool();
        pool.run(|| {
            let mut map = crate::collections::LpBTreeMap::new();
            map.try_insert(1, 10)?;
            map.try_insert(2, 20)?;
            map.try_insert(3, 30)?;

            *map.get_mut(&1).unwrap() = 100;
            *map.get_mut(&2).unwrap() = 200;
            *map.get_mut(&3).unwrap() = 300;

            assert_eq!(map.get(&1), Some(&100));
            assert_eq!(map.get(&2), Some(&200));
            assert_eq!(map.get(&3), Some(&300));

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}
