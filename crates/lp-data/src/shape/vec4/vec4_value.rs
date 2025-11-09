//! Vec4 value wrapper.

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
use crate::value::{LpValue, RuntimeError};
use lp_math::fixed::Fixed;

/// Type-safe wrapper for Vec4 values.
pub struct LpVec4(Fixed, Fixed, Fixed, Fixed);

impl LpVec4 {
    /// Create a new LpVec4 from x, y, z, and w components.
    pub fn new(x: Fixed, y: Fixed, z: Fixed, w: Fixed) -> Self {
        Self(x, y, z, w)
    }

    /// Get the x component.
    pub fn x(&self) -> Fixed {
        self.0
    }

    /// Get the y component.
    pub fn y(&self) -> Fixed {
        self.1
    }

    /// Get the z component.
    pub fn z(&self) -> Fixed {
        self.2
    }

    /// Get the w component.
    pub fn w(&self) -> Fixed {
        self.3
    }

    /// Get mutable references to components.
    pub fn as_tuple_mut(&mut self) -> (&mut Fixed, &mut Fixed, &mut Fixed, &mut Fixed) {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3)
    }

    /// Convert to LpValue.
    pub fn into_lp_value(self) -> LpValue {
        LpValue::Vec4(self.0, self.1, self.2, self.3)
    }

    /// Create from LpValue (with type checking).
    pub fn from_lp_value(value: LpValue) -> Result<Self, RuntimeError> {
        match value {
            LpValue::Vec4(x, y, z, w) => Ok(Self(x, y, z, w)),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Vec4",
                actual: "other",
            }),
        }
    }
}

impl LpValueTrait for LpVec4 {
    fn shape(&self) -> &ShapeRef {
        static DEFAULT: ShapeRef = ShapeRef::vec4_default();
        &DEFAULT
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Vec4
    }
}

impl core::fmt::Debug for LpVec4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LpVec4")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .field(&self.3)
            .finish()
    }
}

impl Clone for LpVec4 {
    fn clone(&self) -> Self {
        Self(self.0, self.1, self.2, self.3)
    }
}
