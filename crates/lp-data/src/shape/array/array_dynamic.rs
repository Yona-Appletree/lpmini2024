//! Dynamic array shape implementation.

use lp_pool::collections::LpBox;
use lp_pool::error::AllocError;

use crate::shape::kind::LpKind;
use crate::shape::shape::{ArrayShape, LpShape};
use crate::shape::shape_ref::ShapeRef;

/// Dynamic array shape (runtime-created element type).
pub struct DynamicArrayShape {
    pub element: LpBox<ShapeRef>,
    pub ui: crate::shape::array::ArrayUi,
}

impl DynamicArrayShape {
    /// Create a new dynamic array shape.
    pub fn try_new(element: ShapeRef) -> Result<Self, AllocError> {
        let element_box = LpBox::try_new(element)?;
        Ok(Self {
            element: element_box,
            ui: crate::shape::array::ArrayUi::List,
        })
    }
}

impl LpShape for DynamicArrayShape {
    fn kind(&self) -> LpKind {
        LpKind::Array
    }
}

impl ArrayShape for DynamicArrayShape {
    fn element(&self) -> &ShapeRef {
        &*self.element
    }
}

impl core::fmt::Debug for DynamicArrayShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DynamicArrayShape")
            .field("element", &*self.element)
            .field("ui", &self.ui)
            .finish()
    }
}
