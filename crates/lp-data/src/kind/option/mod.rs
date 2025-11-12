//! Option type support.

pub mod option_dyn;
pub mod option_meta;
pub mod option_shape;
pub mod option_static;
pub mod option_value;
pub mod option_value_dyn;

pub use option_dyn::OptionShapeDyn;
pub use option_meta::{OptionMeta, OptionMetaDyn, OptionMetaStatic};
pub use option_shape::OptionShape;
pub use option_static::OptionShapeStatic;
pub use option_value::OptionValue;
pub use option_value_dyn::OptionValueDyn;
