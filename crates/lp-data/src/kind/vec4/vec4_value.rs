//! Value implementation for Vec4.

use lp_math::dec32::Vec4;

use crate::kind::vec4::vec4_static::VEC4_SHAPE;

crate::define_primitive_value! {
    rust_type: Vec4,
    kind: Vec4,
    shape_const: VEC4_SHAPE,
    value_box_variant: Vec4,
}
