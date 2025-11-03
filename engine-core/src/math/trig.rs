/// Trigonometric functions using lookup tables
use super::fixed::{Fixed, SHIFT, ONE, mul};
use crate::sin_table::SIN_TABLE_I32 as SIN_TABLE;

/// Sine function using lookup table
/// Input: Fixed-point value where 1.0 = full circle (0..1)
/// Output: -1..1 in fixed-point
#[inline]
pub fn sin(x: Fixed) -> Fixed {
    // Normalize to 0..1 range
    let angle = x & (ONE - 1); // Get fractional part (0..1)
    
    // Scale to table size
    let table_size = SIN_TABLE.len();
    let index = ((angle as u32 * table_size as u32) >> SHIFT) as usize;
    let idx = index.min(table_size - 1);
    
    SIN_TABLE[idx]
}

/// Cosine function using lookup table
/// Input: Fixed-point value where 1.0 = full circle (0..1)
/// Output: -1..1 in fixed-point
#[inline]
pub fn cos(x: Fixed) -> Fixed {
    // cos(x) = sin(x + 0.25)
    sin(x + (ONE / 4))
}

/// Tangent (simple approximation: sin/cos)
#[inline]
pub fn tan(x: Fixed) -> Fixed {
    let s = sin(x);
    let c = cos(x);
    if c.abs() < 100 { // Avoid division by very small numbers
        return if s >= 0 { ONE * 100 } else { -ONE * 100 };
    }
    super::fixed::div(s, c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::fixed::{from_f32, to_f32};
    
    #[test]
    fn test_sin_values() {
        // sin(0) = 0
        let s0 = sin(0);
        assert!(s0.abs() < 1000, "sin(0) should be ~0, got {}", to_f32(s0));
        
        // sin(0.25) = 1 (quarter circle = 90 degrees)
        let s90 = sin(from_f32(0.25));
        assert!((to_f32(s90) - 1.0).abs() < 0.02, "sin(0.25) should be ~1.0, got {}", to_f32(s90));
        
        // sin(0.5) = 0 (half circle = 180 degrees)
        let s180 = sin(from_f32(0.5));
        assert!(s180.abs() < 1000, "sin(0.5) should be ~0, got {}", to_f32(s180));
        
        // sin(0.75) = -1 (three-quarter circle = 270 degrees)
        let s270 = sin(from_f32(0.75));
        assert!((to_f32(s270) + 1.0).abs() < 0.02, "sin(0.75) should be ~-1.0, got {}", to_f32(s270));
    }
    
    #[test]
    fn test_cos_values() {
        // cos(0) = 1
        let c0 = cos(0);
        assert!((to_f32(c0) - 1.0).abs() < 0.02, "cos(0) should be ~1.0, got {}", to_f32(c0));
        
        // cos(0.25) = 0 (quarter circle = 90 degrees)
        let c90 = cos(from_f32(0.25));
        assert!(c90.abs() < 1000, "cos(0.25) should be ~0, got {}", to_f32(c90));
        
        // cos(0.5) = -1 (half circle = 180 degrees)
        let c180 = cos(from_f32(0.5));
        assert!((to_f32(c180) + 1.0).abs() < 0.02, "cos(0.5) should be ~-1.0, got {}", to_f32(c180));
    }
}

