//! Array value handling.

use lp_pool::collections::vec::LpVec;
use lp_pool::error::AllocError;

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
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

impl LpValueTrait for ArrayValue {
    fn shape(&self) -> &ShapeRef {
        &self.shape
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Array
    }
}

impl core::fmt::Debug for ArrayValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ArrayValue")
            .field("shape", &self.shape)
            .field("len", &self.values.len())
            .finish()
    }
}

// Note: Clone is not implemented because ShapeRef contains references
// and cannot be cloned. If cloning is needed, use a different approach.

impl crate::shape::value::ArrayValue for ArrayValue {
    fn get_element(&self, index: usize) -> Result<&dyn LpValueTrait, RuntimeError> {
        let value = self.get(index)?;
        Ok(value as &dyn LpValueTrait)
    }

    fn get_element_mut(&mut self, index: usize) -> Result<&mut dyn LpValueTrait, RuntimeError> {
        let value = self.get_mut(index)?;
        Ok(value as &mut dyn LpValueTrait)
    }

    fn set_element(&mut self, index: usize, value: LpValue) -> Result<(), RuntimeError> {
        self.try_set(index, value)
    }

    fn push_element(&mut self, value: LpValue) -> Result<(), RuntimeError> {
        self.try_push(value)
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}
