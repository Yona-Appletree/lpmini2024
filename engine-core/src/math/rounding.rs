/// Rounding functions for fixed-point numbers
use super::fixed::{Fixed, ONE};

/// Get fractional part of a fixed-point number
#[inline(always)]
pub fn frac(f: Fixed) -> Fixed {
    Fixed(f.0 & (ONE - 1))
}

/// Floor (round down to integer)
#[inline(always)]
pub fn floor(f: Fixed) -> Fixed {
    Fixed(f.0 & !(ONE - 1))
}

/// Ceiling (round up to integer)
#[inline(always)]
pub fn ceil(f: Fixed) -> Fixed {
    let frac_part = frac(f);
    if frac_part.0 > 0 {
        floor(f) + Fixed::ONE
    } else {
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frac() {
        let f = Fixed::from_f32(1.25);
        let result = frac(f);
        assert!((result.to_f32() - 0.25).abs() < 0.001, "Expected 0.25, got {}", result.to_f32());
        
        // Test with negative number
        let f_neg = Fixed::from_f32(-1.75);
        let result_neg = frac(f_neg);
        // Fractional part of negative numbers in this implementation
        assert!(result_neg.to_f32() >= 0.0);
    }

    #[test]
    fn test_floor() {
        let f = Fixed::from_f32(1.7);
        assert_eq!(floor(f).to_f32(), 1.0);
        
        let f2 = Fixed::from_f32(2.1);
        assert_eq!(floor(f2).to_f32(), 2.0);
        
        let f3 = Fixed::from_f32(-1.7);
        assert_eq!(floor(f3).to_f32(), -2.0);
    }

    #[test]
    fn test_ceil() {
        let f = Fixed::from_f32(1.1);
        assert_eq!(ceil(f).to_f32(), 2.0);
        
        let f2 = Fixed::from_f32(2.9);
        assert_eq!(ceil(f2).to_f32(), 3.0);
        
        let f3 = Fixed::from_f32(3.0);
        assert_eq!(ceil(f3).to_f32(), 3.0);
    }

    #[test]
    fn test_floor_ceil_edge_cases() {
        // Test zero
        assert_eq!(floor(Fixed::ZERO).to_f32(), 0.0);
        assert_eq!(ceil(Fixed::ZERO).to_f32(), 0.0);
        
        // Test exact integers
        let exact = Fixed::from_i32(5);
        assert_eq!(floor(exact).to_f32(), 5.0);
        assert_eq!(ceil(exact).to_f32(), 5.0);
    }
}

