//! Schema types for Array shapes.
//!
//! Note: Metadata types are in `array_meta.rs`.

use super::array_meta::ArrayMeta;
use crate::kind::shape::LpShape;

/// Trait for array shapes that have elements.
pub trait ArrayShape: LpShape {
    /// Get the metadata for this array shape.
    fn meta(&self) -> &dyn ArrayMeta;

    /// Get the shape of elements in this array.
    fn element_shape(&self) -> &'static dyn LpShape;

    /// Get the length of this array.
    ///
    /// For static arrays, this is known at compile time.
    /// For dynamic arrays, this is the current length.
    fn len(&self) -> usize;

    /// Check if this array is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
