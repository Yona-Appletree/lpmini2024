//! Int32 value wrapper.

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
use crate::value::{LpValue, RuntimeError};
use serde::{Deserialize, Serialize};

/// Type-safe wrapper for Int32 values.
#[derive(PartialEq, Serialize, Deserialize)]
pub struct LpInt32(pub i32);

impl LpInt32 {
    /// Create a new LpInt32 from an i32 value.
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    /// Get the underlying i32 value.
    pub fn as_int32(&self) -> i32 {
        self.0
    }

    /// Get a mutable reference to the underlying i32 value.
    pub fn as_int32_mut(&mut self) -> &mut i32 {
        &mut self.0
    }

    /// Convert to LpValue.
    pub fn into_lp_value(self) -> LpValue {
        LpValue::Int32(self.0)
    }

    /// Create from LpValue (with type checking).
    pub fn from_lp_value(value: LpValue) -> Result<Self, RuntimeError> {
        match value {
            LpValue::Int32(i) => Ok(Self(i)),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Int32",
                actual: "other",
            }),
        }
    }
}

impl LpValueTrait for LpInt32 {
    fn shape(&self) -> &ShapeRef {
        static SHAPE: crate::shape::int32::StaticInt32Shape =
            crate::shape::int32::StaticInt32Shape::default();
        static DEFAULT: ShapeRef =
            ShapeRef::Int32(crate::shape::shape_ref::Int32ShapeRef::Static(&SHAPE));
        &DEFAULT
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Int32
    }
}

impl core::fmt::Debug for LpInt32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl Clone for LpInt32 {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
