/// Interpolation functions for dec32-point numbers
use super::dec32::Dec32;

/// Linear interpolation
/// lerp(a, b, t) = a + (b - a) * t
#[inline(always)]
pub fn lerp(a: Dec32, b: Dec32, t: Dec32) -> Dec32 {
    a + (b - a) * t
}

/// Step function
/// Returns 0.0 if x < edge, else 1.0
#[inline(always)]
pub fn step(edge: Dec32, x: Dec32) -> Dec32 {
    if x.0 < edge.0 {
        Dec32::ZERO
    } else {
        Dec32::ONE
    }
}

/// Smooth Hermite interpolation
/// Returns smooth transition between 0 and 1 when edge0 < x < edge1
#[inline(always)]
pub fn smoothstep(edge0: Dec32, edge1: Dec32, x: Dec32) -> Dec32 {
    // Clamp x to 0..1 range based on edges
    let t = ((x - edge0) / (edge1 - edge0)).clamp(Dec32::ZERO, Dec32::ONE);
    // Hermite interpolation: 3t² - 2t³
    let t2 = t * t;
    let t3 = t2 * t;
    t2 * Dec32::from_i32(3) - t3 * Dec32::from_i32(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp_basic() {
        let a = Dec32::from_i32(0);
        let b = Dec32::from_i32(1);
        let t = Dec32::from_f32(0.5);
        let result = lerp(a, b, t);
        assert!(
            (result.to_f32() - 0.5).abs() < 0.01,
            "Expected 0.5, got {}",
            result.to_f32()
        );
    }

    #[test]
    fn test_lerp_edge_cases() {
        let a = Dec32::from_i32(10);
        let b = Dec32::from_i32(20);

        // t = 0 should return a
        let t0 = Dec32::ZERO;
        let result0 = lerp(a, b, t0);
        assert_eq!(result0.to_f32(), 10.0);

        // t = 1 should return b
        let t1 = Dec32::ONE;
        let result1 = lerp(a, b, t1);
        assert_eq!(result1.to_f32(), 20.0);
    }

    #[test]
    fn test_lerp_negative() {
        let a = Dec32::from_i32(-5);
        let b = Dec32::from_i32(5);
        let t = Dec32::from_f32(0.5);
        let result = lerp(a, b, t);
        assert!(
            (result.to_f32() - 0.0).abs() < 0.01,
            "Expected 0, got {}",
            result.to_f32()
        );
    }

    #[test]
    fn test_lerp_extrapolation() {
        // Test with t > 1
        let a = Dec32::from_i32(0);
        let b = Dec32::from_i32(10);
        let t = Dec32::from_f32(1.5);
        let result = lerp(a, b, t);
        assert!(
            (result.to_f32() - 15.0).abs() < 0.1,
            "Expected 15, got {}",
            result.to_f32()
        );
    }

    #[test]
    fn test_step() {
        let edge = Dec32::from_f32(0.5);

        // Below edge
        let x1 = Dec32::from_f32(0.3);
        assert_eq!(step(edge, x1), Dec32::ZERO);

        // Above edge
        let x2 = Dec32::from_f32(0.7);
        assert_eq!(step(edge, x2), Dec32::ONE);

        // At edge
        let x3 = Dec32::from_f32(0.5);
        assert_eq!(step(edge, x3), Dec32::ONE);
    }

    #[test]
    fn test_smoothstep() {
        let edge0 = Dec32::ZERO;
        let edge1 = Dec32::ONE;

        // Below range
        let x1 = Dec32::from_f32(-0.5);
        assert_eq!(smoothstep(edge0, edge1, x1), Dec32::ZERO);

        // Above range
        let x2 = Dec32::from_f32(1.5);
        assert_eq!(smoothstep(edge0, edge1, x2), Dec32::ONE);

        // At midpoint should be 0.5
        let x3 = Dec32::from_f32(0.5);
        let result = smoothstep(edge0, edge1, x3);
        assert!(
            (result.to_f32() - 0.5).abs() < 0.01,
            "Expected 0.5, got {}",
            result.to_f32()
        );

        // At edges
        assert_eq!(smoothstep(edge0, edge1, edge0), Dec32::ZERO);
        assert_eq!(smoothstep(edge0, edge1, edge1), Dec32::ONE);
    }
}
