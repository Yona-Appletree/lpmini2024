//! Enum value handling.

use lp_pool::collections::{LpBox, LpString};

use crate::metadata::TypeRef;
use crate::value::RuntimeError;

/// Enum value storage.
pub struct EnumValue {
    pub enum_type: TypeRef,
    pub variant_name: LpString,
    pub payload: Option<LpBox<crate::value::LpValue>>,
}

impl EnumValue {
    /// Create a new enum value with a unit variant.
    pub fn try_unit(
        enum_type: TypeRef,
        variant_name: LpString,
    ) -> Result<Self, lp_pool::error::AllocError> {
        Ok(Self {
            enum_type,
            variant_name,
            payload: None,
        })
    }

    /// Create a new enum value with a payload.
    pub fn try_with_payload(
        enum_type: TypeRef,
        variant_name: LpString,
        payload: crate::value::LpValue,
    ) -> Result<Self, lp_pool::error::AllocError> {
        let boxed = LpBox::try_new(payload)?;
        Ok(Self {
            enum_type,
            variant_name,
            payload: Some(boxed),
        })
    }

    /// Get the variant name.
    pub fn variant_name(&self) -> &str {
        self.variant_name.as_str()
    }

    /// Get the payload if present.
    pub fn payload(&self) -> Option<&crate::value::LpValue> {
        self.payload.as_ref().map(|p| p.as_ref())
    }

    /// Get the payload mutably if present.
    pub fn payload_mut(&mut self) -> Option<&mut crate::value::LpValue> {
        self.payload.as_mut().map(|p| p.as_mut())
    }
}
