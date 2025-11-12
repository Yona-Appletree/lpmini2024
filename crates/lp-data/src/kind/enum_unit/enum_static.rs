//! Static shape implementation for Enum.

use super::enum_meta::{
    EnumUnitMeta, EnumUnitMetaStatic, EnumUnitVariantMeta, EnumUnitVariantMetaStatic,
};
use super::enum_shape::{EnumUnitShape, EnumUnitVariantShape};
use crate::kind::kind::LpKind;
use crate::kind::shape::LpShape;

/// Static variant in an enum shape.
pub struct EnumUnitVariantStatic {
    /// Variant name.
    pub name: &'static str,

    /// Variant metadata.
    pub meta: EnumUnitVariantMetaStatic,
}

impl EnumUnitVariantShape for EnumUnitVariantStatic {
    fn name(&self) -> &str {
        self.name
    }

    fn meta(&self) -> &dyn EnumUnitVariantMeta {
        &self.meta
    }
}

/// Static enum shape.
///
/// Uses `&'static` references for zero-cost storage.
pub struct EnumUnitShapeStatic {
    /// Metadata for this enum shape.
    pub meta: EnumUnitMetaStatic,

    /// Variants in this enum.
    pub variants: &'static [EnumUnitVariantStatic],
}

impl LpShape for EnumUnitShapeStatic {
    fn kind(&self) -> LpKind {
        LpKind::EnumUnit
    }
}

impl EnumUnitShape for EnumUnitShapeStatic {
    fn meta(&self) -> &dyn EnumUnitMeta {
        &self.meta as &dyn EnumUnitMeta
    }

    fn variant_count(&self) -> usize {
        self.variants.len()
    }

    fn get_variant(&self, index: usize) -> Option<&dyn EnumUnitVariantShape> {
        self.variants
            .get(index)
            .map(|v| v as &dyn EnumUnitVariantShape)
    }

    fn find_variant(&self, name: &str) -> Option<&dyn EnumUnitVariantShape> {
        self.variants
            .iter()
            .find(|v| v.name == name)
            .map(|v| v as &dyn EnumUnitVariantShape)
    }
}
