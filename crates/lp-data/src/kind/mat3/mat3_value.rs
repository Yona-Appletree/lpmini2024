//! Value implementation for Mat3.

use lp_math::fixed::Mat3;

use crate::kind::mat3::mat3_static::MAT3_SHAPE;

crate::define_primitive_value! {
    rust_type: Mat3,
    kind: Mat3,
    shape_const: MAT3_SHAPE,
    value_box_variant: Mat3,
}
