//! Shape system for lp-data types.
//!
//! This module provides the foundation for the new type system architecture,
//! separating type kinds, shapes (metadata), and values (runtime data).

pub mod kind;
pub mod shape;
pub mod shape_ref;
pub mod value;

// Re-export core types
pub use kind::LpKind;
pub use record::RecordShape;
pub use record::RecordValue;
pub use shape::{ArrayShape, EnumShape, LpShape, MapShape, OptionShape, TupleShape};
pub use shape_ref::ShapeRef;
pub use value::{
    ArrayValue as ArrayValueTrait, EnumValue as EnumValueTrait, LpValueTrait,
    MapValue as MapValueTrait, OptionValue as OptionValueTrait, TupleValue as TupleValueTrait,
};

// Re-export type-specific modules
pub mod array;
pub mod bool;
pub mod r#enum;
pub mod fixed;
pub mod int32;
pub mod map;
pub mod option;
pub mod record;
pub mod string;
pub mod tuple;
pub mod vec2;
pub mod vec3;
pub mod vec4;
