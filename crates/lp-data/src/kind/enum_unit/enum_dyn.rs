//! Dynamic shape implementation for Enum.

use alloc::string::String;
use alloc::vec::Vec;

use super::enum_meta::{
    EnumUnitMeta, EnumUnitMetaDyn, EnumUnitVariantMeta, EnumUnitVariantMetaDyn,
};
use super::enum_shape::{EnumUnitShape, EnumUnitVariantShape};
use crate::kind::kind::LpKind;
use crate::kind::shape::LpShape;

/// Dynamic variant in an enum shape.
///
/// Allocated in lp-pool.
pub struct EnumUnitVariantDyn {
    /// Variant name.
    pub name: String,

    /// Variant metadata.
    pub meta: EnumUnitVariantMetaDyn,
}

impl EnumUnitVariantShape for EnumUnitVariantDyn {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn meta(&self) -> &dyn EnumUnitVariantMeta {
        &self.meta
    }
}

/// Dynamic enum shape.
///
/// Allocated in lp-pool.
pub struct EnumUnitShapeDyn {
    /// Metadata for this enum shape.
    pub meta: EnumUnitMetaDyn,

    /// Variants in this enum.
    pub variants: Vec<EnumUnitVariantDyn>,
}

impl EnumUnitShapeDyn {
    pub fn new() -> Self {
        Self {
            meta: EnumUnitMetaDyn {
                name: String::new(),
                docs: None,
            },
            variants: Vec::new(),
        }
    }
}

impl Default for EnumUnitShapeDyn {
    fn default() -> Self {
        Self::new()
    }
}

impl LpShape for EnumUnitShapeDyn {
    fn kind(&self) -> LpKind {
        LpKind::EnumUnit
    }
}

impl EnumUnitShape for EnumUnitShapeDyn {
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
            .find(|v| v.name.as_str() == name)
            .map(|v| v as &dyn EnumUnitVariantShape)
    }
}
