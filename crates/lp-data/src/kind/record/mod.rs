//! Record (struct-like) type support.

pub mod record_dyn;
pub mod record_meta;
pub mod record_shape;
pub mod record_static;
pub mod record_value;
pub mod record_value_dyn;

#[cfg(test)]
mod record_tests;

pub use record_dyn::{RecordFieldDyn, RecordShapeDyn};
pub use record_meta::{
    RecordFieldMeta, RecordFieldMetaDyn, RecordFieldMetaStatic, RecordMeta, RecordMetaDyn,
    RecordMetaStatic,
};
pub use record_shape::{RecordFieldShape, RecordShape};
pub use record_static::{RecordFieldStatic, RecordShapeStatic};
pub use record_value_dyn::RecordValueDyn;
