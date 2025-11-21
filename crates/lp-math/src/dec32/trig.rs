/// Trigonometric functions using lookup tables
use core::cmp::Ord;

use super::dec32::Dec32;
use crate::dec32::sin_table::SIN_TABLE_I32 as SIN_TABLE;

/// Sine function using lookup table
/// Input: Radians (GLSL-compatible: 2π = full circle)
/// Output: -1..1 in dec32-point
#[inline]
pub fn sin(x: Dec32) -> Dec32 {
    // Convert radians to normalized 0..1 range
    // normalized = (x / TAU).frac()
    let normalized = (x / Dec32::TAU).frac();

    // Scale to table size
    let table_size = SIN_TABLE.len() as i32;
    let index = normalized.mul_int(table_size).to_i32() as usize;
    let idx = index.min(table_size as usize - 1);

    Dec32(SIN_TABLE[idx])
}

/// Cosine function using lookup table
/// Input: Radians (GLSL-compatible: 2π = full circle)
/// Output: -1..1 in dec32-point
#[inline]
pub fn cos(x: Dec32) -> Dec32 {
    // cos(x) = sin(x + π/2)
    let pi_over_2 = Dec32::PI / Dec32::from_i32(2);
    sin(x + pi_over_2)
}

/// Tangent (simple approximation: sin/cos)
#[inline]
pub fn tan(x: Dec32) -> Dec32 {
    let s = sin(x);
    let c = cos(x);
    if c.0.abs() < 100 {
        // Avoid division by very small numbers
        let large = Dec32::ONE * Dec32::from_i32(100);
        return if s.0 >= 0 { large } else { -large };
    }
    s / c
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sin_values() {
        // sin(0) = 0
        let s0 = sin(Dec32::ZERO);
        assert!(
            s0.0.abs() < 1000,
            "sin(0) should be ~0, got {}",
            s0.to_f32()
        );

        // sin(π/2) = 1 (quarter circle = 90 degrees)
        let s90 = sin(Dec32::PI / Dec32::from_i32(2));
        assert!(
            (s90.to_f32() - 1.0).abs() < 0.02,
            "sin(π/2) should be ~1.0, got {}",
            s90.to_f32()
        );

        // sin(π) = 0 (half circle = 180 degrees)
        let s180 = sin(Dec32::PI);
        assert!(
            s180.to_f32().abs() < 0.03,
            "sin(π) should be ~0, got {}",
            s180.to_f32()
        );

        // sin(3π/2) = -1 (three-quarter circle = 270 degrees)
        let s270 = sin(Dec32::PI + Dec32::PI / Dec32::from_i32(2));
        assert!(
            (s270.to_f32() + 1.0).abs() < 0.02,
            "sin(3π/2) should be ~-1.0, got {}",
            s270.to_f32()
        );
    }

    #[test]
    fn test_cos_values() {
        // cos(0) = 1
        let c0 = cos(Dec32::ZERO);
        assert!(
            (c0.to_f32() - 1.0).abs() < 0.02,
            "cos(0) should be ~1.0, got {}",
            c0.to_f32()
        );

        // cos(π/2) = 0 (quarter circle = 90 degrees)
        let c90 = cos(Dec32::PI / Dec32::from_i32(2));
        assert!(
            c90.to_f32().abs() < 0.03,
            "cos(π/2) should be ~0, got {}",
            c90.to_f32()
        );

        // cos(π) = -1 (half circle = 180 degrees)
        let c180 = cos(Dec32::PI);
        assert!(
            (c180.to_f32() + 1.0).abs() < 0.02,
            "cos(π) should be ~-1.0, got {}",
            c180.to_f32()
        );
    }
}
