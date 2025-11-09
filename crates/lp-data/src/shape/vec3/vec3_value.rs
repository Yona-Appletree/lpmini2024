//! Vec3 value wrapper.

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
use crate::value::{LpValue, RuntimeError};
use lp_math::fixed::{Fixed, Vec3};

/// Type-safe wrapper for Vec3 values.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LpVec3(pub Vec3);

impl LpVec3 {
    /// Create a new LpVec3 from x, y, and z components.
    pub fn new(x: Fixed, y: Fixed, z: Fixed) -> Self {
        Self(Vec3::new(x, y, z))
    }

    /// Get the x component.
    pub fn x(&self) -> Fixed {
        self.0.x()
    }

    /// Get the y component.
    pub fn y(&self) -> Fixed {
        self.0.y()
    }

    /// Get the z component.
    pub fn z(&self) -> Fixed {
        self.0.z()
    }

    /// Get mutable references to components.
    pub fn as_tuple_mut(&mut self) -> (&mut Fixed, &mut Fixed, &mut Fixed) {
        (&mut self.0.x, &mut self.0.y, &mut self.0.z)
    }

    /// Convert to LpValue.
    pub fn into_lp_value(self) -> LpValue {
        LpValue::Vec3(self.0.x(), self.0.y(), self.0.z())
    }

    /// Create from LpValue (with type checking).
    pub fn from_lp_value(value: LpValue) -> Result<Self, RuntimeError> {
        match value {
            LpValue::Vec3(x, y, z) => Ok(Self(Vec3::new(x, y, z))),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "Vec3",
                actual: "other",
            }),
        }
    }
}

impl LpValueTrait for LpVec3 {
    fn shape(&self) -> &ShapeRef {
        static SHAPE: crate::shape::vec3::StaticVec3Shape =
            crate::shape::vec3::StaticVec3Shape::default();
        static DEFAULT: ShapeRef =
            ShapeRef::Vec3(crate::shape::shape_ref::Vec3ShapeRef::Static(&SHAPE));
        &DEFAULT
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Vec3
    }
}
