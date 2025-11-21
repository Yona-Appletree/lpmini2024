/// Conversion trait for ergonomic dec32-point conversions
use super::dec32::Dec32;

/// Trait for converting types to Dec32
///
/// # Examples
/// ```
/// use lp_math::dec32::{Dec32, ToDec32};
/// let a = 5i32.to_dec32();
/// let b = 1.5f32.to_dec32();
/// ```
pub trait ToDec32 {
    fn to_dec32(self) -> Dec32;
}

impl ToDec32 for i32 {
    #[inline(always)]
    fn to_dec32(self) -> Dec32 {
        Dec32::from_i32(self)
    }
}

impl ToDec32 for f32 {
    #[inline(always)]
    fn to_dec32(self) -> Dec32 {
        Dec32::from_f32(self)
    }
}

impl ToDec32 for f64 {
    #[inline(always)]
    fn to_dec32(self) -> Dec32 {
        Dec32::from_f32(self as f32)
    }
}

impl ToDec32 for u32 {
    #[inline(always)]
    fn to_dec32(self) -> Dec32 {
        Dec32::from_i32(self as i32)
    }
}

impl ToDec32 for usize {
    #[inline(always)]
    fn to_dec32(self) -> Dec32 {
        Dec32::from_i32(self as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_to_dec32() {
        let a = 5i32.to_dec32();
        assert_eq!(a.to_f32(), 5.0);

        let b = (-3i32).to_dec32();
        assert_eq!(b.to_f32(), -3.0);
    }

    #[test]
    fn test_f32_to_dec32() {
        let a = 1.5f32.to_dec32();
        assert!((a.to_f32() - 1.5).abs() < 0.001);

        let b = (-2.75f32).to_dec32();
        assert!((b.to_f32() - (-2.75)).abs() < 0.001);
    }

    #[test]
    fn test_f64_to_dec32() {
        let a = 3.14159f64.to_dec32();
        assert!((a.to_f32() - 3.14159).abs() < 0.001);
    }

    #[test]
    fn test_u32_to_dec32() {
        let a = 100u32.to_dec32();
        assert_eq!(a.to_f32(), 100.0);
    }

    #[test]
    fn test_usize_to_dec32() {
        let a = 42usize.to_dec32();
        assert_eq!(a.to_f32(), 42.0);
    }
}
