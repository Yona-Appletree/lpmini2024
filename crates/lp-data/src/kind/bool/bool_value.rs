//! Value implementation for Bool.

use crate::kind::bool::bool_static::BOOL_SHAPE;

crate::define_primitive_value! {
    rust_type: bool,
    kind: Bool,
    shape_const: BOOL_SHAPE,
    value_box_variant: Bool,
}
