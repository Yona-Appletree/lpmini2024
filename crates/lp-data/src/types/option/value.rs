//! Option value handling.

use lp_pool::collections::LpBox;

use crate::shape::shape_ref::ShapeRef;
use crate::value::RuntimeError;

/// Option value storage.
pub struct OptionValue {
    pub shape: ShapeRef,
    pub value: Option<LpBox<crate::value::LpValue>>,
}

impl OptionValue {
    /// Create an Option::None value.
    pub fn try_none(shape: ShapeRef) -> Result<Self, lp_pool::error::AllocError> {
        Ok(Self { shape, value: None })
    }

    /// Create an Option::Some value.
    pub fn try_some(
        shape: ShapeRef,
        value: crate::value::LpValue,
    ) -> Result<Self, lp_pool::error::AllocError> {
        let boxed = LpBox::try_new(value)?;
        Ok(Self {
            shape,
            value: Some(boxed),
        })
    }

    /// Check if the option is Some.
    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }

    /// Check if the option is None.
    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }

    /// Unwrap the option, returning the inner value.
    pub fn try_unwrap(&self) -> Result<&crate::value::LpValue, RuntimeError> {
        self.value
            .as_ref()
            .map(|v| v.as_ref())
            .ok_or(RuntimeError::OptionIsNone)
    }

    /// Unwrap the option mutably, returning the inner value.
    pub fn try_unwrap_mut(&mut self) -> Result<&mut crate::value::LpValue, RuntimeError> {
        self.value
            .as_mut()
            .map(|v| v.as_mut())
            .ok_or(RuntimeError::OptionIsNone)
    }
}
