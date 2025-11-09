//! Dynamic shape implementation for Fixed.

use super::fixed_meta::{FixedMeta, FixedMetaDyn};
use super::fixed_shape::FixedShape;
use crate::kind::{kind::LpKind, shape::LpShape};

/// Dynamic shape for Fixed values.
///
/// This shape is allocated in lp-pool and can have runtime metadata.
pub struct FixedShapeDyn {
    /// Metadata for this shape.
    pub meta: Option<FixedMetaDyn>,
}

impl FixedShapeDyn {
    /// Create a new Fixed shape without metadata.
    pub fn new() -> Self {
        Self { meta: None }
    }

    /// Create a new Fixed shape with metadata.
    pub fn with_meta(meta: FixedMetaDyn) -> Self {
        Self { meta: Some(meta) }
    }
}

impl LpShape for FixedShapeDyn {
    fn kind(&self) -> LpKind {
        LpKind::Fixed
    }
}

impl FixedShape for FixedShapeDyn {
    fn meta(&self) -> Option<&dyn FixedMeta> {
        self.meta.as_ref().map(|m| m as &dyn FixedMeta)
    }
}
