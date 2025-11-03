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
/// - `fixed::from_int(n)` - Convert integer to fixed-point
/// - `fixed::from_f32(f)` - Convert f32 to fixed-point
/// - `fixed::to_int(f)` - Convert fixed-point to integer
/// - `fixed::to_f32(f)` - Convert fixed-point to f32
/// 
/// # Arithmetic
/// - `fixed::mul(a, b)` - Fixed-point multiplication
/// - `fixed::div(a, b)` - Fixed-point division
/// - `fixed::frac(f)` - Get fractional part
/// - `fixed::floor(f)` - Round down
/// - `fixed::ceil(f)` - Round up
/// 
/// # Trigonometry
/// - `trig::sin(x)` - Sine (0..1 = full circle)
/// - `trig::cos(x)` - Cosine
/// - `trig::tan(x)` - Tangent
/// 
/// # Noise
/// - `noise::perlin3(x, y, z, octaves)` - 3D Perlin noise

pub mod dec;
pub mod vec2;
pub mod fixed;
pub mod trig;
pub mod noise;

// Re-export commonly used items at module level
pub use fixed::{Fixed, SHIFT as FIXED_SHIFT, ONE as FIXED_ONE};
pub use vec2::Vec2;

// Legacy compatibility - re-export all fixed functions
pub use fixed::{
    mul as fixed_mul,
    div as fixed_div,
    from_int as fixed_from_int,
    from_f32 as fixed_from_f32,
    to_f32 as fixed_to_f32,
};

// Legacy trig
pub use trig::{sin as sin_fixed, cos as cos_fixed};
