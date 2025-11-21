/// Clamping and sign functions for dec32-point numbers
use super::dec32::Dec32;

/// Sign function: returns -1, 0, or 1
#[inline(always)]
pub fn sign(a: Dec32) -> Dec32 {
    if a.0 > 0 {
        Dec32::ONE
    } else if a.0 < 0 {
        -Dec32::ONE
    } else {
        Dec32::ZERO
    }
}

/// Saturate: clamp to 0..1
#[inline(always)]
pub fn saturate(a: Dec32) -> Dec32 {
    a.clamp(Dec32::ZERO, Dec32::ONE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        let pos = Dec32::from_i32(5);
        assert_eq!(sign(pos).to_f32(), 1.0);

        let neg = Dec32::from_i32(-5);
        assert_eq!(sign(neg).to_f32(), -1.0);

        let zero = Dec32::ZERO;
        assert_eq!(sign(zero).to_f32(), 0.0);
    }

    #[test]
    fn test_sign_edge_cases() {
        // Very small positive
        let small_pos = Dec32::from_f32(0.001);
        assert_eq!(sign(small_pos).to_f32(), 1.0);

        // Very small negative
        let small_neg = Dec32::from_f32(-0.001);
        assert_eq!(sign(small_neg).to_f32(), -1.0);
    }

    #[test]
    fn test_saturate() {
        // Below range
        let below = Dec32::from_f32(-0.5);
        assert_eq!(saturate(below).to_f32(), 0.0);

        // In range
        let in_range = Dec32::from_f32(0.5);
        assert_eq!(saturate(in_range).to_f32(), 0.5);

        // Above range
        let above = Dec32::from_f32(1.5);
        assert_eq!(saturate(above).to_f32(), 1.0);
    }

    #[test]
    fn test_saturate_boundaries() {
        // Exactly at boundaries
        let zero = Dec32::ZERO;
        assert_eq!(saturate(zero).to_f32(), 0.0);

        let one = Dec32::ONE;
        assert_eq!(saturate(one).to_f32(), 1.0);
    }
}
