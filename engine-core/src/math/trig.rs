/// Trigonometric functions using lookup tables
use super::fixed::Fixed;
use crate::sin_table::SIN_TABLE_I32 as SIN_TABLE;

/// Sine function using lookup table
/// Input: Fixed-point value where 1.0 = full circle (0..1)
/// Output: -1..1 in fixed-point
#[inline]
pub fn sin(x: Fixed) -> Fixed {
    // Normalize to 0..1 range
    let angle = x.frac();

    // Scale to table size
    let table_size = SIN_TABLE.len() as i32;
    let index = angle.mul_int(table_size).to_i32() as usize;
    let idx = index.min(table_size as usize - 1);

    Fixed(SIN_TABLE[idx])
}

/// Cosine function using lookup table
/// Input: Fixed-point value where 1.0 = full circle (0..1)
/// Output: -1..1 in fixed-point
#[inline]
pub fn cos(x: Fixed) -> Fixed {
    // cos(x) = sin(x + 0.25)
    sin(x + Fixed::HALF / Fixed::from_i32(2))
}

/// Tangent (simple approximation: sin/cos)
#[inline]
pub fn tan(x: Fixed) -> Fixed {
    let s = sin(x);
    let c = cos(x);
    if c.0.abs() < 100 {
        // Avoid division by very small numbers
        let large = Fixed::ONE * Fixed::from_i32(100);
        return if s.0 >= 0 { large } else { -large };
    }
    s / c
}

#[cfg(test)]
mod tests {
    use super::super::conversions::ToFixed;
    use super::*;

    #[test]
    fn test_sin_values() {
        // sin(0) = 0
        let s0 = sin(0i32.to_fixed());
        assert!(
            s0.0.abs() < 1000,
            "sin(0) should be ~0, got {}",
            s0.to_f32()
        );

        // sin(0.25) = 1 (quarter circle = 90 degrees)
        let s90 = sin(0.25f32.to_fixed());
        assert!(
            (s90.to_f32() - 1.0).abs() < 0.02,
            "sin(0.25) should be ~1.0, got {}",
            s90.to_f32()
        );

        // sin(0.5) = 0 (half circle = 180 degrees)
        let s180 = sin(0.5f32.to_fixed());
        assert!(
            s180.0.abs() < 1000,
            "sin(0.5) should be ~0, got {}",
            s180.to_f32()
        );

        // sin(0.75) = -1 (three-quarter circle = 270 degrees)
        let s270 = sin(0.75f32.to_fixed());
        assert!(
            (s270.to_f32() + 1.0).abs() < 0.02,
            "sin(0.75) should be ~-1.0, got {}",
            s270.to_f32()
        );
    }

    #[test]
    fn test_cos_values() {
        // cos(0) = 1
        let c0 = cos(0i32.to_fixed());
        assert!(
            (c0.to_f32() - 1.0).abs() < 0.02,
            "cos(0) should be ~1.0, got {}",
            c0.to_f32()
        );

        // cos(0.25) = 0 (quarter circle = 90 degrees)
        let c90 = cos(0.25f32.to_fixed());
        assert!(
            c90.0.abs() < 1000,
            "cos(0.25) should be ~0, got {}",
            c90.to_f32()
        );

        // cos(0.5) = -1 (half circle = 180 degrees)
        let c180 = cos(0.5f32.to_fixed());
        assert!(
            (c180.to_f32() + 1.0).abs() < 0.02,
            "cos(0.5) should be ~-1.0, got {}",
            c180.to_f32()
        );
    }
}
