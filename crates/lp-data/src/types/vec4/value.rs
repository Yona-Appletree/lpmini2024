//! Vec4 value handling.

use lp_math::fixed::Fixed;

use crate::value::RuntimeError;

/// Construct a Vec4 value.
pub fn vec4(x: Fixed, y: Fixed, z: Fixed, w: Fixed) -> crate::value::LpValue {
    crate::value::LpValue::Vec4(x, y, z, w)
}

/// Get value as Vec4, if it is a Vec4.
pub fn as_vec4(
    value: &crate::value::LpValue,
) -> Result<(Fixed, Fixed, Fixed, Fixed), RuntimeError> {
    match value {
        crate::value::LpValue::Vec4(x, y, z, w) => Ok((*x, *y, *z, *w)),
        _ => Err(RuntimeError::NotAVector),
    }
}
