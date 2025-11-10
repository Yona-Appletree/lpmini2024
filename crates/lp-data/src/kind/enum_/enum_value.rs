//! Value implementation for Enum.

use lp_pool::LpBoxDyn;

use crate::kind::value::{LpValueBox, LpValueRef, LpValueRefMut};
use crate::kind::{EnumShape, LpValue};
use crate::RuntimeError;

/// Trait for enum values that have variants.
pub trait EnumValue: LpValue {
    fn shape(&self) -> &dyn EnumShape;

    /// Get the index of the current variant.
    ///
    /// Returns the index of the variant that this value represents.
    fn variant_index(&self) -> usize;

    /// Get the name of the current variant.
    ///
    /// Convenience method that uses `variant_index` and `shape().get_variant(index)`.
    fn variant_name(&self) -> Result<&str, RuntimeError> {
        let shape = EnumValue::shape(self);
        let index = self.variant_index();
        shape
            .get_variant(index)
            .map(|v| v.name())
            .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                index,
                len: shape.variant_count(),
            })
    }
}

impl From<LpBoxDyn<dyn EnumValue>> for LpValueBox {
    fn from(value: LpBoxDyn<dyn EnumValue>) -> Self {
        LpValueBox::Enum(value)
    }
}

impl<'a> LpValueRef<'a> {
    /// Try to get a reference to the value as EnumValue.
    pub fn as_enum(&self) -> Option<&'a dyn EnumValue> {
        match self {
            LpValueRef::Fixed(_) => None,
            LpValueRef::Int32(_) => None,
            LpValueRef::Bool(_) => None,
            LpValueRef::Vec2(_) => None,
            LpValueRef::Vec3(_) => None,
            LpValueRef::Vec4(_) => None,
            LpValueRef::Record(_) => None,
            LpValueRef::Enum(v) => Some(*v),
        }
    }
}

impl<'a> LpValueRefMut<'a> {
    /// Try to get a mutable reference to the value as EnumValue.
    pub fn as_enum_mut(&mut self) -> Option<&mut dyn EnumValue> {
        match self {
            LpValueRefMut::Fixed(_) => None,
            LpValueRefMut::Int32(_) => None,
            LpValueRefMut::Bool(_) => None,
            LpValueRefMut::Vec2(_) => None,
            LpValueRefMut::Vec3(_) => None,
            LpValueRefMut::Vec4(_) => None,
            LpValueRefMut::Record(_) => None,
            LpValueRefMut::Enum(v) => Some(*v),
        }
    }
}
