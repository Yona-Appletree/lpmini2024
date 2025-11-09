//! Enum value handling.

use lp_pool::collections::{LpBox, LpString};
use lp_pool::error::AllocError;

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
use crate::value::{LpValue, RuntimeError};

/// Enum value storage.
pub struct EnumValue {
    pub shape: ShapeRef,
    pub variant_name: LpString,
    pub payload: Option<LpBox<LpValue>>,
}

impl EnumValue {
    /// Create a new enum value with a unit variant.
    pub fn try_unit(shape: ShapeRef, variant_name: LpString) -> Result<Self, AllocError> {
        Ok(Self {
            shape,
            variant_name,
            payload: None,
        })
    }

    /// Create a new enum value with a payload.
    pub fn try_with_payload(
        shape: ShapeRef,
        variant_name: LpString,
        payload: LpValue,
    ) -> Result<Self, AllocError> {
        let boxed = LpBox::try_new(payload)?;
        Ok(Self {
            shape,
            variant_name,
            payload: Some(boxed),
        })
    }

    /// Get the variant name.
    pub fn variant_name(&self) -> &str {
        self.variant_name.as_str()
    }

    /// Get the payload if present.
    pub fn payload(&self) -> Option<&LpValue> {
        self.payload.as_ref().map(|p| p.as_ref())
    }

    /// Get the payload mutably if present.
    pub fn payload_mut(&mut self) -> Option<&mut LpValue> {
        self.payload.as_mut().map(|p| p.as_mut())
    }
}

impl LpValueTrait for EnumValue {
    fn shape(&self) -> &ShapeRef {
        &self.shape
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Enum
    }
}

impl core::fmt::Debug for EnumValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EnumValue")
            .field("shape", &self.shape)
            .field("variant_name", &self.variant_name)
            .field("has_payload", &self.payload.is_some())
            .finish()
    }
}

// Note: Clone is not implemented because ShapeRef contains references
// and cannot be cloned. If cloning is needed, use a different approach.

impl crate::shape::value::EnumValue for EnumValue {
    fn variant_index(&self) -> usize {
        // TODO: Need to look up variant index from shape
        0
    }

    fn variant_name(&self) -> Option<&str> {
        Some(self.variant_name.as_str())
    }

    fn value(&self) -> Option<&dyn LpValueTrait> {
        // Convert &LpValue to &dyn LpValueTrait
        // Will fix when LpValue implements LpValueTrait
        None
    }

    fn value_mut(&mut self) -> Option<&mut dyn LpValueTrait> {
        None
    }
}
