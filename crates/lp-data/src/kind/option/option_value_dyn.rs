//! Dynamic Option value implementation.

use super::option_dyn::OptionShapeDyn;
use super::option_value::OptionValue;
use crate::kind::option::option_shape::OptionShape;
use crate::kind::shape::LpShape;
use crate::kind::value::{LpValue, LpValueBox, LpValueRef, LpValueRefMut};
use crate::RuntimeError;

/// Dynamic Option value.
///
/// Stores either Some(value) or None.
pub struct OptionValueDyn {
    /// The shape of this Option.
    shape: OptionShapeDyn,

    /// The value if Some, None if this Option is None.
    value: Option<LpValueBox>,
}

impl OptionValueDyn {
    /// Create a new Option value with Some(value).
    pub fn some(shape: OptionShapeDyn, value: LpValueBox) -> Self {
        Self {
            shape,
            value: Some(value),
        }
    }

    /// Create a new Option value with None.
    pub fn none(shape: OptionShapeDyn) -> Self {
        Self { shape, value: None }
    }

    fn lp_value_ref(value: &LpValueBox) -> LpValueRef<'_> {
        match value {
            LpValueBox::Dec32(boxed) => LpValueRef::Dec32(boxed.as_ref()),
            LpValueBox::Int32(boxed) => LpValueRef::Int32(boxed.as_ref()),
            LpValueBox::Bool(boxed) => LpValueRef::Bool(boxed.as_ref()),
            LpValueBox::Vec2(boxed) => LpValueRef::Vec2(boxed.as_ref()),
            LpValueBox::Vec3(boxed) => LpValueRef::Vec3(boxed.as_ref()),
            LpValueBox::Vec4(boxed) => LpValueRef::Vec4(boxed.as_ref()),
            LpValueBox::Mat3(boxed) => LpValueRef::Mat3(boxed.as_ref()),
            LpValueBox::Record(boxed) => LpValueRef::Record(boxed.as_ref()),
            LpValueBox::EnumUnit(boxed) => LpValueRef::EnumUnit(boxed.as_ref()),
            LpValueBox::EnumStruct(boxed) => LpValueRef::EnumStruct(boxed.as_ref()),
            LpValueBox::Array(boxed) => LpValueRef::Array(boxed.as_ref()),
            LpValueBox::Option(boxed) => LpValueRef::Option(boxed.as_ref()),
        }
    }

    fn lp_value_ref_mut(value: &mut LpValueBox) -> LpValueRefMut<'_> {
        match value {
            LpValueBox::Dec32(boxed) => LpValueRefMut::Dec32(boxed.as_mut()),
            LpValueBox::Int32(boxed) => LpValueRefMut::Int32(boxed.as_mut()),
            LpValueBox::Bool(boxed) => LpValueRefMut::Bool(boxed.as_mut()),
            LpValueBox::Vec2(boxed) => LpValueRefMut::Vec2(boxed.as_mut()),
            LpValueBox::Vec3(boxed) => LpValueRefMut::Vec3(boxed.as_mut()),
            LpValueBox::Vec4(boxed) => LpValueRefMut::Vec4(boxed.as_mut()),
            LpValueBox::Mat3(boxed) => LpValueRefMut::Mat3(boxed.as_mut()),
            LpValueBox::Record(boxed) => LpValueRefMut::Record(boxed.as_mut()),
            LpValueBox::EnumUnit(boxed) => LpValueRefMut::EnumUnit(boxed.as_mut()),
            LpValueBox::EnumStruct(boxed) => LpValueRefMut::EnumStruct(boxed.as_mut()),
            LpValueBox::Array(boxed) => LpValueRefMut::Array(boxed.as_mut()),
            LpValueBox::Option(boxed) => LpValueRefMut::Option(boxed.as_mut()),
        }
    }
}

impl LpValue for OptionValueDyn {
    fn shape(&self) -> &dyn LpShape {
        &self.shape
    }
}

impl OptionValue for OptionValueDyn {
    fn shape(&self) -> &dyn OptionShape {
        &self.shape
    }

    fn is_some(&self) -> bool {
        self.value.is_some()
    }

    fn get_value(&self) -> Result<LpValueRef<'_>, RuntimeError> {
        self.value
            .as_ref()
            .map(Self::lp_value_ref)
            .ok_or(RuntimeError::OptionIsNone)
    }

    fn get_value_mut(&mut self) -> Result<LpValueRefMut<'_>, RuntimeError> {
        self.value
            .as_mut()
            .map(Self::lp_value_ref_mut)
            .ok_or(RuntimeError::OptionIsNone)
    }
}
