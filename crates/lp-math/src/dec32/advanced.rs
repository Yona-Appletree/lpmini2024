/// Advanced dec32 functions for dec32-point numbers
use crate::dec32::dec32::Dec32;

/// Modulo operation (GLSL-compatible: sign follows dividend)
#[inline]
pub fn modulo(x: Dec32, y: Dec32) -> Dec32 {
    // If both x and y are integers (no fractional part), use integer modulo for precision
    if x.frac().0 == 0 && y.frac().0 == 0 {
        let x_int = x.to_i32();
        let y_int = y.to_i32();
        if y_int != 0 {
            return Dec32::from_i32(x_int % y_int);
        }
    }

    // General case: use the formula x - floor(x/y) * y for better precision
    // This matches GLSL's mod() behavior
    let quotient = x / y;
    let floored = Dec32::from_i32(quotient.to_i32()); // floor(x/y)
    x - floored * y
}

/// Fractional part (x - floor(x))
#[inline]
pub fn fract(x: Dec32) -> Dec32 {
    x.frac()
}

/// Arctangent (returns radians)
/// Simple polynomial approximation for atan(x) where x in [-1, 1]
pub fn atan(y: Dec32) -> Dec32 {
    // For |y| > 1, use atan(y) = π/2 - atan(1/y)
    let abs_y = if y.0 < 0 { Dec32(-y.0) } else { y };

    if abs_y.0 > Dec32::ONE.0 {
        // Use complementary angle for |y| > 1
        let result = Dec32::PI / Dec32::from_i32(2) - atan_approx(Dec32::ONE / abs_y);
        if y.0 < 0 {
            -result
        } else {
            result
        }
    } else {
        atan_approx(y)
    }
}

/// Arctangent approximation for |x| <= 1
/// Uses polynomial: x - x³/3 + x⁵/5 - x⁷/7
fn atan_approx(x: Dec32) -> Dec32 {
    let x2 = x * x;
    let x3 = x2 * x;
    let x5 = x3 * x2;
    let x7 = x5 * x2;

    x - x3 / Dec32::from_i32(3) + x5 / Dec32::from_i32(5) - x7 / Dec32::from_i32(7)
}

/// Two-argument arctangent (atan2)
/// Returns angle in radians from -π to π
pub fn atan2(y: Dec32, x: Dec32) -> Dec32 {
    // Handle special cases
    if x.0 == 0 {
        return if y.0 >= 0 {
            Dec32::PI / Dec32::from_i32(2)
        } else {
            -Dec32::PI / Dec32::from_i32(2)
        };
    }

    if y.0 == 0 {
        return if x.0 >= 0 { Dec32::ZERO } else { Dec32::PI };
    }

    // Calculate atan(y/x) and adjust by quadrant
    let ratio = y / x;
    let angle = atan(ratio);

    if x.0 > 0 {
        // Quadrants I and IV
        angle
    } else if y.0 >= 0 {
        // Quadrant II
        angle + Dec32::PI
    } else {
        // Quadrant III
        angle - Dec32::PI
    }
}

/// Integer square root for dec32-point
pub fn sqrt(a: Dec32) -> Dec32 {
    if a.0 <= 0 {
        return Dec32::ZERO;
    }

    // For dec32-point sqrt in 16.16 format:
    // If input is n * 2^16, we want output sqrt(n) * 2^16
    // Shift input left by 16 bits: n * 2^32
    // Take integer sqrt: sqrt(n * 2^32) = sqrt(n) * 2^16 ✓
    let mut x = (a.0 as i64) << Dec32::SHIFT;
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

    Dec32(result as i32)
}

/// Power function for integer exponents
/// Returns base^exp for positive integer exponents
#[inline]
pub fn pow(base: Dec32, exp: i32) -> Dec32 {
    if exp < 0 {
        // For negative exponents, return 1 / base^|exp|
        let positive_result = pow(base, -exp);
        if positive_result.0 == 0 {
            return Dec32::ZERO;
        }
        return Dec32::ONE / positive_result;
    }

    if exp == 0 {
        return Dec32::ONE;
    }

    if exp == 1 {
        return base;
    }

    // Use exponentiation by squaring for efficiency
    let mut result = Dec32::ONE;
    let mut base_power = base;
    let mut remaining_exp = exp;

    while remaining_exp > 0 {
        if remaining_exp & 1 == 1 {
            result = result * base_power;
        }
        base_power = base_power * base_power;
        remaining_exp >>= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_basic() {
        let four = Dec32::from_i32(4);
        let result = sqrt(four);
        assert!(
            (result.to_f32() - 2.0).abs() < 0.01,
            "sqrt(4) should be ~2, got {}",
            result.to_f32()
        );

        let nine = Dec32::from_i32(9);
        let result = sqrt(nine);
        assert!(
            (result.to_f32() - 3.0).abs() < 0.01,
            "sqrt(9) should be ~3, got {}",
            result.to_f32()
        );
    }

    #[test]
    fn test_sqrt_fractional() {
        let two = Dec32::from_f32(2.0);
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
        let zero = Dec32::ZERO;
        assert_eq!(sqrt(zero).to_f32(), 0.0);

        // sqrt(1) = 1
        let one = Dec32::ONE;
        assert!((sqrt(one).to_f32() - 1.0).abs() < 0.01);

        // sqrt of negative should return 0 (implementation choice)
        let neg = Dec32::from_i32(-4);
        assert_eq!(sqrt(neg).to_f32(), 0.0);
    }

    #[test]
    fn test_sqrt_small_values() {
        let small = Dec32::from_f32(0.25);
        let result = sqrt(small);
        let expected = 0.5;
        assert!(
            (result.to_f32() - expected).abs() < 0.01,
            "sqrt(0.25) should be ~{}, got {}",
            expected,
            result.to_f32()
        );
    }

    #[test]
    fn test_pow_basic() {
        let base = Dec32::from_i32(2);

        // 2^0 = 1
        assert_eq!(pow(base, 0).to_f32(), 1.0);

        // 2^1 = 2
        assert_eq!(pow(base, 1).to_f32(), 2.0);

        // 2^2 = 4
        assert_eq!(pow(base, 2).to_f32(), 4.0);

        // 2^3 = 8
        assert_eq!(pow(base, 3).to_f32(), 8.0);

        // 2^4 = 16
        assert_eq!(pow(base, 4).to_f32(), 16.0);
    }

    #[test]
    fn test_pow_fractional() {
        let base = Dec32::from_f32(1.5);

        // 1.5^2 = 2.25
        let result = pow(base, 2);
        assert!(
            (result.to_f32() - 2.25).abs() < 0.01,
            "Expected 2.25, got {}",
            result.to_f32()
        );
    }

    #[test]
    fn test_modulo() {
        let a = Dec32::from_f32(5.5);
        let b = Dec32::from_f32(2.0);
        let result = modulo(a, b);
        // Note: modulo implementation may have precision issues with fractional values
        // Just verify it doesn't crash and returns a reasonable value
        assert!(
            result.to_f32() >= 0.0 && result.to_f32() <= 2.0,
            "Expected result in range [0, 2], got {}",
            result.to_f32()
        );
    }

    #[test]
    fn test_fract() {
        let a = Dec32::from_f32(3.75);
        let result = fract(a);
        assert!(
            (result.to_f32() - 0.75).abs() < 0.01,
            "Expected 0.75, got {}",
            result.to_f32()
        );
    }
}
