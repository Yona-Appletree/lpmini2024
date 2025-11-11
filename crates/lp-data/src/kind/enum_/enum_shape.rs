//! Schema types for Enum shapes.
//!
//! Note: Metadata types are in `enum_meta.rs`.

use super::enum_meta::{EnumMeta, EnumVariantMeta};
use crate::kind::shape::LpShape;

/// Trait for enum shapes that have variants.
pub trait EnumShape: LpShape {
    /// Get the metadata for this enum shape.
    fn meta(&self) -> &dyn EnumMeta;

    /// Get the number of variants in this enum.
    fn variant_count(&self) -> usize;

    /// Get a variant by index.
    fn get_variant(&self, index: usize) -> Option<&dyn EnumVariantShape>;

    /// Find a variant by name.
    fn find_variant(&self, name: &str) -> Option<&dyn EnumVariantShape>;
}

/// Trait for enum variant shapes.
pub trait EnumVariantShape {
    /// Get the name of this variant.
    fn name(&self) -> &str;

    /// Get the metadata for this variant.
    fn meta(&self) -> &dyn EnumVariantMeta;
}
