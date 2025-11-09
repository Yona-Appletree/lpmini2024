//! Vec2 value wrapper.

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
use crate::value::{LpValue, RuntimeError};
use lp_math::fixed::Fixed;

/// Type-safe wrapper for Vec2 values.
pub struct LpVec2(Fixed, Fixed);

impl LpVec2 {
    /// Create a new LpVec2 from x and y components.
    pub fn new(x: Fixed, y: Fixed) -> Self {
        Self(x, y)
    }

    /// Get the x component.
    pub fn x(&self) -> Fixed {
        self.0
    }

    /// Get the y component.
    pub fn y(&self) -> Fixed {
        self.1
    }

    /// Get mutable references to components.
    pub fn as_tuple_mut(&mut self) -> (&mut Fixed, &mut Fixed) {
        (&mut self.0, &mut self.1)
    }

    /// Convert to LpValue.
    pub fn into_lp_value(self) -> LpValue {
        LpValue::Vec2(self.0, self.1)
    }

    /// Create from LpValue (with type checking).
    pub fn from_lp_value(value: LpValue) -> Result<Self, RuntimeError> {
        match value {
            LpValue::Vec2(x, y) => Ok(Self(x, y)),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Vec2",
                actual: "other",
            }),
        }
    }
}

impl LpValueTrait for LpVec2 {
    fn shape(&self) -> &ShapeRef {
        static DEFAULT: ShapeRef = ShapeRef::vec2_default();
        &DEFAULT
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Vec2
    }
}

impl core::fmt::Debug for LpVec2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LpVec2")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl Clone for LpVec2 {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}
