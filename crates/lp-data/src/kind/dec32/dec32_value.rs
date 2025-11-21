//! Value implementation for Dec32.

use lp_math::dec32::Dec32;

use crate::kind::dec32::dec32_static::DEC32_SHAPE;

crate::define_primitive_value! {
    rust_type: Dec32,
    kind: Dec32,
    shape_const: DEC32_SHAPE,
    value_box_variant: Dec32,
}
