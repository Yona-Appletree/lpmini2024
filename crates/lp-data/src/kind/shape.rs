//! Shape system for lp-data types.
//!
//! Shapes represent metadata about data structures, including their structure
//! and properties needed for UI generation and introspection.

use super::kind::LpKind;

/// Base trait for all shape types.
///
/// Shapes describe the structure and metadata of data types. They can be
/// either static (compile-time constants) or dynamic (runtime-allocated).
pub trait LpShape {
    /// Get the kind of this shape.
    fn kind(&self) -> LpKind;
}
