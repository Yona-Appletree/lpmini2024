//! Static shape implementation for Array.

use super::array_meta::{ArrayMeta, ArrayMetaStatic};
use super::array_shape::ArrayShape;
use crate::kind::kind::LpKind;
use crate::kind::shape::LpShape;

/// Static array shape.
///
/// Uses `&'static` references for zero-cost storage.
/// The length is stored as a compile-time constant.
pub struct ArrayShapeStatic {
    /// Metadata for this array shape.
    pub meta: ArrayMetaStatic,

    /// Shape of elements in this array.
    pub element_shape: &'static dyn LpShape,

    /// Length of this array (known at compile time).
    pub len: usize,
}

impl LpShape for ArrayShapeStatic {
    fn kind(&self) -> LpKind {
        LpKind::Array
    }
}

impl ArrayShape for ArrayShapeStatic {
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

/// Default static array shape for generic array types.
///
/// This can be used when the specific length is not known at compile time,
/// but the element shape is known.
pub const ARRAY_SHAPE: ArrayShapeStatic = ArrayShapeStatic {
    meta: ArrayMetaStatic {
        name: "Array",
        docs: None,
    },
    element_shape: &crate::kind::dec32::dec32_static::DEC32_SHAPE,
    len: 0,
};
