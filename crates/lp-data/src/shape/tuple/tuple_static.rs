//! Static tuple shape implementation.

use crate::shape::kind::LpKind;
use crate::shape::shape::{LpShape, TupleShape};
use crate::shape::shape_ref::ShapeRef;

/// Static tuple shape (compile-time known element types).
pub struct StaticTupleShape {
    pub elements: &'static [ShapeRef],
}

impl LpShape for StaticTupleShape {
    fn kind(&self) -> LpKind {
        LpKind::Tuple
    }
}

impl TupleShape for StaticTupleShape {
    fn elements(&self) -> &[ShapeRef] {
        self.elements
    }
}

impl core::fmt::Debug for StaticTupleShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticTupleShape")
            .field("elements", &self.elements)
            .finish()
    }
}
