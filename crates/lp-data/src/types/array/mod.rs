pub mod r#type;
pub mod value;

pub use r#type::{ArrayType, ArrayUi};
// ArrayValue moved to shape::array::array_value - not re-exported here to avoid conflicts
