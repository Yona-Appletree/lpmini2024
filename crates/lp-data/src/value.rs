//! Runtime error types for lp-data.

#[cfg(feature = "alloc")]
extern crate alloc;

/// Runtime errors that can occur when working with lp-data values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    /// Field not found in a record.
    FieldNotFound {
        /// Name of the record type.
        #[cfg(feature = "alloc")]
        record_name: alloc::string::String,
        #[cfg(not(feature = "alloc"))]
        record_name: &'static str,
        /// Name of the field that was not found.
        #[cfg(feature = "alloc")]
        field_name: alloc::string::String,
        #[cfg(not(feature = "alloc"))]
        field_name: &'static str,
    },

    /// Type mismatch when setting a field value.
    TypeMismatch {
        /// Expected type name.
        #[cfg(feature = "alloc")]
        expected: alloc::string::String,
        #[cfg(not(feature = "alloc"))]
        expected: &'static str,
        /// Actual type name.
        #[cfg(feature = "alloc")]
        actual: alloc::string::String,
        #[cfg(not(feature = "alloc"))]
        actual: &'static str,
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
                #[cfg(feature = "alloc")]
                return write!(
                    f,
                    "Field '{}' not found in record '{}'",
                    field_name, record_name
                );
                #[cfg(not(feature = "alloc"))]
                return write!(
                    f,
                    "Field '{}' not found in record '{}'",
                    field_name, record_name
                );
            }
            RuntimeError::TypeMismatch { expected, actual } => {
                #[cfg(feature = "alloc")]
                return write!(
                    f,
                    "Type mismatch: expected '{}', got '{}'",
                    expected, actual
                );
                #[cfg(not(feature = "alloc"))]
                return write!(
                    f,
                    "Type mismatch: expected '{}', got '{}'",
                    expected, actual
                );
            }
            RuntimeError::IndexOutOfBounds { index, len } => {
                write!(f, "Index {} out of bounds for length {}", index, len)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RuntimeError {}
