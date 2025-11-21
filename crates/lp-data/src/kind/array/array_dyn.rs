//! Dynamic shape implementation for Array.

use alloc::string::String;

use super::array_meta::{ArrayMeta, ArrayMetaDyn};
use super::array_shape::ArrayShape;
use crate::kind::kind::LpKind;
use crate::kind::shape::LpShape;

/// Dynamic array shape.
///
/// Allocated in lp-pool.
pub struct ArrayShapeDyn {
    /// Metadata for this array shape.
    pub meta: ArrayMetaDyn,

    /// Shape of elements in this array.
    pub element_shape: &'static dyn LpShape,

    /// Current length of this array.
    pub len: usize,
}

impl ArrayShapeDyn {
    pub fn new() -> Self {
        Self {
            meta: ArrayMetaDyn {
                name: String::new(),
                docs: None,
            },
            element_shape: &crate::kind::dec32::dec32_static::DEC32_SHAPE,
            len: 0,
        }
    }
}

impl Default for ArrayShapeDyn {
    fn default() -> Self {
        Self::new()
    }
}

impl LpShape for ArrayShapeDyn {
    fn kind(&self) -> LpKind {
        LpKind::Array
    }
}

impl ArrayShape for ArrayShapeDyn {
    fn meta(&self) -> &dyn ArrayMeta {
        &self.meta as &dyn ArrayMeta
    }

    fn element_shape(&self) -> &'static dyn LpShape {
        self.element_shape
    }

    fn len(&self) -> usize {
        self.len
    }
}
