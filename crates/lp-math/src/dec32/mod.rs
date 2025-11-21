pub mod advanced;
pub mod clamping;
pub mod conversions;
/// Dec32-point dec32 library
///
/// Provides clean APIs for dec32-point arithmetic and dec32 functions.
///
/// # Core Types
/// - `Dec32` - 16.16 dec32-point integer
///
/// # Constants
/// - `dec32::ONE` - 1.0 in dec32-point
/// - `dec32::ZERO` - 0.0 in dec32-point
/// - `dec32::HALF` - 0.5 in dec32-point
///
/// # Conversions
/// Use the `ToDec32` trait for ergonomic conversions:
/// ```
/// use lp_math::dec32::ToDec32;
/// let a = 5i32.to_dec32();
/// let b = 1.5f32.to_dec32();
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
pub mod dec32;
pub mod interpolation;
pub mod mat3;
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
pub use conversions::ToDec32;
pub use dec32::Dec32;
pub use interpolation::{lerp, smoothstep, step};
pub use mat3::Mat3;
pub use rounding::{ceil, floor, frac};
pub use trig::{cos, sin, tan};
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

// Re-export dec32 utilities
pub use crate::dec32::advanced::{atan, atan2, fract, modulo, pow, sqrt};

#[deprecated(note = "Use Dec32 operators instead")]
#[inline(always)]
pub fn fixed_mul(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) >> Dec32::SHIFT) as i32
}

#[deprecated(note = "Use Dec32 operators instead")]
#[inline(always)]
pub fn fixed_div(a: i32, b: i32) -> i32 {
    if b != 0 {
        ((a as i64 * Dec32::ONE.0 as i64) / b as i64) as i32
    } else {
        0
    }
}

#[deprecated(note = "Use Dec32::from_i32 instead")]
#[inline(always)]
pub fn fixed_from_int(i: i32) -> i32 {
    i << Dec32::SHIFT
}

#[deprecated(note = "Use Dec32::from_f32 instead")]
#[inline(always)]
pub fn fixed_from_f32(f: f32) -> i32 {
    (f * Dec32::ONE.0 as f32) as i32
}

#[deprecated(note = "Use Dec32::to_f32 instead")]
#[inline(always)]
pub fn fixed_to_f32(f: i32) -> f32 {
    f as f32 / Dec32::ONE.0 as f32
}

// Legacy trig
pub use trig::{cos as cos_dec32, sin as sin_dec32};
