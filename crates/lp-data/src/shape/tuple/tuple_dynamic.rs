//! Dynamic tuple shape implementation.

use lp_pool::collections::LpVec;
use lp_pool::error::AllocError;

use crate::shape::kind::LpKind;
use crate::shape::shape::{LpShape, TupleShape};
use crate::shape::shape_ref::ShapeRef;

/// Dynamic tuple shape (runtime-created element types).
pub struct DynamicTupleShape {
    pub elements: LpVec<ShapeRef>,
}

impl DynamicTupleShape {
    /// Create a new dynamic tuple shape.
    pub fn try_new(elements: LpVec<ShapeRef>) -> Result<Self, AllocError> {
        Ok(Self { elements })
    }
}

impl LpShape for DynamicTupleShape {
    fn kind(&self) -> LpKind {
        LpKind::Tuple
    }
}

impl TupleShape for DynamicTupleShape {
    fn elements(&self) -> &[ShapeRef] {
        &self.elements
    }
}

impl core::fmt::Debug for DynamicTupleShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DynamicTupleShape")
            .field("elements", &self.elements)
            .finish()
    }
}
