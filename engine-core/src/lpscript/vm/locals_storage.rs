/// Local variable storage for LPS VM
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use super::error::RuntimeError;
use super::locals::LocalType;
use crate::math::Fixed;

/// Storage for local variables with frame-based allocation
///
/// Pre-allocates a large array to support deep call stacks without
/// runtime allocations. Each function call frame uses 32 locals.
pub struct LocalsStorage {
    data: Vec<LocalType>,
    capacity: usize,
}

impl LocalsStorage {
    /// Create new locals storage with the given capacity
    ///
    /// Capacity should be: frame_size * max_call_depth
    /// Typical: 32 locals/frame * 64 max frames = 2048
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::new();
        data.resize(capacity, LocalType::Fixed(Fixed::ZERO));
        
        LocalsStorage { data, capacity }
    }

    /// Initialize locals from program definitions
    ///
    /// Sets up the main frame (first 32 locals) with default values
    pub fn init_from_defs(&mut self, defs: &[super::locals::LocalDef]) {
        for (i, def) in defs.iter().enumerate() {
            if i < self.data.len() {
                self.data[i] = def.ty.clone();
            }
        }
    }

    /// Set an input local (overrides program defaults)
    pub fn set_input(&mut self, idx: usize, value: LocalType) -> Result<(), RuntimeError> {
        if idx >= self.capacity {
            return Err(RuntimeError::LocalOutOfBounds {
                local_idx: idx,
                max: self.capacity,
            });
        }
        self.data[idx] = value;
        Ok(())
    }

    /// Get a local variable by index
    #[inline(always)]
    pub fn get(&self, idx: usize) -> Result<&LocalType, RuntimeError> {
        if idx >= self.capacity {
            return Err(RuntimeError::LocalOutOfBounds {
                local_idx: idx,
                max: self.capacity,
            });
        }
        Ok(&self.data[idx])
    }

    /// Set a local variable by index
    #[inline(always)]
    pub fn set(&mut self, idx: usize, value: LocalType) -> Result<(), RuntimeError> {
        if idx >= self.capacity {
            return Err(RuntimeError::LocalOutOfBounds {
                local_idx: idx,
                max: self.capacity,
            });
        }
        self.data[idx] = value;
        Ok(())
    }

    /// Get mutable reference to locals array (for opcode handlers)
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut Vec<LocalType> {
        &mut self.data
    }

    /// Get immutable reference to locals array (for opcode handlers)
    #[inline(always)]
    pub fn as_slice(&self) -> &[LocalType] {
        &self.data
    }

    /// Get capacity
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpscript::vm::locals::{LocalAccess, LocalDef};
    use crate::math::ToFixed;

    #[test]
    fn test_locals_storage_creation() {
        let storage = LocalsStorage::new(2048);
        assert_eq!(storage.capacity(), 2048);
    }

    #[test]
    fn test_get_set() {
        let mut storage = LocalsStorage::new(64);

        storage
            .set(5, LocalType::Fixed(3.14.to_fixed()))
            .unwrap();
        
        match storage.get(5).unwrap() {
            LocalType::Fixed(val) => assert!((val.to_f32() - 3.14).abs() < 0.01),
            _ => panic!("Expected Fixed type"),
        }
    }

    #[test]
    fn test_set_input() {
        let mut storage = LocalsStorage::new(64);

        storage
            .set_input(10, LocalType::Int32(42))
            .unwrap();
        
        match storage.get(10).unwrap() {
            LocalType::Int32(val) => assert_eq!(*val, 42),
            _ => panic!("Expected Int32 type"),
        }
    }

    #[test]
    fn test_init_from_defs() {
        let mut storage = LocalsStorage::new(64);

        let defs = vec![
            LocalDef::new(
                "x".into(),
                LocalType::Fixed(1.0.to_fixed()),
                LocalAccess::Scratch,
            ),
            LocalDef::new(
                "y".into(),
                LocalType::Int32(42),
                LocalAccess::Scratch,
            ),
        ];

        storage.init_from_defs(&defs);

        match storage.get(0).unwrap() {
            LocalType::Fixed(val) => assert_eq!(val.to_f32(), 1.0),
            _ => panic!("Expected Fixed type"),
        }

        match storage.get(1).unwrap() {
            LocalType::Int32(val) => assert_eq!(*val, 42),
            _ => panic!("Expected Int32 type"),
        }
    }

    #[test]
    fn test_out_of_bounds() {
        let mut storage = LocalsStorage::new(10);

        let result = storage.get(15);
        assert!(matches!(
            result,
            Err(RuntimeError::LocalOutOfBounds {
                local_idx: 15,
                max: 10
            })
        ));

        let result = storage.set(20, LocalType::Fixed(Fixed::ZERO));
        assert!(matches!(
            result,
            Err(RuntimeError::LocalOutOfBounds {
                local_idx: 20,
                max: 10
            })
        ));
    }
}

