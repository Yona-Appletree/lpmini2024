//! Boolean value wrapper.

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
use crate::value::{LpValue, RuntimeError};

/// Type-safe wrapper for Bool values.
pub struct LpBool(bool);

impl LpBool {
    /// Create a new LpBool from a bool value.
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    /// Get the underlying bool value.
    pub fn as_bool(&self) -> bool {
        self.0
    }

    /// Get a mutable reference to the underlying bool value.
    pub fn as_bool_mut(&mut self) -> &mut bool {
        &mut self.0
    }

    /// Convert to LpValue.
    pub fn into_lp_value(self) -> LpValue {
        LpValue::Bool(self.0)
    }

    /// Create from LpValue (with type checking).
    pub fn from_lp_value(value: LpValue) -> Result<Self, RuntimeError> {
        match value {
            LpValue::Bool(b) => Ok(Self(b)),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Bool",
                actual: "other",
            }),
        }
    }
}

impl LpValueTrait for LpBool {
    fn shape(&self) -> &ShapeRef {
        static DEFAULT: ShapeRef = ShapeRef::bool_default();
        &DEFAULT
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Bool
    }
}

impl core::fmt::Debug for LpBool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl Clone for LpBool {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
