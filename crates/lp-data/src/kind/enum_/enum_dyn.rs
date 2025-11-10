//! Dynamic shape implementation for Enum.

use super::enum_meta::{EnumMeta, EnumMetaDyn, EnumVariantMeta, EnumVariantMetaDyn};
use super::enum_shape::{EnumShape, EnumVariantShape};
use crate::kind::{kind::LpKind, shape::LpShape};
use lp_pool::LpVec;

/// Dynamic variant in an enum shape.
///
/// Allocated in lp-pool.
pub struct EnumVariantDyn {
    /// Variant name.
    pub name: lp_pool::LpString,

    /// Variant metadata.
    pub meta: EnumVariantMetaDyn,
}

impl EnumVariantShape for EnumVariantDyn {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn meta(&self) -> &dyn EnumVariantMeta {
        &self.meta
    }
}

/// Dynamic enum shape.
///
/// Allocated in lp-pool.
pub struct EnumShapeDyn {
    /// Metadata for this enum shape.
    pub meta: EnumMetaDyn,

    /// Variants in this enum.
    pub variants: LpVec<EnumVariantDyn>,
}

impl EnumShapeDyn {
    pub fn new() -> Self {
        Self {
            meta: EnumMetaDyn {
                name: lp_pool::LpString::new(),
                docs: None,
            },
            variants: LpVec::new(),
        }
    }
}

impl LpShape for EnumShapeDyn {
    fn kind(&self) -> LpKind {
        LpKind::Enum
    }
}

impl EnumShape for EnumShapeDyn {
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
            .find(|v| v.name.as_str() == name)
            .map(|v| v as &dyn EnumVariantShape)
    }
}
