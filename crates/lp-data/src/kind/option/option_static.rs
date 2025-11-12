//! Static shape implementation for Option.

use super::option_meta::{OptionMeta, OptionMetaStatic};
use super::option_shape::OptionShape;
use crate::kind::kind::LpKind;
use crate::kind::shape::LpShape;

/// Static Option shape.
///
/// Uses `&'static` references for zero-cost storage.
pub struct OptionShapeStatic {
    /// Metadata for this Option shape.
    pub meta: OptionMetaStatic,

    /// Shape of the inner value type (when Some).
    pub inner_shape: &'static dyn LpShape,
}

impl LpShape for OptionShapeStatic {
    fn kind(&self) -> LpKind {
        LpKind::Option
    }
}

impl OptionShape for OptionShapeStatic {
    fn meta(&self) -> &dyn OptionMeta {
        &self.meta as &dyn OptionMeta
    }

    fn inner_shape(&self) -> &'static dyn LpShape {
        self.inner_shape
    }
}
