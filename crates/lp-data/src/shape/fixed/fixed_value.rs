//! Fixed-point value wrapper.

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
use crate::value::{LpValue, RuntimeError};
use lp_math::fixed::Fixed;

/// Type-safe wrapper for Fixed values.
pub struct LpFixed(Fixed);

impl LpFixed {
    /// Create a new LpFixed from a Fixed value.
    pub fn new(value: Fixed) -> Self {
        Self(value)
    }

    /// Get the underlying Fixed value.
    pub fn as_fixed(&self) -> Fixed {
        self.0
    }

    /// Get a mutable reference to the underlying Fixed value.
    pub fn as_fixed_mut(&mut self) -> &mut Fixed {
        &mut self.0
    }

    /// Convert to LpValue.
    pub fn into_lp_value(self) -> LpValue {
        LpValue::Fixed(self.0)
    }

    /// Create from LpValue (with type checking).
    pub fn from_lp_value(value: LpValue) -> Result<Self, RuntimeError> {
        match value {
            LpValue::Fixed(f) => Ok(Self(f)),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Fixed",
                actual: "other",
            }),
        }
    }
}

impl LpValueTrait for LpFixed {
    fn shape(&self) -> &ShapeRef {
        static DEFAULT: ShapeRef = ShapeRef::fixed_default();
        &DEFAULT
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Fixed
    }
}

impl core::fmt::Debug for LpFixed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl Clone for LpFixed {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
