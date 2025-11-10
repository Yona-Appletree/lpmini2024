//! Value implementation for Fixed.

use lp_math::fixed::Fixed;

use crate::kind::fixed::fixed_static::FIXED_SHAPE;

crate::define_primitive_value! {
    rust_type: Fixed,
    kind: Fixed,
    shape_const: FIXED_SHAPE,
    value_box_variant: Fixed,
}
