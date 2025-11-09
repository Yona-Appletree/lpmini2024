//! Static int32 shape implementation.

use crate::shape::int32::int32_meta::Int32Ui;
use crate::shape::kind::LpKind;
use crate::shape::shape::LpShape;

/// Static int32 shape (compile-time known).
pub struct StaticInt32Shape {
    pub ui: Int32Ui,
}

impl StaticInt32Shape {
    pub const fn new(ui: Int32Ui) -> Self {
        Self { ui }
    }

    pub const fn default() -> Self {
        Self {
            ui: Int32Ui::Textbox,
        }
    }
}

impl LpShape for StaticInt32Shape {
    fn kind(&self) -> LpKind {
        LpKind::Int32
    }
}

impl core::fmt::Debug for StaticInt32Shape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticInt32Shape")
            .field("ui", &self.ui)
            .finish()
    }
}
