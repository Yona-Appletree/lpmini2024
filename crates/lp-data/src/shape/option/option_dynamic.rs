//! Dynamic option shape implementation.

use lp_pool::collections::LpBox;
use lp_pool::error::AllocError;

use crate::shape::kind::LpKind;
use crate::shape::shape::{LpShape, OptionShape};
use crate::shape::shape_ref::ShapeRef;

/// Dynamic option shape (runtime-created inner type).
pub struct DynamicOptionShape {
    pub inner: LpBox<ShapeRef>,
}

impl DynamicOptionShape {
    /// Create a new dynamic option shape.
    pub fn try_new(inner: ShapeRef) -> Result<Self, AllocError> {
        let inner_box = LpBox::try_new(inner)?;
        Ok(Self { inner: inner_box })
    }
}

impl LpShape for DynamicOptionShape {
    fn kind(&self) -> LpKind {
        LpKind::Option
    }
}

impl OptionShape for DynamicOptionShape {
    fn inner(&self) -> &ShapeRef {
        &*self.inner
    }
}

impl core::fmt::Debug for DynamicOptionShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DynamicOptionShape")
            .field("inner", &*self.inner)
            .finish()
    }
}
