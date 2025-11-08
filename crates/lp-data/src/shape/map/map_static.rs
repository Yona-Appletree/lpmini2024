//! Static map shape implementation.

use crate::shape::kind::LpKind;
use crate::shape::shape::{LpShape, MapShape};

/// Static map shape (empty - maps are fully dynamic).
pub struct StaticMapShape;

impl LpShape for StaticMapShape {
    fn kind(&self) -> LpKind {
        LpKind::Map
    }
}

impl MapShape for StaticMapShape {}

impl core::fmt::Debug for StaticMapShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticMapShape").finish()
    }
}
