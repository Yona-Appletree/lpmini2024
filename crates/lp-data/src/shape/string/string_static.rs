//! Static string shape implementation.

use crate::shape::kind::LpKind;
use crate::shape::shape::LpShape;
use crate::shape::string::string_meta::StringUi;

/// Static string shape (compile-time known).
pub struct StaticStringShape {
    pub ui: StringUi,
}

impl StaticStringShape {
    pub const fn new(ui: StringUi) -> Self {
        Self { ui }
    }

    pub const fn default() -> Self {
        Self {
            ui: StringUi::SingleLine,
        }
    }
}

impl LpShape for StaticStringShape {
    fn kind(&self) -> LpKind {
        LpKind::String
    }
}

impl core::fmt::Debug for StaticStringShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticStringShape")
            .field("ui", &self.ui)
            .finish()
    }
}
