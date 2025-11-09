pub mod r#type;
pub mod value;

pub use r#type::OptionType;
// OptionValue moved to shape::option::option_value - not re-exported here to avoid conflicts
