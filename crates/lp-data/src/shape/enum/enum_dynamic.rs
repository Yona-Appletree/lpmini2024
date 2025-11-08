//! Dynamic enum shape implementation.

use lp_pool::collections::{LpString, LpVec};
use lp_pool::error::AllocError;

use crate::shape::kind::LpKind;
use crate::shape::r#enum::enum_meta::EnumVariant;
use crate::shape::shape::{EnumShape, LpShape};

/// Dynamic enum shape (runtime-created variants).
pub struct DynamicEnumShape {
    pub name: LpString,
    pub variants: LpVec<EnumVariant>,
    pub ui: crate::shape::r#enum::enum_meta::EnumUi,
}

impl DynamicEnumShape {
    /// Create a new dynamic enum shape.
    pub fn try_new(name: &str, variants: LpVec<EnumVariant>) -> Result<Self, AllocError> {
        let name_str = LpString::try_from_str(name)?;
        Ok(Self {
            name: name_str,
            variants,
            ui: crate::shape::r#enum::enum_meta::EnumUi::Dropdown,
        })
    }
}

impl LpShape for DynamicEnumShape {
    fn kind(&self) -> LpKind {
        LpKind::Enum
    }
}

impl EnumShape for DynamicEnumShape {
    fn variants(&self) -> &[EnumVariant] {
        self.variants.as_slice()
    }
}

impl core::fmt::Debug for DynamicEnumShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DynamicEnumShape")
            .field("name", &self.name)
            .field("variants_len", &self.variants.len())
            .field("ui", &self.ui)
            .finish()
    }
}
