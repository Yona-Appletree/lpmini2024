//! Static array shape implementation.

use crate::shape::kind::LpKind;
use crate::shape::shape::{ArrayShape, LpShape};
use crate::shape::shape_ref::ShapeRef;

/// Static array shape (compile-time known element type).
pub struct StaticArrayShape {
    pub element: ShapeRef,
    pub ui: crate::shape::array::ArrayUi,
}

impl LpShape for StaticArrayShape {
    fn kind(&self) -> LpKind {
        LpKind::Array
    }
}

impl ArrayShape for StaticArrayShape {
    fn element(&self) -> &ShapeRef {
        &self.element
    }
}

impl core::fmt::Debug for StaticArrayShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticArrayShape")
            .field("element", &self.element)
            .field("ui", &self.ui)
            .finish()
    }
}
