//! Fixed-point value handling.

use lp_math::fixed::Fixed;

use crate::value::RuntimeError;

/// Construct a Fixed value.
pub fn fixed(value: Fixed) -> crate::value::LpValue {
    crate::value::LpValue::Fixed(value)
}

/// Get value as Fixed, if it is a Fixed scalar.
pub fn as_fixed(value: &crate::value::LpValue) -> Result<Fixed, RuntimeError> {
    match value {
        crate::value::LpValue::Fixed(f) => Ok(*f),
        _ => Err(RuntimeError::NotAScalar),
    }
}
