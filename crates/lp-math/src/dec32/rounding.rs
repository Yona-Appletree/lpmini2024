/// Rounding functions for dec32-point numbers
use super::dec32::Dec32;

/// Get fractional part of a dec32-point number
#[inline(always)]
pub fn frac(f: Dec32) -> Dec32 {
    f.frac()
}

/// Floor (round down to integer)
#[inline(always)]
pub fn floor(f: Dec32) -> Dec32 {
    Dec32(f.0 & !(Dec32::ONE.0 - 1))
}

/// Ceiling (round up to integer)
#[inline(always)]
pub fn ceil(f: Dec32) -> Dec32 {
    let frac_part = frac(f);
    if frac_part.0 > 0 {
        floor(f) + Dec32::ONE
    } else {
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frac() {
        let f = Dec32::from_f32(1.25);
        let result = frac(f);
        assert!(
            (result.to_f32() - 0.25).abs() < 0.001,
            "Expected 0.25, got {}",
            result.to_f32()
        );

        // Test with negative number
        let f_neg = Dec32::from_f32(-1.75);
        let result_neg = frac(f_neg);
        // Fractional part of negative numbers in this implementation
        assert!(result_neg.to_f32() >= 0.0);
    }

    #[test]
    fn test_floor() {
        let f = Dec32::from_f32(1.7);
        assert_eq!(floor(f).to_f32(), 1.0);

        let f2 = Dec32::from_f32(2.1);
        assert_eq!(floor(f2).to_f32(), 2.0);

        let f3 = Dec32::from_f32(-1.7);
        assert_eq!(floor(f3).to_f32(), -2.0);
    }

    #[test]
    fn test_ceil() {
        let f = Dec32::from_f32(1.1);
        assert_eq!(ceil(f).to_f32(), 2.0);

        let f2 = Dec32::from_f32(2.9);
        assert_eq!(ceil(f2).to_f32(), 3.0);

        let f3 = Dec32::from_f32(3.0);
        assert_eq!(ceil(f3).to_f32(), 3.0);
    }

    #[test]
    fn test_floor_ceil_edge_cases() {
        // Test zero
        assert_eq!(floor(Dec32::ZERO).to_f32(), 0.0);
        assert_eq!(ceil(Dec32::ZERO).to_f32(), 0.0);

        // Test exact integers
        let exact = Dec32::from_i32(5);
        assert_eq!(floor(exact).to_f32(), 5.0);
        assert_eq!(ceil(exact).to_f32(), 5.0);
    }
}
