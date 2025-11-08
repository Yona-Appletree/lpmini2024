//! Static option shape implementation.

use crate::shape::kind::LpKind;
use crate::shape::shape::{LpShape, OptionShape};
use crate::shape::shape_ref::ShapeRef;

/// Static option shape (compile-time known inner type).
pub struct StaticOptionShape {
    pub inner: ShapeRef,
}

impl LpShape for StaticOptionShape {
    fn kind(&self) -> LpKind {
        LpKind::Option
    }
}

impl OptionShape for StaticOptionShape {
    fn inner(&self) -> &ShapeRef {
        &self.inner
    }
}

impl core::fmt::Debug for StaticOptionShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticOptionShape")
            .field("inner", &self.inner)
            .finish()
    }
}
