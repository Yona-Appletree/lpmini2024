pub mod r#type;
pub mod value;

pub use r#type::{EnumType, EnumUi, EnumVariant};
// EnumValue moved to shape::enum::enum_value - not re-exported here to avoid conflicts
