//! Value implementation for Vec3.

use crate::kind::vec3::vec3_static::VEC3_SHAPE;
use lp_math::fixed::Vec3;

crate::define_primitive_value! {
    rust_type: Vec3,
    kind: Vec3,
    shape_const: VEC3_SHAPE,
    value_box_variant: Vec3,
}
