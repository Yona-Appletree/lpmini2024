//! Shape traits for Option types.

use super::option_meta::OptionMeta;
use crate::kind::shape::LpShape;

/// Trait for Option shapes that wrap an inner type.
pub trait OptionShape: LpShape {
    /// Get the metadata for this Option shape.
    fn meta(&self) -> &dyn OptionMeta;

    /// Get the shape of the inner value type (when Some).
    fn inner_shape(&self) -> &'static dyn LpShape;
}
