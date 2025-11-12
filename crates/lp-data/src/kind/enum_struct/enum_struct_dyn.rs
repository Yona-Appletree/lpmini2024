//! Dynamic shape implementation for Union.

use alloc::string::String;
use alloc::vec::Vec;

use super::enum_struct_meta::{
    EnumStructMeta, EnumStructMetaDyn, EnumStructVariantMeta, EnumStructVariantMetaDyn,
};
use super::enum_struct_shape::{EnumStructShape, EnumStructVariantShape};
use crate::kind::kind::LpKind;
use crate::kind::shape::LpShape;

/// Dynamic variant in a enum struct shape.
///
/// Allocated in lp-pool.
pub struct EnumStructVariantDyn {
    /// Variant name.
    pub name: String,

    /// Shape of the variant's value.
    pub shape: &'static dyn LpShape,

    /// Variant metadata.
    pub meta: EnumStructVariantMetaDyn,
}

impl EnumStructVariantShape for EnumStructVariantDyn {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn shape(&self) -> &'static dyn LpShape {
        self.shape
    }

    fn meta(&self) -> &dyn EnumStructVariantMeta {
        &self.meta
    }
}

/// Dynamic enum struct shape.
///
/// Allocated in lp-pool.
pub struct EnumStructShapeDyn {
    /// Metadata for this enum struct shape.
    pub meta: EnumStructMetaDyn,

    /// Variants in this enum struct.
    pub variants: Vec<EnumStructVariantDyn>,
}

impl EnumStructShapeDyn {
    /// Create an empty dynamic enum struct shape.
    pub fn new() -> Self {
        Self {
            meta: EnumStructMetaDyn {
                name: String::new(),
                docs: None,
            },
            variants: Vec::new(),
        }
    }
}

impl Default for EnumStructShapeDyn {
    fn default() -> Self {
        Self::new()
    }
}

impl LpShape for EnumStructShapeDyn {
    fn kind(&self) -> LpKind {
        LpKind::EnumStruct
    }
}

impl EnumStructShape for EnumStructShapeDyn {
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
            .find(|variant| variant.name.as_str() == name)
            .map(|variant| variant as &dyn EnumStructVariantShape)
    }
}
