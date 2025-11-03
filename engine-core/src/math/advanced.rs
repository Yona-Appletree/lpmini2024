/// Advanced math functions for fixed-point numbers
use super::fixed::Fixed;

/// Integer square root for fixed-point
pub fn sqrt(a: Fixed) -> Fixed {
    if a.0 <= 0 {
        return Fixed::ZERO;
    }

    // For fixed-point sqrt in 16.16 format:
    // If input is n * 2^16, we want output sqrt(n) * 2^16
    // Shift input left by 16 bits: n * 2^32
    // Take integer sqrt: sqrt(n * 2^32) = sqrt(n) * 2^16 ✓
    let mut x = (a.0 as i64) << Fixed::SHIFT;
    let mut result = 0i64;
    let mut bit = 1i64 << 46; // Start high enough for 48-bit values

    while bit > x {
        bit >>= 2;
    }

    while bit != 0 {
        if x >= result + bit {
            x -= result + bit;
            result = (result >> 1) + bit;
        } else {
            result >>= 1;
        }
        bit >>= 2;
    }

    Fixed(result as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_basic() {
        let four = Fixed::from_i32(4);
        let result = sqrt(four);
        assert!(
            (result.to_f32() - 2.0).abs() < 0.01,
            "sqrt(4) should be ~2, got {}",
            result.to_f32()
        );

        let nine = Fixed::from_i32(9);
        let result = sqrt(nine);
        assert!(
            (result.to_f32() - 3.0).abs() < 0.01,
            "sqrt(9) should be ~3, got {}",
            result.to_f32()
        );
    }

    #[test]
    fn test_sqrt_fractional() {
        let two = Fixed::from_f32(2.0);
        let result = sqrt(two);
        let expected = 1.414; // sqrt(2)
        assert!(
            (result.to_f32() - expected).abs() < 0.01,
            "sqrt(2) should be ~{}, got {}",
            expected,
            result.to_f32()
        );
    }

    #[test]
    fn test_sqrt_edge_cases() {
        // sqrt(0) = 0
        let zero = Fixed::ZERO;
        assert_eq!(sqrt(zero).to_f32(), 0.0);

        // sqrt(1) = 1
        let one = Fixed::ONE;
        assert!((sqrt(one).to_f32() - 1.0).abs() < 0.01);

        // sqrt of negative should return 0 (implementation choice)
        let neg = Fixed::from_i32(-4);
        assert_eq!(sqrt(neg).to_f32(), 0.0);
    }

    #[test]
    fn test_sqrt_small_values() {
        let small = Fixed::from_f32(0.25);
        let result = sqrt(small);
        let expected = 0.5;
        assert!(
            (result.to_f32() - expected).abs() < 0.01,
            "sqrt(0.25) should be ~{}, got {}",
            expected,
            result.to_f32()
        );
    }
}
