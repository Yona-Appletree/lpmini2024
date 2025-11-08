//! Array value handling.

use lp_pool::collections::vec::LpVec;
use lp_pool::error::AllocError;

use crate::shape::shape_ref::ShapeRef;
use crate::value::{LpValue, RuntimeError};

/// Array value storage.
pub struct ArrayValue {
    pub shape: ShapeRef,
    pub values: LpVec<LpValue>,
}

impl ArrayValue {
    /// Create a new empty array.
    pub fn try_new(shape: ShapeRef, capacity: usize) -> Result<Self, AllocError> {
        let mut values = LpVec::new();
        if capacity > 0 {
            values.try_reserve(capacity)?;
        }
        Ok(Self { shape, values })
    }

    /// Get an element by index.
    pub fn get(&self, index: usize) -> Result<&LpValue, RuntimeError> {
        self.values
            .get(index)
            .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                array_len: self.values.len(),
                index,
            })
    }

    /// Get a mutable element by index.
    pub fn get_mut(&mut self, index: usize) -> Result<&mut LpValue, RuntimeError> {
        let len = self.values.len();
        self.values
            .get_mut(index)
            .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                array_len: len,
                index,
            })
    }

    /// Set an element value.
    pub fn try_set(&mut self, index: usize, value: LpValue) -> Result<(), RuntimeError> {
        let element = self.get_mut(index)?;
        *element = value;
        Ok(())
    }

    /// Push a value to the array.
    pub fn try_push(&mut self, value: LpValue) -> Result<(), RuntimeError> {
        self.values
            .try_push(value)
            .map_err(RuntimeError::AllocError)?;
        Ok(())
    }

    /// Get the length of the array.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Check if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
