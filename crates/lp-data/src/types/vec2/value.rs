//! Vec2 value handling.

use lp_math::fixed::Fixed;

use crate::value::RuntimeError;

/// Construct a Vec2 value.
pub fn vec2(x: Fixed, y: Fixed) -> crate::value::LpValue {
    crate::value::LpValue::Vec2(x, y)
}

/// Get value as Vec2, if it is a Vec2.
pub fn as_vec2(value: &crate::value::LpValue) -> Result<(Fixed, Fixed), RuntimeError> {
    match value {
        crate::value::LpValue::Vec2(x, y) => Ok((*x, *y)),
        _ => Err(RuntimeError::NotAVector),
    }
}
