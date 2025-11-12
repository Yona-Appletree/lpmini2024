//! Static shape implementation for Union.

use super::enum_struct_meta::{
    EnumStructMeta, EnumStructMetaStatic, EnumStructVariantMeta, EnumStructVariantMetaStatic,
};
use super::enum_struct_shape::{EnumStructShape, EnumStructVariantShape};
use crate::kind::kind::LpKind;
use crate::kind::shape::LpShape;

/// Static variant in a enum struct shape.
pub struct EnumStructVariantStatic {
    /// Variant name.
    pub name: &'static str,

    /// Shape of the variant's value.
    pub shape: &'static dyn LpShape,

    /// Variant metadata.
    pub meta: EnumStructVariantMetaStatic,
}

impl EnumStructVariantShape for EnumStructVariantStatic {
    fn name(&self) -> &str {
        self.name
    }

    fn shape(&self) -> &'static dyn LpShape {
        self.shape
    }

    fn meta(&self) -> &dyn EnumStructVariantMeta {
        &self.meta
    }
}

/// Static enum struct shape.
///
/// Uses `&'static` references for zero-cost storage.
pub struct EnumStructShapeStatic {
    /// Metadata for this enum struct shape.
    pub meta: EnumStructMetaStatic,

    /// Variants in this enum struct.
    pub variants: &'static [EnumStructVariantStatic],
}

impl LpShape for EnumStructShapeStatic {
    fn kind(&self) -> LpKind {
        LpKind::EnumStruct
    }
}

impl EnumStructShape for EnumStructShapeStatic {
    fn meta(&self) -> &dyn EnumStructMeta {
        &self.meta as &dyn EnumStructMeta
    }

    fn variant_count(&self) -> usize {
        self.variants.len()
    }

    fn get_variant(&self, index: usize) -> Option<&dyn EnumStructVariantShape> {
        self.variants
            .get(index)
            .map(|variant| variant as &dyn EnumStructVariantShape)
    }

    fn find_variant(&self, name: &str) -> Option<&dyn EnumStructVariantShape> {
        self.variants
            .iter()
            .find(|variant| variant.name == name)
            .map(|variant| variant as &dyn EnumStructVariantShape)
    }
}
