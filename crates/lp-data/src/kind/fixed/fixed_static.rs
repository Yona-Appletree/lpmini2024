//! Static shape implementation for Fixed.

use super::fixed_meta::{FixedMeta, FixedMetaStatic};
use super::fixed_shape::FixedShape;
use crate::kind::{kind::LpKind, shape::LpShape};

/// Static shape for Fixed values.
///
/// This is a singleton that can optionally have metadata attached.
pub struct FixedShapeStatic {
    /// Optional metadata for this shape.
    pub meta: Option<FixedMetaStatic>,
}

impl FixedShapeStatic {
    /// Create a new Fixed shape without metadata.
    pub const fn new() -> Self {
        Self { meta: None }
    }

    /// Create a new Fixed shape with metadata.
    pub const fn with_meta(meta: FixedMetaStatic) -> Self {
        Self { meta: Some(meta) }
    }
}

impl LpShape for FixedShapeStatic {
    fn kind(&self) -> LpKind {
        LpKind::Fixed
    }
}

impl FixedShape for FixedShapeStatic {
    fn meta(&self) -> Option<&dyn FixedMeta> {
        self.meta.as_ref().map(|m| m as &dyn FixedMeta)
    }
}

/// Default singleton instance of FixedShapeStatic without metadata.
pub const FIXED_SHAPE: FixedShapeStatic = FixedShapeStatic::new();
