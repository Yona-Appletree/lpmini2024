//! Shape traits for Union types.

use super::enum_struct_meta::{EnumStructMeta, EnumStructVariantMeta};
use crate::kind::shape::LpShape;

/// Trait for enum struct shapes that have variants with associated shapes.
pub trait EnumStructShape: LpShape {
    /// Get the metadata for this enum struct shape.
    fn meta(&self) -> &dyn EnumStructMeta;

    /// Get the number of variants in this enum struct.
    fn variant_count(&self) -> usize;

    /// Get a variant by index.
    fn get_variant(&self, index: usize) -> Option<&dyn EnumStructVariantShape>;

    /// Find a variant by name.
    fn find_variant(&self, name: &str) -> Option<&dyn EnumStructVariantShape>;
}

/// Trait for enum struct variant shapes.
pub trait EnumStructVariantShape {
    /// Get the name of this variant.
    fn name(&self) -> &str;

    /// Get the shape of this variant's value.
    fn shape(&self) -> &'static dyn LpShape;

    /// Get the metadata for this variant.
    fn meta(&self) -> &dyn EnumStructVariantMeta;
}
