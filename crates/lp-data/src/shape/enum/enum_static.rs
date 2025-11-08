//! Static enum shape implementation.

use crate::shape::kind::LpKind;
use crate::shape::r#enum::enum_meta::EnumVariant;
use crate::shape::shape::{EnumShape, LpShape};

/// Static enum shape (compile-time known variants).
pub struct StaticEnumShape {
    pub name: &'static str,
    pub variants: &'static [EnumVariant],
    pub ui: crate::shape::r#enum::enum_meta::EnumUi,
}

impl LpShape for StaticEnumShape {
    fn kind(&self) -> LpKind {
        LpKind::Enum
    }
}

impl EnumShape for StaticEnumShape {
    fn variants(&self) -> &[EnumVariant] {
        self.variants
    }
}

impl core::fmt::Debug for StaticEnumShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticEnumShape")
            .field("name", &self.name)
            .field("variants", &self.variants)
            .field("ui", &self.ui)
            .finish()
    }
}
