//! Static vec4 shape implementation.

use crate::shape::kind::LpKind;
use crate::shape::shape::LpShape;
use crate::shape::vec4::vec4_meta::Vec4Ui;

/// Static vec4 shape (compile-time known).
pub struct StaticVec4Shape {
    pub ui: Vec4Ui,
}

impl StaticVec4Shape {
    pub const fn new(ui: Vec4Ui) -> Self {
        Self { ui }
    }

    pub const fn default() -> Self {
        Self { ui: Vec4Ui::Raw }
    }
}

impl LpShape for StaticVec4Shape {
    fn kind(&self) -> LpKind {
        LpKind::Vec4
    }
}

impl core::fmt::Debug for StaticVec4Shape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticVec4Shape")
            .field("ui", &self.ui)
            .finish()
    }
}
