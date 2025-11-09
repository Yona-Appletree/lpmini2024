//! Static vec2 shape implementation.

use crate::shape::kind::LpKind;
use crate::shape::shape::LpShape;
use crate::shape::vec2::vec2_meta::Vec2Ui;

/// Static vec2 shape (compile-time known).
pub struct StaticVec2Shape {
    pub ui: Vec2Ui,
}

impl StaticVec2Shape {
    pub const fn new(ui: Vec2Ui) -> Self {
        Self { ui }
    }

    pub const fn default() -> Self {
        Self { ui: Vec2Ui::Raw }
    }
}

impl LpShape for StaticVec2Shape {
    fn kind(&self) -> LpKind {
        LpKind::Vec2
    }
}

impl core::fmt::Debug for StaticVec2Shape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticVec2Shape")
            .field("ui", &self.ui)
            .finish()
    }
}
