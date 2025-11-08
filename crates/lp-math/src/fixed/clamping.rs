/// Clamping and sign functions for fixed-point numbers
use super::fixed::Fixed;

/// Sign function: returns -1, 0, or 1
#[inline(always)]
pub fn sign(a: Fixed) -> Fixed {
    if a.0 > 0 {
        Fixed::ONE
    } else if a.0 < 0 {
        -Fixed::ONE
    } else {
        Fixed::ZERO
    }
}

/// Saturate: clamp to 0..1
#[inline(always)]
pub fn saturate(a: Fixed) -> Fixed {
    a.clamp(Fixed::ZERO, Fixed::ONE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        let pos = Fixed::from_i32(5);
        assert_eq!(sign(pos).to_f32(), 1.0);

        let neg = Fixed::from_i32(-5);
        assert_eq!(sign(neg).to_f32(), -1.0);

        let zero = Fixed::ZERO;
        assert_eq!(sign(zero).to_f32(), 0.0);
    }

    #[test]
    fn test_sign_edge_cases() {
        // Very small positive
        let small_pos = Fixed::from_f32(0.001);
        assert_eq!(sign(small_pos).to_f32(), 1.0);

        // Very small negative
        let small_neg = Fixed::from_f32(-0.001);
        assert_eq!(sign(small_neg).to_f32(), -1.0);
    }

    #[test]
    fn test_saturate() {
        // Below range
        let below = Fixed::from_f32(-0.5);
        assert_eq!(saturate(below).to_f32(), 0.0);

        // In range
        let in_range = Fixed::from_f32(0.5);
        assert_eq!(saturate(in_range).to_f32(), 0.5);

        // Above range
        let above = Fixed::from_f32(1.5);
        assert_eq!(saturate(above).to_f32(), 1.0);
    }

    #[test]
    fn test_saturate_boundaries() {
        // Exactly at boundaries
        let zero = Fixed::ZERO;
        assert_eq!(saturate(zero).to_f32(), 0.0);

        let one = Fixed::ONE;
        assert_eq!(saturate(one).to_f32(), 1.0);
    }
}
