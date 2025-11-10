//! Dynamic shape implementation for Enum.

use alloc::{string::String, vec::Vec};

use super::enum_meta::{EnumMeta, EnumMetaDyn, EnumVariantMeta, EnumVariantMetaDyn};
use super::enum_shape::{EnumShape, EnumVariantShape};
use crate::kind::kind::LpKind;
use crate::kind::shape::LpShape;

/// Dynamic variant in an enum shape.
///
/// Allocated in lp-pool.
pub struct EnumVariantDyn {
    /// Variant name.
    pub name: String,

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
    pub variants: Vec<EnumVariantDyn>,
}

impl EnumShapeDyn {
    pub fn new() -> Self {
        Self {
            meta: EnumMetaDyn {
                name: String::new(),
                docs: None,
            },
            variants: Vec::new(),
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
