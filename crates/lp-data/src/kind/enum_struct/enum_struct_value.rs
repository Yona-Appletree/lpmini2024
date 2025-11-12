//! Union value trait and helper implementations.

use alloc::boxed::Box;

use crate::kind::value::{LpValueBox, LpValueRef, LpValueRefMut};
use crate::kind::{EnumStructShape, LpValue};
use crate::RuntimeError;

/// Trait for enum struct values that carry variant-specific data.
pub trait EnumStructValue: LpValue {
    /// Get the enum struct shape for this value.
    fn shape(&self) -> &dyn EnumStructShape;

    /// Get the index of the active variant.
    fn variant_index(&self) -> usize;

    /// Get the name of the active variant.
    ///
    /// Convenience method that uses `variant_index` and `shape().get_variant(index)`.
    fn variant_name(&self) -> Result<&str, RuntimeError> {
        let shape = EnumStructValue::shape(self);
        let index = self.variant_index();
        shape
            .get_variant(index)
            .map(|variant| variant.name())
            .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                index,
                len: shape.variant_count(),
            })
    }

    /// Get the value of the active variant.
    fn variant_value(&self) -> Result<LpValueRef<'_>, RuntimeError>;

    /// Get the value of the active variant (mutable).
    fn variant_value_mut(&mut self) -> Result<LpValueRefMut<'_>, RuntimeError>;
}

impl From<Box<dyn EnumStructValue>> for LpValueBox {
    fn from(value: Box<dyn EnumStructValue>) -> Self {
        LpValueBox::EnumStruct(value)
    }
}

impl<'a> LpValueRef<'a> {
    /// Try to get a reference to the value as UnionValue.
    pub fn as_union(&self) -> Option<&'a dyn EnumStructValue> {
        match self {
            LpValueRef::EnumStruct(value) => Some(*value),
            _ => None,
        }
    }
}

impl<'a> LpValueRefMut<'a> {
    /// Try to get a mutable reference to the value as UnionValue.
    pub fn as_union_mut(&mut self) -> Option<&mut dyn EnumStructValue> {
        match self {
            LpValueRefMut::EnumStruct(value) => Some(*value),
            _ => None,
        }
    }
}
