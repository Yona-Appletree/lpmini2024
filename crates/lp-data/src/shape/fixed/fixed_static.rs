//! Static fixed-point shape implementation.

use crate::shape::fixed::fixed_meta::FixedUi;
use crate::shape::kind::LpKind;
use crate::shape::shape::LpShape;

/// Static fixed-point shape (compile-time known).
pub struct StaticFixedShape {
    pub ui: FixedUi,
}

impl StaticFixedShape {
    pub const fn new(ui: FixedUi) -> Self {
        Self { ui }
    }

    pub const fn default() -> Self {
        Self {
            ui: FixedUi::Textbox,
        }
    }
}

impl LpShape for StaticFixedShape {
    fn kind(&self) -> LpKind {
        LpKind::Fixed
    }
}

impl core::fmt::Debug for StaticFixedShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticFixedShape")
            .field("ui", &self.ui)
            .finish()
    }
}
