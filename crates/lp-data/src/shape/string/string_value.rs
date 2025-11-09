//! String value wrapper.

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
use crate::value::{LpValue, RuntimeError};
use lp_pool::collections::LpString;

/// Type-safe wrapper for String values.
pub struct LpStringValue(LpString);

impl LpStringValue {
    /// Create a new LpStringValue from an LpString.
    pub fn new(value: LpString) -> Self {
        Self(value)
    }

    /// Get the underlying LpString value.
    pub fn as_string(&self) -> &LpString {
        &self.0
    }

    /// Get a mutable reference to the underlying LpString value.
    pub fn as_string_mut(&mut self) -> &mut LpString {
        &mut self.0
    }

    /// Convert to LpValue.
    pub fn into_lp_value(self) -> LpValue {
        LpValue::String(self.0)
    }

    /// Create from LpValue (with type checking).
    pub fn from_lp_value(value: LpValue) -> Result<Self, RuntimeError> {
        match value {
            LpValue::String(s) => Ok(Self(s)),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "String",
                actual: "other",
            }),
        }
    }
}

impl LpValueTrait for LpStringValue {
    fn shape(&self) -> &ShapeRef {
        static SHAPE: crate::shape::string::StaticStringShape =
            crate::shape::string::StaticStringShape::default();
        static DEFAULT: ShapeRef =
            ShapeRef::String(crate::shape::shape_ref::StringShapeRef::Static(&SHAPE));
        &DEFAULT
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::String
    }
}

impl core::fmt::Debug for LpStringValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

// LpString doesn't implement Clone, so we can't implement Clone for LpStringValue
// If cloning is needed, it should be done through the memory pool
