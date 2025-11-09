//! Vec3 value wrapper.

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
use crate::value::{LpValue, RuntimeError};
use lp_math::fixed::Fixed;

/// Type-safe wrapper for Vec3 values.
pub struct LpVec3(Fixed, Fixed, Fixed);

impl LpVec3 {
    /// Create a new LpVec3 from x, y, and z components.
    pub fn new(x: Fixed, y: Fixed, z: Fixed) -> Self {
        Self(x, y, z)
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

    /// Get mutable references to components.
    pub fn as_tuple_mut(&mut self) -> (&mut Fixed, &mut Fixed, &mut Fixed) {
        (&mut self.0, &mut self.1, &mut self.2)
    }

    /// Convert to LpValue.
    pub fn into_lp_value(self) -> LpValue {
        LpValue::Vec3(self.0, self.1, self.2)
    }

    /// Create from LpValue (with type checking).
    pub fn from_lp_value(value: LpValue) -> Result<Self, RuntimeError> {
        match value {
            LpValue::Vec3(x, y, z) => Ok(Self(x, y, z)),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Vec3",
                actual: "other",
            }),
        }
    }
}

impl LpValueTrait for LpVec3 {
    fn shape(&self) -> &ShapeRef {
        static DEFAULT: ShapeRef = ShapeRef::vec3_default();
        &DEFAULT
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Vec3
    }
}

impl core::fmt::Debug for LpVec3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LpVec3")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

impl Clone for LpVec3 {
    fn clone(&self) -> Self {
        Self(self.0, self.1, self.2)
    }
}
