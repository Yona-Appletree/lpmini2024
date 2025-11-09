//! Static boolean shape implementation.

use crate::shape::bool::bool_meta::BoolUi;
use crate::shape::kind::LpKind;
use crate::shape::shape::LpShape;

/// Static boolean shape (compile-time known).
pub struct StaticBoolShape {
    pub ui: BoolUi,
}

impl StaticBoolShape {
    pub const fn new(ui: BoolUi) -> Self {
        Self { ui }
    }

    pub const fn default() -> Self {
        Self {
            ui: BoolUi::Checkbox,
        }
    }
}

impl LpShape for StaticBoolShape {
    fn kind(&self) -> LpKind {
        LpKind::Bool
    }
}

impl core::fmt::Debug for StaticBoolShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticBoolShape")
            .field("ui", &self.ui)
            .finish()
    }
}
