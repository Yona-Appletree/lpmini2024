//! Option value handling.

use lp_pool::collections::LpBox;
use lp_pool::error::AllocError;

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
use crate::value::{LpValue, RuntimeError};

/// Option value storage.
pub struct OptionValue {
    pub shape: ShapeRef,
    pub value: Option<LpBox<LpValue>>,
}

impl OptionValue {
    /// Create an Option::None value.
    pub fn try_none(shape: ShapeRef) -> Result<Self, AllocError> {
        Ok(Self { shape, value: None })
    }

    /// Create an Option::Some value.
    pub fn try_some(shape: ShapeRef, value: LpValue) -> Result<Self, AllocError> {
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
    pub fn try_unwrap(&self) -> Result<&LpValue, RuntimeError> {
        self.value
            .as_ref()
            .map(|v| v.as_ref())
            .ok_or(RuntimeError::OptionIsNone)
    }

    /// Unwrap the option mutably, returning the inner value.
    pub fn try_unwrap_mut(&mut self) -> Result<&mut LpValue, RuntimeError> {
        self.value
            .as_mut()
            .map(|v| v.as_mut())
            .ok_or(RuntimeError::OptionIsNone)
    }
}

impl LpValueTrait for OptionValue {
    fn shape(&self) -> &ShapeRef {
        &self.shape
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Option
    }
}

impl core::fmt::Debug for OptionValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OptionValue")
            .field("shape", &self.shape)
            .field("is_some", &self.value.is_some())
            .finish()
    }
}

// Note: Clone is not implemented because ShapeRef contains references
// and cannot be cloned. If cloning is needed, use a different approach.

impl crate::shape::value::OptionValue for OptionValue {
    fn is_some(&self) -> bool {
        self.value.is_some()
    }

    fn unwrap(&self) -> Result<&dyn LpValueTrait, RuntimeError> {
        let value = self.try_unwrap()?;
        Ok(value as &dyn LpValueTrait)
    }

    fn unwrap_mut(&mut self) -> Result<&mut dyn LpValueTrait, RuntimeError> {
        let value = self.try_unwrap_mut()?;
        Ok(value as &mut dyn LpValueTrait)
    }
}
