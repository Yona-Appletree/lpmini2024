//! Value implementation for Fixed.

use crate::kind::value::LpValueBox;
use crate::kind::{fixed::fixed_static::FIXED_SHAPE, shape::LpShape, value::LpValue};
use lp_math::fixed::Fixed;
use lp_pool::lp_box_dyn;

impl LpValue for Fixed {
    fn shape(&self) -> &dyn LpShape {
        &FIXED_SHAPE
    }
}

impl From<Fixed> for LpValueBox {
    fn from(value: Fixed) -> Self {
        // Box the Fixed value as a trait object
        // Fixed is Copy, so we can move it into pool memory
        let boxed =
            lp_box_dyn!(value, dyn LpValue).expect("Failed to allocate Fixed value in pool");
        LpValueBox::Fixed(boxed)
    }
}
