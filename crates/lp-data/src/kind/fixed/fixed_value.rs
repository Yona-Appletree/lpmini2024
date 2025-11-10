//! Value implementation for Fixed.

use crate::kind::fixed::fixed_static::FIXED_SHAPE;
use lp_math::fixed::Fixed;

crate::define_primitive_value! {
    rust_type: Fixed,
    kind: Fixed,
    shape_const: FIXED_SHAPE,
    value_box_variant: Fixed,
}
