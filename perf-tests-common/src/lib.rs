#![cfg_attr(all(feature = "use-libm", not(test)), no_std)]

/// Shared sine lookup table to avoid duplication
mod sin_table;

/// Performance test modules for LED matrix rendering
/// Each module implements a render_frame function with different optimizations

pub mod perlin3_float_libm;
pub mod perlin3_float_approx;
pub mod perlin3_fixed;
pub mod perlin3_fixed_crate;
pub mod perlin3_decimal;

/// Common render function signature
pub type RenderFn = fn(buffer: &mut [u8], time: f32, width: usize, height: usize);
