//! Static vec3 shape implementation.

use crate::shape::kind::LpKind;
use crate::shape::shape::LpShape;
use crate::shape::vec3::vec3_meta::Vec3Ui;

/// Static vec3 shape (compile-time known).
pub struct StaticVec3Shape {
    pub ui: Vec3Ui,
}

impl StaticVec3Shape {
    pub const fn new(ui: Vec3Ui) -> Self {
        Self { ui }
    }

    pub const fn default() -> Self {
        Self { ui: Vec3Ui::Raw }
    }
}

impl LpShape for StaticVec3Shape {
    fn kind(&self) -> LpKind {
        LpKind::Vec3
    }
}

impl core::fmt::Debug for StaticVec3Shape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticVec3Shape")
            .field("ui", &self.ui)
            .finish()
    }
}
