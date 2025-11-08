pub mod advanced;
pub mod clamping;
pub mod conversions;
/// Fixed-point fixed library
///
/// Provides clean APIs for fixed-point arithmetic and fixed functions.
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
/// use lp_math::fixed::ToFixed;
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
/// - `trig::sin(x)` - Sine (input in radians, 2π = full circle)
/// - `trig::cos(x)` - Cosine (input in radians)
/// - `trig::tan(x)` - Tangent (input in radians)
///
/// # Noise
/// - `noise::perlin3(x, y, z, octaves)` - 3D Perlin noise
#[allow(clippy::module_inception)]
pub mod fixed;
pub mod interpolation;
pub mod noise;
pub mod rounding;
#[cfg(feature = "serde")]
pub mod serde_impl;
pub mod sin_table;
pub mod trig;
pub mod vec2;
pub mod vec3;
pub mod vec4;

// Re-export commonly used items at module level
pub use clamping::{saturate, sign};
pub use conversions::ToFixed;
pub use fixed::Fixed;
pub use interpolation::{lerp, smoothstep, step};
pub use rounding::{ceil, floor, frac};
pub use trig::{cos, sin, tan};
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

// Re-export fixed utilities
pub use crate::fixed::advanced::{atan, atan2, fract, modulo, pow, sqrt};

// Legacy compatibility - maintain old function names
// Re-export for backwards compatibility
pub const FIXED_ONE: i32 = Fixed::ONE.0;
pub const FIXED_SHIFT: i32 = Fixed::SHIFT;

#[deprecated(note = "Use Fixed operators instead")]
#[inline(always)]
pub fn fixed_mul(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) >> Fixed::SHIFT) as i32
}

#[deprecated(note = "Use Fixed operators instead")]
#[inline(always)]
pub fn fixed_div(a: i32, b: i32) -> i32 {
    if b != 0 {
        ((a as i64 * Fixed::ONE.0 as i64) / b as i64) as i32
    } else {
        0
    }
}

#[deprecated(note = "Use Fixed::from_i32 instead")]
#[inline(always)]
pub fn fixed_from_int(i: i32) -> i32 {
    i << Fixed::SHIFT
}

#[deprecated(note = "Use Fixed::from_f32 instead")]
#[inline(always)]
pub fn fixed_from_f32(f: f32) -> i32 {
    (f * Fixed::ONE.0 as f32) as i32
}

#[deprecated(note = "Use Fixed::to_f32 instead")]
#[inline(always)]
pub fn fixed_to_f32(f: i32) -> f32 {
    f as f32 / Fixed::ONE.0 as f32
}

// Legacy trig
pub use trig::{cos as cos_fixed, sin as sin_fixed};
