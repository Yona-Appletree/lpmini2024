use core::ptr::NonNull;
use crate::error::AllocError;
use crate::memory_pool::with_active_pool;
use core::alloc::Layout;

#[cfg(feature = "alloc-meta")]
use super::super::alloc_meta::{AllocationMeta, record_allocation_meta, remove_allocation_meta};

/// A node in the B-tree
pub struct Node<K, V> {
    key: K,
    value: V,
    left: Option<NonNull<Node<K, V>>>,
    right: Option<NonNull<Node<K, V>>>,
}

impl<K, V> Node<K, V>
where
    K: Ord,
{
    /// Create a new node
    pub fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }
    
    /// Allocate a new node from the pool
    pub fn allocate(key: K, value: V) -> Result<NonNull<Self>, AllocError> {
        let node = Self::new(key, value);
        let layout = Layout::new::<Self>();
        
        #[cfg(feature = "alloc-meta")]
        let meta = AllocationMeta {
            type_name: core::any::type_name::<Self>(),
            scope: None,
        };
        
        let ptr = with_active_pool(|pool| {
            let allocated = pool.allocate(layout)?;
            let ptr = NonNull::new(allocated.as_ptr() as *mut Self).unwrap();
            
            unsafe {
                core::ptr::write(ptr.as_ptr(), node);
            }
            
            #[cfg(feature = "alloc-meta")]
            {
                record_allocation_meta(meta, layout.size());
            }
            
            Ok(ptr)
        })?;
        
        Ok(ptr)
    }
    
    /// Deallocate a node
    pub unsafe fn deallocate(ptr: NonNull<Self>) {
        let layout = Layout::new::<Self>();
        
        #[cfg(feature = "alloc-meta")]
        {
            let meta = AllocationMeta {
                type_name: core::any::type_name::<Self>(),
                scope: None,
            };
            remove_allocation_meta(meta, layout.size());
        }
        
        let _ = with_active_pool(|pool| {
            pool.deallocate(ptr.cast(), layout);
            Ok::<(), AllocError>(())
        });
        
        core::ptr::drop_in_place(ptr.as_ptr());
    }
    
    pub fn key(&self) -> &K {
        &self.key
    }
    
    pub fn value(&self) -> &V {
        &self.value
    }
    
    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }
    
    pub fn left(&self) -> Option<NonNull<Node<K, V>>> {
        self.left
    }
    
    pub fn right(&self) -> Option<NonNull<Node<K, V>>> {
        self.right
    }
    
    pub fn set_left(&mut self, left: Option<NonNull<Node<K, V>>>) {
        self.left = left;
    }
    
    pub fn set_right(&mut self, right: Option<NonNull<Node<K, V>>>) {
        self.right = right;
    }
}

