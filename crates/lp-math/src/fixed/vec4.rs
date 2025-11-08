/// 4D vector for fixed-point fixed (useful for RGBA colors and homogeneous coordinates)
use core::cmp::Ord;
use core::ops::{Add, Div, Mul, Neg, Sub};
use super::conversions::ToFixed;
use super::fixed::Fixed;
use super::vec2::Vec2;
use super::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec4 {
    pub x: Fixed,
    pub y: Fixed,
    pub z: Fixed,
    pub w: Fixed,
}

impl Vec4 {
    #[inline(always)]
    pub const fn new(x: Fixed, y: Fixed, z: Fixed, w: Fixed) -> Self {
        Vec4 { x, y, z, w }
    }

    #[inline(always)]
    pub fn from_f32(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 {
            x: x.to_fixed(),
            y: y.to_fixed(),
            z: z.to_fixed(),
            w: w.to_fixed(),
        }
    }

    #[inline(always)]
    pub fn from_i32(x: i32, y: i32, z: i32, w: i32) -> Self {
        Vec4 {
            x: x.to_fixed(),
            y: y.to_fixed(),
            z: z.to_fixed(),
            w: w.to_fixed(),
        }
    }

    #[inline(always)]
    pub const fn zero() -> Self {
        Vec4::new(Fixed(0), Fixed(0), Fixed(0), Fixed(0))
    }

    #[inline(always)]
    pub const fn one() -> Self {
        Vec4::new(Fixed::ONE, Fixed::ONE, Fixed::ONE, Fixed::ONE)
    }

    /// Dot product
    #[inline(always)]
    pub fn dot(self, rhs: Self) -> Fixed {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }

    /// Length squared (avoids sqrt)
    #[inline(always)]
    pub fn length_squared(self) -> Fixed {
        self.dot(self)
    }

    /// Length
    #[inline(always)]
    pub fn length(self) -> Fixed {
        use crate::fixed::advanced::sqrt;
        sqrt(self.length_squared())
    }

    /// Distance between two vectors
    #[inline(always)]
    pub fn distance(self, other: Self) -> Fixed {
        (self - other).length()
    }

    /// Normalize (returns zero vector if length is zero)
    #[inline(always)]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len.0 == 0 {
            return Vec4::zero();
        }
        self / len
    }

    // Swizzle accessors (GLSL-style) - scalar
    #[inline(always)]
    pub fn x(self) -> Fixed {
        self.x
    }
    #[inline(always)]
    pub fn y(self) -> Fixed {
        self.y
    }
    #[inline(always)]
    pub fn z(self) -> Fixed {
        self.z
    }
    #[inline(always)]
    pub fn w(self) -> Fixed {
        self.w
    }
    #[inline(always)]
    pub fn r(self) -> Fixed {
        self.x
    }
    #[inline(always)]
    pub fn g(self) -> Fixed {
        self.y
    }
    #[inline(always)]
    pub fn b(self) -> Fixed {
        self.z
    }
    #[inline(always)]
    pub fn a(self) -> Fixed {
        self.w
    }

    // 2-component swizzles (most common)
    #[inline(always)]
    pub fn xy(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    #[inline(always)]
    pub fn xz(self) -> Vec2 {
        Vec2::new(self.x, self.z)
    }
    #[inline(always)]
    pub fn xw(self) -> Vec2 {
        Vec2::new(self.x, self.w)
    }
    #[inline(always)]
    pub fn yz(self) -> Vec2 {
        Vec2::new(self.y, self.z)
    }
    #[inline(always)]
    pub fn yw(self) -> Vec2 {
        Vec2::new(self.y, self.w)
    }
    #[inline(always)]
    pub fn zw(self) -> Vec2 {
        Vec2::new(self.z, self.w)
    }

    // 3-component swizzles (most common)
    #[inline(always)]
    pub fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
    #[inline(always)]
    pub fn xyw(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.w)
    }
    #[inline(always)]
    pub fn xzw(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.w)
    }
    #[inline(always)]
    pub fn yzw(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.w)
    }

    // 4-component swizzle (identity)
    #[inline(always)]
    pub fn xyzw(self) -> Vec4 {
        self
    }

    // RGBA variants
    #[inline(always)]
    pub fn rg(self) -> Vec2 {
        self.xy()
    }
    #[inline(always)]
    pub fn rb(self) -> Vec2 {
        self.xz()
    }
    #[inline(always)]
    pub fn rgb(self) -> Vec3 {
        self.xyz()
    }
    #[inline(always)]
    pub fn rgba(self) -> Vec4 {
        self
    }

    /// Component-wise multiply
    #[inline(always)]
    pub fn mul_comp(self, rhs: Self) -> Self {
        Vec4::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w,
        )
    }

    /// Component-wise divide
    #[inline(always)]
    pub fn div_comp(self, rhs: Self) -> Self {
        Vec4::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
            self.w / rhs.w,
        )
    }

    /// Clamp components between min and max
    #[inline(always)]
    pub fn clamp(self, min: Fixed, max: Fixed) -> Self {
        Vec4::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
            self.w.clamp(min, max),
        )
    }
}

// Vector + Vector
impl Add for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Vec4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

// Vector - Vector
impl Sub for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Vec4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

