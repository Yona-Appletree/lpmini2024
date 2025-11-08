//! Type system modules organized by feature.

pub mod array;
pub mod bool;
pub mod r#enum;
pub mod fixed;
pub mod int32;
pub mod option;
pub mod record;
pub mod record_dyn;
pub mod string;
pub mod vec2;
pub mod vec3;
pub mod vec4;

// Re-export types (metadata)
pub use array::{ArrayType, ArrayUi};
pub use bool::{BoolScalar, BoolUi};
pub use fixed::{FixedScalar, NumberUi, SliderUi};
pub use int32::Int32Scalar;
pub use option::OptionType;
pub use r#enum::{EnumType, EnumUi, EnumVariant};
pub use record::{RecordField, RecordType, RecordUi};
pub use record_dyn::MapType;
pub use string::{StringScalar, StringUi};
pub use vec2::{Vec2Type, Vec2Ui};
pub use vec3::{Vec3Type, Vec3Ui};
pub use vec4::{Vec4Type, Vec4Ui};

// Re-export values
pub use array::ArrayValue;
pub use bool::{as_bool, bool};
pub use fixed::{as_fixed, fixed};
pub use int32::{as_int32, int32};
pub use option::OptionValue;
pub use r#enum::EnumValue;
pub use record::StructValue;
pub use record_dyn::MapValue;
pub use string::{as_string, string};
pub use vec2::{as_vec2, vec2};
pub use vec3::{as_vec3, vec3};
pub use vec4::{as_vec4, vec4};
