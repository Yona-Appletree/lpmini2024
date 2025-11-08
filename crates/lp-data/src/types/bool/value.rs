//! Boolean value handling.

use crate::value::RuntimeError;

/// Construct a Bool value.
pub fn bool(value: bool) -> crate::value::LpValue {
    crate::value::LpValue::Bool(value)
}

/// Get value as Bool, if it is a Bool scalar.
pub fn as_bool(value: &crate::value::LpValue) -> Result<bool, RuntimeError> {
    match value {
        crate::value::LpValue::Bool(b) => Ok(*b),
        _ => Err(RuntimeError::NotAScalar),
    }
}
