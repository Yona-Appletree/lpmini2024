//! Static shape implementation for Enum.

use super::enum_meta::{EnumMeta, EnumMetaStatic, EnumVariantMeta, EnumVariantMetaStatic};
use super::enum_shape::{EnumShape, EnumVariantShape};
use crate::kind::kind::LpKind;
use crate::kind::shape::LpShape;

/// Static variant in an enum shape.
pub struct EnumVariantStatic {
    /// Variant name.
    pub name: &'static str,

    /// Variant metadata.
    pub meta: EnumVariantMetaStatic,
}

impl EnumVariantShape for EnumVariantStatic {
    fn name(&self) -> &str {
        self.name
    }

    fn meta(&self) -> &dyn EnumVariantMeta {
        &self.meta
    }
}

/// Static enum shape.
///
/// Uses `&'static` references for zero-cost storage.
pub struct EnumShapeStatic {
    /// Metadata for this enum shape.
    pub meta: EnumMetaStatic,

    /// Variants in this enum.
    pub variants: &'static [EnumVariantStatic],
}

impl LpShape for EnumShapeStatic {
    fn kind(&self) -> LpKind {
        LpKind::Enum
    }
}

impl EnumShape for EnumShapeStatic {
    fn meta(&self) -> &dyn EnumMeta {
        &self.meta as &dyn EnumMeta
    }

    fn variant_count(&self) -> usize {
        self.variants.len()
    }

    fn get_variant(&self, index: usize) -> Option<&dyn EnumVariantShape> {
        self.variants.get(index).map(|v| v as &dyn EnumVariantShape)
    }

    fn find_variant(&self, name: &str) -> Option<&dyn EnumVariantShape> {
        self.variants
            .iter()
            .find(|v| v.name == name)
            .map(|v| v as &dyn EnumVariantShape)
    }
}