// Vector * Scalar
impl Mul<Fixed> for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Fixed) -> Self {
        Vec4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

// Vector / Scalar
impl Div<Fixed> for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Fixed) -> Self {
        Vec4::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl Neg for Vec4 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Vec4::new(-self.x, -self.y, -self.z, -self.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec4::new(1.to_fixed(), 2.to_fixed(), 3.to_fixed(), 4.to_fixed());
        assert_eq!(v.x.to_f32(), 1.0);
        assert_eq!(v.y.to_f32(), 2.0);
        assert_eq!(v.z.to_f32(), 3.0);
        assert_eq!(v.w.to_f32(), 4.0);
    }

    #[test]
    fn test_from_f32() {
        let v = Vec4::from_f32(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.x.to_f32(), 1.0);
        assert_eq!(v.y.to_f32(), 2.0);
        assert_eq!(v.z.to_f32(), 3.0);
        assert_eq!(v.w.to_f32(), 4.0);
    }

    #[test]
    fn test_add() {
        let a = Vec4::from_f32(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::from_f32(5.0, 6.0, 7.0, 8.0);
        let c = a + b;
        assert_eq!(c.x.to_f32(), 6.0);
        assert_eq!(c.y.to_f32(), 8.0);
        assert_eq!(c.z.to_f32(), 10.0);
        assert_eq!(c.w.to_f32(), 12.0);
    }

    #[test]
    fn test_sub() {
        let a = Vec4::from_f32(5.0, 7.0, 9.0, 11.0);
        let b = Vec4::from_f32(1.0, 2.0, 3.0, 4.0);
        let c = a - b;
        assert_eq!(c.x.to_f32(), 4.0);
        assert_eq!(c.y.to_f32(), 5.0);
        assert_eq!(c.z.to_f32(), 6.0);
        assert_eq!(c.w.to_f32(), 7.0);
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec4::from_f32(1.0, 2.0, 3.0, 4.0);
        let s = 2.0.to_fixed();
        let result = v * s;
        assert_eq!(result.x.to_f32(), 2.0);
        assert_eq!(result.y.to_f32(), 4.0);
        assert_eq!(result.z.to_f32(), 6.0);
        assert_eq!(result.w.to_f32(), 8.0);
    }

    #[test]
    fn test_div_scalar() {
        let v = Vec4::from_f32(4.0, 6.0, 8.0, 10.0);
        let s = 2.0.to_fixed();
        let result = v / s;
        assert_eq!(result.x.to_f32(), 2.0);
        assert_eq!(result.y.to_f32(), 3.0);
        assert_eq!(result.z.to_f32(), 4.0);
        assert_eq!(result.w.to_f32(), 5.0);
    }

    #[test]
    fn test_dot() {
        let a = Vec4::from_f32(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::from_f32(5.0, 6.0, 7.0, 8.0);
        let dot = a.dot(b);
        // 1*5 + 2*6 + 3*7 + 4*8 = 5 + 12 + 21 + 32 = 70
        assert_eq!(dot.to_f32(), 70.0);
    }

    #[test]
    fn test_length_squared() {
        let v = Vec4::from_f32(1.0, 2.0, 2.0, 4.0);
        let len_sq = v.length_squared();
        // 1^2 + 2^2 + 2^2 + 4^2 = 1 + 4 + 4 + 16 = 25
        assert_eq!(len_sq.to_f32(), 25.0);
    }

    #[test]
    fn test_normalize() {
        let v = Vec4::from_f32(3.0, 0.0, 4.0, 0.0);
        // Length is 5, so normalized should have length ~1
        let n = v.normalize();

        // Check length is approximately 1
        let len = n.length();
        assert!((len.to_f32() - 1.0).abs() < 0.01); // Within tolerance
    }

    #[test]
    fn test_distance() {
        let a = Vec4::from_f32(0.0, 0.0, 0.0, 0.0);
        let b = Vec4::from_f32(1.0, 2.0, 2.0, 4.0);
        let dist = a.distance(b);
        // Distance should be 5
        assert_eq!(dist.to_f32(), 5.0);
    }

    #[test]
    fn test_mul_comp() {
        let a = Vec4::from_f32(2.0, 3.0, 4.0, 5.0);
        let b = Vec4::from_f32(6.0, 7.0, 8.0, 9.0);
        let c = a.mul_comp(b);
        assert_eq!(c.x.to_f32(), 12.0);
        assert_eq!(c.y.to_f32(), 21.0);
        assert_eq!(c.z.to_f32(), 32.0);
        assert_eq!(c.w.to_f32(), 45.0);
    }

    #[test]
    fn test_zero_one() {
        let z = Vec4::zero();
        assert_eq!(z.x.to_f32(), 0.0);
        assert_eq!(z.y.to_f32(), 0.0);
        assert_eq!(z.z.to_f32(), 0.0);
        assert_eq!(z.w.to_f32(), 0.0);

        let o = Vec4::one();
        assert_eq!(o.x.to_f32(), 1.0);
        assert_eq!(o.y.to_f32(), 1.0);
        assert_eq!(o.z.to_f32(), 1.0);
        assert_eq!(o.w.to_f32(), 1.0);
    }
}
