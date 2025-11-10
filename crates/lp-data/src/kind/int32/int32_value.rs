//! Value implementation for Int32.

use crate::kind::int32::int32_static::INT32_SHAPE;

crate::define_primitive_value! {
    rust_type: i32,
    kind: Int32,
    shape_const: INT32_SHAPE,
    value_box_variant: Int32,
}
