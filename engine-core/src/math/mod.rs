/// Math utilities for fixed-point arithmetic

pub mod dec;
pub mod vec2;

pub use dec::{Dec, Fixed, FIXED_SHIFT, FIXED_ONE, fixed_from_f32, fixed_to_f32, fixed_mul, fixed_div, fixed_from_int, fixed_to_int};
pub use vec2::Vec2;

