//! Value implementation for Fixed.

use lp_math::fixed::Fixed;

use crate::kind::{fixed::fixed_static::FIXED_SHAPE, shape::LpShape, value::LpValue};

impl LpValue for Fixed {
    fn shape(&self) -> &dyn LpShape {
        &FIXED_SHAPE
    }
}
