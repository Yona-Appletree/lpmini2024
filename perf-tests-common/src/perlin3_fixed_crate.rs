/// Perlin3 noise using the `fixed` crate
/// This uses the standard fixed-point library with optimized operations

use crate::sin_table::{SIN_TABLE_I16F16 as SIN_TABLE, SIN_TABLE_SIZE};
use fixed::types::I16F16;

/// Fast sine using lookup table
/// Input x should be in I16F16 format representing radians
#[inline(always)]
fn sin_fixed(x: I16F16) -> I16F16 {
    // The table covers 0 to 2π mapped to indices 0-255
    const TWO_PI: I16F16 = I16F16::from_bits(411775); // 2π ≈ 6.28318
    
    // Normalize to 0 to 2π range
    let normalized = x.rem_euclid(TWO_PI);
    
    // Map 0..TWO_PI to 0..255
    let index_fixed = (normalized * 256) / TWO_PI;
    let index = (index_fixed.to_num::<i32>() & 0xFF) as usize;
    SIN_TABLE[index]
}

/// Fast cosine using lookup table
#[inline(always)]
fn cos_fixed(x: I16F16) -> I16F16 {
    // cos(x) = sin(x + π/2)
    const PI_DIV_2: I16F16 = I16F16::from_bits(102944); // π/2 ≈ 1.5708
    sin_fixed(x + PI_DIV_2)
}

/// Perlin3 noise using fixed crate
#[inline(always)]
fn perlin3_fixed_crate(x: I16F16, y: I16F16, z: I16F16) -> I16F16 {
    let freq1 = I16F16::from_num(1);
    let freq2 = I16F16::from_num(2);
    let freq3 = I16F16::from_num(4);

    let n1 = sin_fixed(x * freq1 + z) * cos_fixed(y * freq1);
    let n2 = sin_fixed(x * freq2 - z) * cos_fixed(y * freq2) / 2;
    let n3 = sin_fixed(x * freq3 + y + z) / 4;

    let sum = n1 + n2 + n3;
    // Divide by 1.75 = multiply by 1/1.75 ≈ 0.5714
    sum * I16F16::from_num(0.5714)
}

#[inline(never)]
pub fn render_frame(buffer: &mut [u8], time: f32, width: usize, height: usize) {
    let width_fixed = I16F16::from_num(width);
    let height_fixed = I16F16::from_num(height);
    let time_fixed = I16F16::from_num(time * 0.001);

    for y in 0..height {
        for x in 0..width {
            let x_fixed = I16F16::from_num(x);
            let y_fixed = I16F16::from_num(y);
            
            let nx = (x_fixed * 4) / width_fixed;
            let ny = (y_fixed * 4) / height_fixed;
            let nz = time_fixed;

            let noise = perlin3_fixed_crate(nx, ny, nz);
            
            // Convert from -1.0 to 1.0 range to 0-255
            let shifted = noise + I16F16::from_num(1); // 0 to 2.0
            let scaled = (shifted * 255) / 2; // Scale to 0-255
            let clamped = scaled.clamp(I16F16::ZERO, I16F16::from_num(255));
            let value = clamped.to_num::<u8>();

            let idx = (y * width + x) * 3;
            buffer[idx] = value;
            buffer[idx + 1] = value;
            buffer[idx + 2] = value;
        }
    }
}

