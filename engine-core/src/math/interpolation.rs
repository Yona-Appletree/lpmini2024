/// Interpolation functions for fixed-point numbers
use super::fixed::Fixed;

/// Linear interpolation
/// lerp(a, b, t) = a + (b - a) * t
#[inline(always)]
pub fn lerp(a: Fixed, b: Fixed, t: Fixed) -> Fixed {
    a + (b - a) * t
}

/// Step function
/// Returns 0.0 if x < edge, else 1.0
#[inline(always)]
pub fn step(edge: Fixed, x: Fixed) -> Fixed {
    if x.0 < edge.0 {
        Fixed::ZERO
    } else {
        Fixed::ONE
    }
}

/// Smooth Hermite interpolation
/// Returns smooth transition between 0 and 1 when edge0 < x < edge1
#[inline(always)]
pub fn smoothstep(edge0: Fixed, edge1: Fixed, x: Fixed) -> Fixed {
    // Clamp x to 0..1 range based on edges
    let t = ((x - edge0) / (edge1 - edge0)).clamp(Fixed::ZERO, Fixed::ONE);
    // Hermite interpolation: 3t² - 2t³
    let t2 = t * t;
    let t3 = t2 * t;
    t2 * Fixed::from_i32(3) - t3 * Fixed::from_i32(2)
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

    #[test]
    fn test_step() {
        let edge = Fixed::from_f32(0.5);
        
        // Below edge
        let x1 = Fixed::from_f32(0.3);
        assert_eq!(step(edge, x1), Fixed::ZERO);
        
        // Above edge
        let x2 = Fixed::from_f32(0.7);
        assert_eq!(step(edge, x2), Fixed::ONE);
        
        // At edge
        let x3 = Fixed::from_f32(0.5);
        assert_eq!(step(edge, x3), Fixed::ONE);
    }

    #[test]
    fn test_smoothstep() {
        let edge0 = Fixed::ZERO;
        let edge1 = Fixed::ONE;
        
        // Below range
        let x1 = Fixed::from_f32(-0.5);
        assert_eq!(smoothstep(edge0, edge1, x1), Fixed::ZERO);
        
        // Above range
        let x2 = Fixed::from_f32(1.5);
        assert_eq!(smoothstep(edge0, edge1, x2), Fixed::ONE);
        
        // At midpoint should be 0.5
        let x3 = Fixed::from_f32(0.5);
        let result = smoothstep(edge0, edge1, x3);
        assert!((result.to_f32() - 0.5).abs() < 0.01, "Expected 0.5, got {}", result.to_f32());
        
        // At edges
        assert_eq!(smoothstep(edge0, edge1, edge0), Fixed::ZERO);
        assert_eq!(smoothstep(edge0, edge1, edge1), Fixed::ONE);
    }
}

