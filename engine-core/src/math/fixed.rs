/// Fixed-point arithmetic (16.16 format)
///
/// Core type and conversion utilities for fixed-point math.
use core::ops::{Add, Div, Mul, Neg, Sub};

/// Fixed-point constants
const SHIFT: i32 = 16;
const ONE: i32 = 1 << SHIFT;
const HALF: i32 = ONE / 2;

/// Fixed-point number (16.16 format)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixed(pub i32);

impl Fixed {
    pub const SHIFT: i32 = SHIFT;
    pub const ZERO: Fixed = Fixed(0);
    pub const ONE: Fixed = Fixed(ONE);
    pub const HALF: Fixed = Fixed(HALF);

    // Mathematical constants
    pub const PI: Fixed = Fixed(205887); // π ≈ 3.14159265359 in 16.16
    pub const TAU: Fixed = Fixed(411774); // 2π ≈ 6.28318530718 in 16.16
    pub const E: Fixed = Fixed(178145); // e ≈ 2.71828182846 in 16.16
    pub const PHI: Fixed = Fixed(106039); // φ ≈ 1.61803398875 in 16.16 (golden ratio)

    /// Create a Fixed from a raw fixed-point value
    #[inline(always)]
    pub const fn from_fixed(f: i32) -> Self {
        Fixed(f)
    }

    /// Create a Fixed from an f32
    #[inline(always)]
    pub fn from_f32(f: f32) -> Self {
        Fixed((f * ONE as f32) as i32)
    }

    /// Create a Fixed from an i32
    #[inline(always)]
    pub const fn from_i32(i: i32) -> Self {
        Fixed(i << Self::SHIFT)
    }

    /// Convert to f32
    #[inline(always)]
    pub fn to_f32(self) -> f32 {
        self.0 as f32 / ONE as f32
    }

    /// Get the raw fixed-point value
    #[inline(always)]
    pub const fn to_fixed(self) -> i32 {
        self.0
    }

    /// Clamp value between min and max
    #[inline(always)]
    pub fn clamp(self, min: Fixed, max: Fixed) -> Fixed {
        Fixed(self.0.clamp(min.0, max.0))
    }

    /// Return the maximum of two values
    #[inline(always)]
    pub fn max(self, other: Fixed) -> Fixed {
        Fixed(self.0.max(other.0))
    }

    /// Return the minimum of two values
    #[inline(always)]
    pub fn min(self, other: Fixed) -> Fixed {
        Fixed(self.0.min(other.0))
    }

    /// Return the absolute value
    #[inline(always)]
    pub fn abs(self) -> Fixed {
        Fixed(self.0.abs())
    }

    /// Get the fractional part (0..1)
    #[inline(always)]
    pub const fn frac(self) -> Fixed {
        Fixed(self.0 & (ONE - 1))
    }

    /// Get the integer part (floor)
    #[inline(always)]
    pub const fn to_i32(self) -> i32 {
        self.0 >> Self::SHIFT
    }

    /// Multiply by an integer (more efficient than converting to Fixed first)
    #[inline(always)]
    pub const fn mul_int(self, i: i32) -> Fixed {
        Fixed(self.0 * i)
    }
}

impl Add for Fixed {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Fixed(self.0 + rhs.0)
    }
}

impl Sub for Fixed {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Fixed(self.0 - rhs.0)
    }
}

impl Mul for Fixed {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Fixed(((self.0 as i64 * rhs.0 as i64) >> Self::SHIFT) as i32)
    }
}

impl Div for Fixed {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        if rhs.0 != 0 {
            Fixed(((self.0 as i64 * ONE as i64) / rhs.0 as i64) as i32)
        } else {
            Fixed(0)
        }
    }
}

impl Neg for Fixed {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Fixed(-self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(Fixed::ZERO.to_f32(), 0.0);
        assert_eq!(Fixed::ONE.to_f32(), 1.0);
        assert_eq!(Fixed::HALF.to_f32(), 0.5);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(Fixed::from_i32(5).to_f32(), 5.0);
        assert_eq!(Fixed::from_i32(-3).to_f32(), -3.0);
        assert_eq!(Fixed::from_i32(0).to_f32(), 0.0);
    }

    #[test]
    fn test_from_f32() {
        let f = Fixed::from_f32(1.5);
        assert!((f.to_f32() - 1.5).abs() < 0.001);

        let f2 = Fixed::from_f32(-2.75);
        assert!((f2.to_f32() - (-2.75)).abs() < 0.001);
    }

    #[test]
    fn test_add() {
        let a = Fixed::from_i32(2);
        let b = Fixed::from_i32(3);
        assert_eq!((a + b).to_f32(), 5.0);
    }

    #[test]
    fn test_sub() {
        let a = Fixed::from_i32(5);
        let b = Fixed::from_i32(3);
        assert_eq!((a - b).to_f32(), 2.0);
    }

    #[test]
    fn test_mul() {
        let a = Fixed::from_i32(2);
        let b = Fixed::from_i32(3);
        assert_eq!((a * b).to_f32(), 6.0);

        let c = Fixed::from_f32(1.5);
        let d = Fixed::from_f32(2.0);
        assert!((c * d).to_f32() - 3.0 < 0.01);
    }

    #[test]
    fn test_div() {
        let a = Fixed::from_i32(6);
        let b = Fixed::from_i32(2);
        assert_eq!((a / b).to_f32(), 3.0);

        let c = Fixed::from_i32(3);
        let d = Fixed::from_i32(2);
        assert!((c / d).to_f32() - 1.5 < 0.01);
    }

    #[test]
    fn test_neg() {
        let a = Fixed::from_i32(5);
        assert_eq!((-a).to_f32(), -5.0);

        let b = Fixed::from_i32(-3);
        assert_eq!((-b).to_f32(), 3.0);
    }

    #[test]
    fn test_clamp() {
        let val = Fixed::from_i32(5);
        let min = Fixed::from_i32(0);
        let max = Fixed::from_i32(10);
        assert_eq!(val.clamp(min, max).to_f32(), 5.0);

        let val2 = Fixed::from_i32(-5);
        assert_eq!(val2.clamp(min, max).to_f32(), 0.0);

        let val3 = Fixed::from_i32(15);
        assert_eq!(val3.clamp(min, max).to_f32(), 10.0);
    }

    #[test]
    fn test_min_max() {
        let a = Fixed::from_i32(5);
        let b = Fixed::from_i32(10);
        assert_eq!(a.min(b).to_f32(), 5.0);
        assert_eq!(a.max(b).to_f32(), 10.0);
    }
}
