//! String value handling.

use lp_pool::collections::string::LpString;

use crate::value::RuntimeError;

/// Construct a String value.
pub fn string(value: LpString) -> crate::value::LpValue {
    crate::value::LpValue::String(value)
}

/// Get value as string slice, if it is a String scalar.
pub fn as_string(value: &crate::value::LpValue) -> Result<&str, RuntimeError> {
    match value {
        crate::value::LpValue::String(s) => Ok(s.as_str()),
        _ => Err(RuntimeError::NotAScalar),
    }
}
