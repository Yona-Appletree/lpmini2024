//! Runtime error types for lp-data.

use lp_pool::LpString;

/// Runtime errors that can occur when working with lp-data values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    /// Field not found in a record.
    FieldNotFound {
        /// Name of the record type.
        record_name: LpString,
        /// Name of the field that was not found.
        field_name: LpString,
    },

    /// Type mismatch when setting a field value.
    TypeMismatch {
        /// Expected type name.
        expected: LpString,
        /// Actual type name.
        actual: LpString,
    },

    /// Index out of bounds.
    IndexOutOfBounds {
        /// The index that was accessed.
        index: usize,
        /// The length of the collection.
        len: usize,
    },
}

impl RuntimeError {
    /// Helper function to create a FieldNotFound error from static strings.
    /// Panics if allocation fails (allocation failures in error contexts are unexpected).
    pub fn field_not_found(record_name: &str, field_name: &str) -> Self {
        RuntimeError::FieldNotFound {
            record_name: LpString::try_from_str(record_name)
                .expect("Failed to allocate string for error message"),
            field_name: LpString::try_from_str(field_name)
                .expect("Failed to allocate string for error message"),
        }
    }

    /// Helper function to create a TypeMismatch error from static strings.
    /// Panics if allocation fails (allocation failures in error contexts are unexpected).
    pub fn type_mismatch(expected: &str, actual: &str) -> Self {
        RuntimeError::TypeMismatch {
            expected: LpString::try_from_str(expected)
                .expect("Failed to allocate string for error message"),
            actual: LpString::try_from_str(actual)
                .expect("Failed to allocate string for error message"),
        }
    }
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
                    field_name.as_str(),
                    record_name.as_str()
                )
            }
            RuntimeError::TypeMismatch { expected, actual } => {
                write!(
                    f,
                    "Type mismatch: expected '{}', got '{}'",
                    expected.as_str(),
                    actual.as_str()
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
