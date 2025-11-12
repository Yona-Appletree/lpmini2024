//! Dynamic enum struct value implementation.

use super::enum_struct_dyn::EnumStructShapeDyn;
use super::enum_struct_value::EnumStructValue;
use crate::kind::enum_struct::enum_struct_shape::EnumStructShape;
use crate::kind::shape::LpShape;
use crate::kind::value::{LpValue, LpValueBox, LpValueRef, LpValueRefMut};
use crate::RuntimeError;

/// Dynamic enum struct value.
///
/// Stores a single active variant and its value using `LpValueBox`.
pub struct EnumStructValueDyn {
    /// The shape of this enum struct.
    shape: EnumStructShapeDyn,

    /// Index of the active variant.
    variant_index: usize,

    /// Value of the active variant.
    value: LpValueBox,
}

impl EnumStructValueDyn {
    /// Create a new dynamic enum struct value.
    pub fn new(shape: EnumStructShapeDyn, variant_index: usize, value: LpValueBox) -> Self {
        Self {
            shape,
            variant_index,
            value,
        }
    }

    fn lp_value_ref(value: &LpValueBox) -> LpValueRef<'_> {
        match value {
            LpValueBox::Fixed(boxed) => LpValueRef::Fixed(boxed.as_ref()),
            LpValueBox::Int32(boxed) => LpValueRef::Int32(boxed.as_ref()),
            LpValueBox::Bool(boxed) => LpValueRef::Bool(boxed.as_ref()),
            LpValueBox::Vec2(boxed) => LpValueRef::Vec2(boxed.as_ref()),
            LpValueBox::Vec3(boxed) => LpValueRef::Vec3(boxed.as_ref()),
            LpValueBox::Vec4(boxed) => LpValueRef::Vec4(boxed.as_ref()),
            LpValueBox::Record(boxed) => LpValueRef::Record(boxed.as_ref()),
            LpValueBox::EnumUnit(boxed) => LpValueRef::EnumUnit(boxed.as_ref()),
            LpValueBox::EnumStruct(boxed) => LpValueRef::EnumStruct(boxed.as_ref()),
            LpValueBox::Array(boxed) => LpValueRef::Array(boxed.as_ref()),
        }
    }

    fn lp_value_ref_mut(value: &mut LpValueBox) -> LpValueRefMut<'_> {
        match value {
            LpValueBox::Fixed(boxed) => LpValueRefMut::Fixed(boxed.as_mut()),
            LpValueBox::Int32(boxed) => LpValueRefMut::Int32(boxed.as_mut()),
            LpValueBox::Bool(boxed) => LpValueRefMut::Bool(boxed.as_mut()),
            LpValueBox::Vec2(boxed) => LpValueRefMut::Vec2(boxed.as_mut()),
            LpValueBox::Vec3(boxed) => LpValueRefMut::Vec3(boxed.as_mut()),
            LpValueBox::Vec4(boxed) => LpValueRefMut::Vec4(boxed.as_mut()),
            LpValueBox::Record(boxed) => LpValueRefMut::Record(boxed.as_mut()),
            LpValueBox::EnumUnit(boxed) => LpValueRefMut::EnumUnit(boxed.as_mut()),
            LpValueBox::EnumStruct(boxed) => LpValueRefMut::EnumStruct(boxed.as_mut()),
            LpValueBox::Array(boxed) => LpValueRefMut::Array(boxed.as_mut()),
        }
    }
}

impl LpValue for EnumStructValueDyn {
    fn shape(&self) -> &dyn LpShape {
        &self.shape
    }
}

impl EnumStructValue for EnumStructValueDyn {
    fn shape(&self) -> &dyn EnumStructShape {
        &self.shape
    }

    fn variant_index(&self) -> usize {
        self.variant_index
    }

    fn variant_value(&self) -> Result<LpValueRef<'_>, RuntimeError> {
        let len = self.shape.variant_count();
        if self.variant_index >= len {
            return Err(RuntimeError::IndexOutOfBounds {
                index: self.variant_index,
                len,
            });
        }
        Ok(Self::lp_value_ref(&self.value))
    }

    fn variant_value_mut(&mut self) -> Result<LpValueRefMut<'_>, RuntimeError> {
        let len = self.shape.variant_count();
        if self.variant_index >= len {
            return Err(RuntimeError::IndexOutOfBounds {
                index: self.variant_index,
                len,
            });
        }
        Ok(Self::lp_value_ref_mut(&mut self.value))
    }
}
