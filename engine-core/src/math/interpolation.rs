/// Interpolation functions for fixed-point numbers
use super::fixed::Fixed;

/// Linear interpolation
/// lerp(a, b, t) = a + (b - a) * t
#[inline(always)]
pub fn lerp(a: Fixed, b: Fixed, t: Fixed) -> Fixed {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp_basic() {
        let a = Fixed::from_i32(0);
        let b = Fixed::from_i32(1);
        let t = Fixed::from_f32(0.5);
        let result = lerp(a, b, t);
        assert!((result.to_f32() - 0.5).abs() < 0.01, "Expected 0.5, got {}", result.to_f32());
    }

    #[test]
    fn test_lerp_edge_cases() {
        let a = Fixed::from_i32(10);
        let b = Fixed::from_i32(20);
        
        // t = 0 should return a
        let t0 = Fixed::ZERO;
        let result0 = lerp(a, b, t0);
        assert_eq!(result0.to_f32(), 10.0);
        
        // t = 1 should return b
        let t1 = Fixed::ONE;
        let result1 = lerp(a, b, t1);
        assert_eq!(result1.to_f32(), 20.0);
    }

    #[test]
    fn test_lerp_negative() {
        let a = Fixed::from_i32(-5);
        let b = Fixed::from_i32(5);
        let t = Fixed::from_f32(0.5);
        let result = lerp(a, b, t);
        assert!((result.to_f32() - 0.0).abs() < 0.01, "Expected 0, got {}", result.to_f32());
    }

    #[test]
    fn test_lerp_extrapolation() {
        // Test with t > 1
        let a = Fixed::from_i32(0);
        let b = Fixed::from_i32(10);
        let t = Fixed::from_f32(1.5);
        let result = lerp(a, b, t);
        assert!((result.to_f32() - 15.0).abs() < 0.1, "Expected 15, got {}", result.to_f32());
    }
}

