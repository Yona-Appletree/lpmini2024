//! Value implementation for Vec2.

use lp_math::dec32::Vec2;

use crate::kind::vec2::vec2_static::VEC2_SHAPE;

crate::define_primitive_value! {
    rust_type: Vec2,
    kind: Vec2,
    shape_const: VEC2_SHAPE,
    value_box_variant: Vec2,
}
