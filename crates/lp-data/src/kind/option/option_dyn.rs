//! Dynamic shape implementation for Option.

use super::option_meta::{OptionMeta, OptionMetaDyn};
use super::option_shape::OptionShape;
use crate::kind::kind::LpKind;
use crate::kind::shape::LpShape;

/// Dynamic Option shape.
///
/// Uses `String` for runtime-allocated strings.
pub struct OptionShapeDyn {
    /// Metadata for this Option shape.
    pub meta: OptionMetaDyn,

    /// Shape of the inner value type (when Some).
    pub inner_shape: &'static dyn LpShape,
}

impl LpShape for OptionShapeDyn {
    fn kind(&self) -> LpKind {
        LpKind::Option
    }
}

impl OptionShape for OptionShapeDyn {
    fn meta(&self) -> &dyn OptionMeta {
        &self.meta as &dyn OptionMeta
    }

    fn inner_shape(&self) -> &'static dyn LpShape {
        self.inner_shape
    }
}
