#![cfg_attr(feature = "use-libm", no_std)]

/// Shared sine lookup table to avoid duplication
mod sin_table;

/// Performance test modules for LED matrix rendering
/// Each module implements a render_frame function with different optimizations

pub mod perlin3_float_libm;
pub mod perlin3_float_approx;
pub mod perlin3_fixed;
pub mod perlin3_fixed_crate;

/// Common render function signature
pub type RenderFn = fn(buffer: &mut [u8], time: f32, width: usize, height: usize);
