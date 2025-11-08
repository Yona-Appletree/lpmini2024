use super::vec::LpVec;

/// Iterator over LpVec
pub struct LpVecIter<'a, T> {
    pub(crate) vec: &'a LpVec<T>,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl<'a, T> Iterator for LpVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let item = self.vec.get(self.start);
            self.start += 1;
            item
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.start);
        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for LpVecIter<'a, T> {
    fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

impl<'a, T> DoubleEndedIterator for LpVecIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= 1;
            self.vec.get(self.end)
        } else {
            None
        }
    }
}

/// Mutable iterator over LpVec
pub struct LpVecIterMut<'a, T> {
    pub(crate) vec: *mut LpVec<T>,
    pub(crate) index: usize,
    pub(crate) len: usize,
    pub(crate) _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for LpVecIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            unsafe {
                let vec = &mut *self.vec;
                let item = vec.get_mut(self.index);
                self.index += 1;
                // Safety: We have exclusive access to vec, and we're returning non-overlapping refs
                item.map(|r| &mut *(r as *mut T))
            }
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for LpVecIterMut<'a, T> {
    fn len(&self) -> usize {
        self.len.saturating_sub(self.index)
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
    fn test_vec_iter() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;

            let sum: i32 = vec.iter().sum();
            assert_eq!(sum, 6);

            let collected = allow_global_alloc(|| vec.iter().collect::<alloc::vec::Vec<_>>());
            let expected = allow_global_alloc(|| alloc::vec![&1, &2, &3]);
            assert_eq!(collected, expected);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_iter_mut() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;

            for val in vec.iter_mut() {
                *val *= 10;
            }

            assert_eq!(vec[0], 10);
            assert_eq!(vec[1], 20);
            assert_eq!(vec[2], 30);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_iter_rev() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;

            let collected =
                allow_global_alloc(|| vec.iter().rev().copied().collect::<alloc::vec::Vec<_>>());
            let expected = allow_global_alloc(|| alloc::vec![3, 2, 1]);
            assert_eq!(collected, expected);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_iterator_exactsize() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;

            let mut iter = vec.iter();
            assert_eq!(iter.len(), 3);

            iter.next();
            assert_eq!(iter.len(), 2);

            iter.next();
            assert_eq!(iter.len(), 1);

            iter.next();
            assert_eq!(iter.len(), 0);

            assert_eq!(iter.next(), None);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_iterator_double_ended() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(1)?;
            vec.try_push(2)?;
            vec.try_push(3)?;
            vec.try_push(4)?;

            let mut iter = vec.iter();

            // Alternate between next and next_back
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next_back(), Some(&4));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next_back(), Some(&3));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next_back(), None);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_iterator_empty() {
        let pool = setup_pool();
        pool.run(|| {
            let vec = LpVec::<i32>::new();
            let mut iter = vec.iter();

            assert_eq!(iter.len(), 0);
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next_back(), None);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_vec_iterator_single_element() {
        let pool = setup_pool();
        pool.run(|| {
            let mut vec = LpVec::new();
            vec.try_push(42)?;

            let mut iter = vec.iter();
            assert_eq!(iter.len(), 1);
            assert_eq!(iter.next(), Some(&42));
            assert_eq!(iter.len(), 0);
            assert_eq!(iter.next(), None);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}
