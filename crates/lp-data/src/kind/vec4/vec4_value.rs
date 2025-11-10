//! Value implementation for Vec4.

use crate::kind::vec4::vec4_static::VEC4_SHAPE;
use lp_math::fixed::Vec4;

crate::define_primitive_value! {
    rust_type: Vec4,
    kind: Vec4,
    shape_const: VEC4_SHAPE,
    value_box_variant: Vec4,
}
