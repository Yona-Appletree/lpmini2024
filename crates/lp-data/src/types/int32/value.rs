//! Int32 value handling.

use crate::value::RuntimeError;

/// Construct an Int32 value.
pub fn int32(value: i32) -> crate::value::LpValue {
    crate::value::LpValue::Int32(value)
}

/// Get value as Int32, if it is an Int32 scalar.
pub fn as_int32(value: &crate::value::LpValue) -> Result<i32, RuntimeError> {
    match value {
        crate::value::LpValue::Int32(i) => Ok(*i),
        _ => Err(RuntimeError::NotAScalar),
    }
}
