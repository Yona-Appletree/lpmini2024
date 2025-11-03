/// Fixed-point math library
/// 
/// Provides clean APIs for fixed-point arithmetic and math functions.
/// 
/// # Core Types
/// - `Fixed` - 16.16 fixed-point integer
/// 
/// # Constants
/// - `fixed::ONE` - 1.0 in fixed-point
/// - `fixed::ZERO` - 0.0 in fixed-point
/// - `fixed::HALF` - 0.5 in fixed-point
/// 
/// # Conversions
/// Use the `ToFixed` trait for ergonomic conversions:
/// ```
/// use engine_core::math::ToFixed;
/// let a = 5i32.to_fixed();
/// let b = 1.5f32.to_fixed();
/// ```
/// 
/// # Math Utilities
/// - `rounding::floor(f)` - Round down
/// - `rounding::ceil(f)` - Round up
/// - `rounding::frac(f)` - Get fractional part
/// - `interpolation::lerp(a, b, t)` - Linear interpolation
/// - `clamping::saturate(a)` - Clamp to 0..1
/// - `clamping::sign(a)` - Get sign (-1, 0, or 1)
/// - `advanced::sqrt(a)` - Square root
/// 
/// # Trigonometry
/// - `trig::sin(x)` - Sine (0..1 = full circle)
/// - `trig::cos(x)` - Cosine
/// - `trig::tan(x)` - Tangent
/// 
/// # Noise
/// - `noise::perlin3(x, y, z, octaves)` - 3D Perlin noise

pub mod fixed;
pub mod conversions;
pub mod rounding;
pub mod interpolation;
pub mod clamping;
pub mod advanced;
pub mod vec2;
pub mod trig;
pub mod noise;

// Re-export commonly used items at module level
pub use fixed::{Fixed, SHIFT as FIXED_SHIFT, ONE as FIXED_ONE};
pub use conversions::ToFixed;
pub use vec2::Vec2;

// Re-export math utilities
pub use rounding::{floor, ceil, frac};
pub use interpolation::lerp;
pub use clamping::{saturate, sign};
pub use advanced::sqrt;

// Legacy compatibility - maintain old function names
#[deprecated(note = "Use Fixed operators instead")]
#[inline(always)]
pub fn fixed_mul(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) >> FIXED_SHIFT) as i32
}

#[deprecated(note = "Use Fixed operators instead")]
#[inline(always)]
pub fn fixed_div(a: i32, b: i32) -> i32 {
    if b != 0 {
        ((a as i64 * FIXED_ONE as i64) / b as i64) as i32
    } else {
        0
    }
}

#[deprecated(note = "Use Fixed::from_i32 instead")]
#[inline(always)]
pub fn fixed_from_int(i: i32) -> i32 {
    i << FIXED_SHIFT
}

#[deprecated(note = "Use Fixed::from_f32 instead")]
#[inline(always)]
pub fn fixed_from_f32(f: f32) -> i32 {
    (f * FIXED_ONE as f32) as i32
}

#[deprecated(note = "Use Fixed::to_f32 instead")]
#[inline(always)]
pub fn fixed_to_f32(f: i32) -> f32 {
    f as f32 / FIXED_ONE as f32
}

// Legacy trig
pub use trig::{sin as sin_fixed, cos as cos_fixed};
