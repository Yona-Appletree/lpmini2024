use alloc::boxed::Box;

use crate::kind::value::{LpValueBox, LpValueRef, LpValueRefMut};
use crate::kind::{ArrayShape, LpValue};
use crate::RuntimeError;

/// Trait for array values that have elements.
pub trait ArrayValue: LpValue {
    fn shape(&self) -> &dyn ArrayShape;

    /// Get an element value by index.
    ///
    /// Returns the value at the given index.
    fn get_element(&self, index: usize) -> Result<LpValueRef<'_>, RuntimeError>;

    /// Get an element value by index (mutable).
    ///
    /// Returns the value at the given index.
    fn get_element_mut(&mut self, index: usize) -> Result<LpValueRefMut<'_>, RuntimeError>;

    /// Get the length of this array.
    fn len(&self) -> usize;

    /// Check if this array is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Push a new element to the end of this array (for dynamic arrays).
    ///
    /// Validates that the element's shape matches the array's element shape.
    fn push(&mut self, value: LpValueBox) -> Result<(), RuntimeError>;

    /// Remove an element at the given index (for dynamic arrays).
    fn remove(&mut self, index: usize) -> Result<(), RuntimeError>;
}

impl From<Box<dyn ArrayValue>> for LpValueBox {
    fn from(value: Box<dyn ArrayValue>) -> Self {
        LpValueBox::Array(value)
    }
}

impl<'a> LpValueRef<'a> {
    /// Try to get a reference to the value as ArrayValue.
    pub fn as_array(&self) -> Option<&'a dyn ArrayValue> {
        match self {
            LpValueRef::Dec32(_) => None,
            LpValueRef::Int32(_) => None,
            LpValueRef::Bool(_) => None,
            LpValueRef::Vec2(_) => None,
            LpValueRef::Vec3(_) => None,
            LpValueRef::Vec4(_) => None,
            LpValueRef::Mat3(_) => None,
            LpValueRef::Record(_) => None,
            LpValueRef::EnumUnit(_) => None,
            LpValueRef::EnumStruct(_) => None,
            LpValueRef::Array(v) => Some(*v),
            LpValueRef::Option(_) => None,
        }
    }
}

impl<'a> LpValueRefMut<'a> {
    /// Try to get a mutable reference to the value as ArrayValue.
    pub fn as_array_mut(&mut self) -> Option<&mut dyn ArrayValue> {
        match self {
            LpValueRefMut::Dec32(_) => None,
            LpValueRefMut::Int32(_) => None,
            LpValueRefMut::Bool(_) => None,
            LpValueRefMut::Vec2(_) => None,
            LpValueRefMut::Vec3(_) => None,
            LpValueRefMut::Vec4(_) => None,
            LpValueRefMut::Mat3(_) => None,
            LpValueRefMut::Record(_) => None,
            LpValueRefMut::EnumUnit(_) => None,
            LpValueRefMut::EnumStruct(_) => None,
            LpValueRefMut::Array(v) => Some(*v),
            LpValueRefMut::Option(_) => None,
        }
    }
}
