//! Tuple value handling.

use lp_pool::collections::vec::LpVec;
use lp_pool::error::AllocError;

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
use crate::value::{LpValue, RuntimeError};

/// Tuple value storage.
pub struct TupleValue {
    pub shape: ShapeRef,
    pub elements: LpVec<LpValue>,
}

impl TupleValue {
    /// Create a new tuple value.
    pub fn try_new(shape: ShapeRef, elements: LpVec<LpValue>) -> Result<Self, AllocError> {
        Ok(Self { shape, elements })
    }

    /// Get an element by index.
    pub fn get(&self, index: usize) -> Result<&LpValue, RuntimeError> {
        self.elements
            .get(index)
            .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                array_len: self.elements.len(),
                index,
            })
    }

    /// Get a mutable element by index.
    pub fn get_mut(&mut self, index: usize) -> Result<&mut LpValue, RuntimeError> {
        let len = self.elements.len();
        self.elements
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

    /// Get the length of the tuple.
    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

impl LpValueTrait for TupleValue {
    fn shape(&self) -> &ShapeRef {
        &self.shape
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Tuple
    }
}

impl core::fmt::Debug for TupleValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TupleValue")
            .field("shape", &self.shape)
            .field("len", &self.elements.len())
            .finish()
    }
}

// Note: Clone is not implemented because ShapeRef contains references
// and cannot be cloned. If cloning is needed, use a different approach.

impl crate::shape::value::TupleValue for TupleValue {
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

    fn len(&self) -> usize {
        self.elements.len()
    }
}
