/// Conversion trait for ergonomic fixed-point conversions
use super::fixed::Fixed;

/// Trait for converting types to Fixed
///
/// # Examples
/// ```
/// use engine_core::math::{Fixed, ToFixed};
/// let a = 5i32.to_fixed();
/// let b = 1.5f32.to_fixed();
/// ```
pub trait ToFixed {
    fn to_fixed(self) -> Fixed;
}

impl ToFixed for i32 {
    #[inline(always)]
    fn to_fixed(self) -> Fixed {
        Fixed::from_i32(self)
    }
}

impl ToFixed for f32 {
    #[inline(always)]
    fn to_fixed(self) -> Fixed {
        Fixed::from_f32(self)
    }
}

impl ToFixed for f64 {
    #[inline(always)]
    fn to_fixed(self) -> Fixed {
        Fixed::from_f32(self as f32)
    }
}

impl ToFixed for u32 {
    #[inline(always)]
    fn to_fixed(self) -> Fixed {
        Fixed::from_i32(self as i32)
    }
}

impl ToFixed for usize {
    #[inline(always)]
    fn to_fixed(self) -> Fixed {
        Fixed::from_i32(self as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_to_fixed() {
        let a = 5i32.to_fixed();
        assert_eq!(a.to_f32(), 5.0);

        let b = (-3i32).to_fixed();
        assert_eq!(b.to_f32(), -3.0);
    }

    #[test]
    fn test_f32_to_fixed() {
        let a = 1.5f32.to_fixed();
        assert!((a.to_f32() - 1.5).abs() < 0.001);

        let b = (-2.75f32).to_fixed();
        assert!((b.to_f32() - (-2.75)).abs() < 0.001);
    }

    #[test]
    fn test_f64_to_fixed() {
        let a = 3.14159f64.to_fixed();
        assert!((a.to_f32() - 3.14159).abs() < 0.001);
    }

    #[test]
    fn test_u32_to_fixed() {
        let a = 100u32.to_fixed();
        assert_eq!(a.to_f32(), 100.0);
    }

    #[test]
    fn test_usize_to_fixed() {
        let a = 42usize.to_fixed();
        assert_eq!(a.to_f32(), 42.0);
    }
}
