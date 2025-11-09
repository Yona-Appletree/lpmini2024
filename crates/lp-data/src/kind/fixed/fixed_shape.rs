//! Schema types for Fixed shapes.
//!
//! Note: Metadata types are in `fixed_meta.rs`.

use super::fixed_meta::FixedMeta;
use crate::kind::shape::LpShape;

/// Trait for Fixed shapes that can have metadata.
pub trait FixedShape: LpShape {
    /// Get the metadata for this shape, if any.
    fn meta(&self) -> Option<&dyn FixedMeta>;
}
