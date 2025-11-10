//! Type system for lp-data.
//!
//! This module provides the foundation for the introspectable data system,
//! separating type kinds, shapes (metadata), and values (runtime data).

pub mod kind;
pub mod shape;
pub mod value;

pub mod fixed;
pub mod record;

// Re-export commonly used items
pub use kind::LpKind;
pub use shape::LpShape;
pub use value::LpValue;

// Re-export record traits and metadata from record module
pub use record::record_value::RecordValue;
pub use record::{RecordFieldMeta, RecordFieldShape, RecordShape};
