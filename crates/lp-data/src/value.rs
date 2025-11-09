//! Runtime error types for lp-data.

extern crate alloc;

use alloc::string::String;

/// Runtime errors that can occur when working with lp-data values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    /// Field not found in a record.
    FieldNotFound {
        /// Name of the record type.
        record_name: String,
        /// Name of the field that was not found.
        field_name: String,
    },

    /// Type mismatch when setting a field value.
    TypeMismatch {
        /// Expected type name.
        expected: String,
        /// Actual type name.
        actual: String,
    },

    /// Index out of bounds.
    IndexOutOfBounds {
        /// The index that was accessed.
        index: usize,
        /// The length of the collection.
        len: usize,
    },
}

#[cfg(feature = "std")]
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::FieldNotFound {
                record_name,
                field_name,
            } => {
                write!(
                    f,
                    "Field '{}' not found in record '{}'",
                    field_name, record_name
                )
            }
            RuntimeError::TypeMismatch { expected, actual } => {
                write!(
                    f,
                    "Type mismatch: expected '{}', got '{}'",
                    expected, actual
                )
            }
            RuntimeError::IndexOutOfBounds { index, len } => {
                write!(f, "Index {} out of bounds for length {}", index, len)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RuntimeError {}
