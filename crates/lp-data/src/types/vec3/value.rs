//! Vec3 value handling.

use lp_math::fixed::Fixed;

use crate::value::RuntimeError;

/// Construct a Vec3 value.
pub fn vec3(x: Fixed, y: Fixed, z: Fixed) -> crate::value::LpValue {
    crate::value::LpValue::Vec3(x, y, z)
}

/// Get value as Vec3, if it is a Vec3.
pub fn as_vec3(value: &crate::value::LpValue) -> Result<(Fixed, Fixed, Fixed), RuntimeError> {
    match value {
        crate::value::LpValue::Vec3(x, y, z) => Ok((*x, *y, *z)),
        _ => Err(RuntimeError::NotAVector),
    }
}
