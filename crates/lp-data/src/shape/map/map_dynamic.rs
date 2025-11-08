//! Dynamic map shape implementation.

use crate::shape::kind::LpKind;
use crate::shape::shape::{LpShape, MapShape};

/// Dynamic map shape (empty - maps are fully dynamic).
pub struct DynamicMapShape;

impl DynamicMapShape {
    /// Create a new dynamic map shape.
    pub fn new() -> Self {
        Self
    }
}

impl LpShape for DynamicMapShape {
    fn kind(&self) -> LpKind {
        LpKind::Map
    }
}

impl MapShape for DynamicMapShape {}

impl core::fmt::Debug for DynamicMapShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DynamicMapShape").finish()
    }
}
