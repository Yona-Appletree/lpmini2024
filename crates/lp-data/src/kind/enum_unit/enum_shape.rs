//! Schema types for Enum shapes.
//!
//! Note: Metadata types are in `enum_meta.rs`.

use super::enum_meta::{EnumUnitMeta, EnumUnitVariantMeta};
use crate::kind::shape::LpShape;

/// Trait for enum shapes that have variants.
pub trait EnumUnitShape: LpShape {
    /// Get the metadata for this enum shape.
    fn meta(&self) -> &dyn EnumUnitMeta;

    /// Get the number of variants in this enum.
    fn variant_count(&self) -> usize;

    /// Get a variant by index.
    fn get_variant(&self, index: usize) -> Option<&dyn EnumUnitVariantShape>;

    /// Find a variant by name.
    fn find_variant(&self, name: &str) -> Option<&dyn EnumUnitVariantShape>;
}

/// Trait for enum variant shapes.
pub trait EnumUnitVariantShape {
    /// Get the name of this variant.
    fn name(&self) -> &str;

    /// Get the metadata for this variant.
    fn meta(&self) -> &dyn EnumUnitVariantMeta;
}
