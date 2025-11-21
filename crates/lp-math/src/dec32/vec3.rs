use core::ops::{Add, Div, Mul, Neg, Sub};

/// 3D vector for dec32-point dec32
use super::conversions::ToDec32;
use super::dec32::Dec32;
use super::vec2::Vec2;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: Dec32,
    pub y: Dec32,
    pub z: Dec32,
}

impl Vec3 {
    #[inline(always)]
    pub const fn new(x: Dec32, y: Dec32, z: Dec32) -> Self {
        Vec3 { x, y, z }
    }

    #[inline(always)]
    pub fn from_f32(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            x: x.to_dec32(),
            y: y.to_dec32(),
            z: z.to_dec32(),
        }
    }

    #[inline(always)]
    pub fn from_i32(x: i32, y: i32, z: i32) -> Self {
        Vec3 {
            x: x.to_dec32(),
            y: y.to_dec32(),
            z: z.to_dec32(),
        }
    }

    #[inline(always)]
    pub const fn zero() -> Self {
        Vec3::new(Dec32(0), Dec32(0), Dec32(0))
    }

    #[inline(always)]
    pub const fn one() -> Self {
        Vec3::new(Dec32::ONE, Dec32::ONE, Dec32::ONE)
    }

    /// Dot product
    #[inline(always)]
    pub fn dot(self, rhs: Self) -> Dec32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Cross product
    #[inline(always)]
    pub fn cross(self, rhs: Self) -> Self {
        Vec3::new(
            (self.y * rhs.z) - (self.z * rhs.y),
            (self.z * rhs.x) - (self.x * rhs.z),
            (self.x * rhs.y) - (self.y * rhs.x),
        )
    }

    /// Length squared (avoids sqrt)
    #[inline(always)]
    pub fn length_squared(self) -> Dec32 {
        self.dot(self)
    }

    /// Length
    #[inline(always)]
    pub fn length(self) -> Dec32 {
        use crate::dec32::advanced::sqrt;
        sqrt(self.length_squared())
    }

    /// Distance between two vectors
    #[inline(always)]
    pub fn distance(self, other: Self) -> Dec32 {
        (self - other).length()
    }

    /// Normalize (returns zero vector if length is zero)
    #[inline(always)]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len.0 == 0 {
            return Vec3::zero();
        }
        self / len
    }

    // Swizzle accessors (GLSL-style) - scalar
    #[inline(always)]
    pub fn x(self) -> Dec32 {
        self.x
    }

    #[inline(always)]
    pub fn y(self) -> Dec32 {
        self.y
    }

    #[inline(always)]
    pub fn z(self) -> Dec32 {
        self.z
    }

    #[inline(always)]
    pub fn r(self) -> Dec32 {
        self.x
    }

    #[inline(always)]
    pub fn g(self) -> Dec32 {
        self.y
    }

    #[inline(always)]
    pub fn b(self) -> Dec32 {
        self.z
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
    pub fn yz(self) -> Vec2 {
        Vec2::new(self.y, self.z)
    }

    #[inline(always)]
    pub fn yx(self) -> Vec2 {
        Vec2::new(self.y, self.x)
    }

    #[inline(always)]
    pub fn zx(self) -> Vec2 {
        Vec2::new(self.z, self.x)
    }

    #[inline(always)]
    pub fn zy(self) -> Vec2 {
        Vec2::new(self.z, self.y)
    }

    // 3-component swizzles (permutations)
    #[inline(always)]
    pub fn xyz(self) -> Vec3 {
        self
    }

    // identity
    #[inline(always)]
    pub fn xzy(self) -> Vec3 {
        Vec3::new(self.x, self.z, self.y)
    }

    #[inline(always)]
    pub fn yxz(self) -> Vec3 {
        Vec3::new(self.y, self.x, self.z)
    }

    #[inline(always)]
    pub fn yzx(self) -> Vec3 {
        Vec3::new(self.y, self.z, self.x)
    }

    #[inline(always)]
    pub fn zxy(self) -> Vec3 {
        Vec3::new(self.z, self.x, self.y)
    }

    #[inline(always)]
    pub fn zyx(self) -> Vec3 {
        Vec3::new(self.z, self.y, self.x)
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
    pub fn gb(self) -> Vec2 {
        self.yz()
    }

    #[inline(always)]
    pub fn rgb(self) -> Vec3 {
        self
    }

    /// Component-wise multiply
    #[inline(always)]
    pub fn mul_comp(self, rhs: Self) -> Self {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

    /// Component-wise divide
    #[inline(always)]
    pub fn div_comp(self, rhs: Self) -> Self {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }

    /// Clamp components between min and max
    #[inline(always)]
    pub fn clamp(self, min: Dec32, max: Dec32) -> Self {
        Vec3::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        )
    }

    /// Reflect vector around normal
    #[inline(always)]
    pub fn reflect(self, normal: Self) -> Self {
        // reflect = v - 2 * dot(v, n) * n
        let dot_2 = self.dot(normal) * Dec32(2 << 16);
        self - (normal * dot_2)
    }
}

// Vector + Vector
impl Add for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// Vector - Vector
impl Sub for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// Vector * Scalar
impl Mul<Dec32> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Dec32) -> Self {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// Vector / Scalar
impl Div<Dec32> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Dec32) -> Self {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3::new(1.to_dec32(), 2.to_dec32(), 3.to_dec32());
        assert_eq!(v.x.to_f32(), 1.0);
        assert_eq!(v.y.to_f32(), 2.0);
        assert_eq!(v.z.to_f32(), 3.0);
    }

    #[test]
    fn test_from_f32() {
        let v = Vec3::from_f32(1.0, 2.0, 3.0);
        assert_eq!(v.x.to_f32(), 1.0);
        assert_eq!(v.y.to_f32(), 2.0);
        assert_eq!(v.z.to_f32(), 3.0);
    }

    #[test]
    fn test_add() {
        let a = Vec3::from_f32(1.0, 2.0, 3.0);
        let b = Vec3::from_f32(4.0, 5.0, 6.0);
        let c = a + b;
        assert_eq!(c.x.to_f32(), 5.0);
        assert_eq!(c.y.to_f32(), 7.0);
        assert_eq!(c.z.to_f32(), 9.0);
    }

    #[test]
    fn test_sub() {
        let a = Vec3::from_f32(5.0, 7.0, 9.0);
        let b = Vec3::from_f32(1.0, 2.0, 3.0);
        let c = a - b;
        assert_eq!(c.x.to_f32(), 4.0);
        assert_eq!(c.y.to_f32(), 5.0);
        assert_eq!(c.z.to_f32(), 6.0);
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec3::from_f32(1.0, 2.0, 3.0);
        let s = 2.0.to_dec32();
        let result = v * s;
        assert_eq!(result.x.to_f32(), 2.0);
        assert_eq!(result.y.to_f32(), 4.0);
        assert_eq!(result.z.to_f32(), 6.0);
    }

    #[test]
    fn test_div_scalar() {
        let v = Vec3::from_f32(4.0, 6.0, 8.0);
        let s = 2.0.to_dec32();
        let result = v / s;
        assert_eq!(result.x.to_f32(), 2.0);
        assert_eq!(result.y.to_f32(), 3.0);
        assert_eq!(result.z.to_f32(), 4.0);
    }

    #[test]
    fn test_dot() {
        let a = Vec3::from_f32(1.0, 2.0, 3.0);
        let b = Vec3::from_f32(4.0, 5.0, 6.0);
        let dot = a.dot(b);
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert_eq!(dot.to_f32(), 32.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::from_f32(1.0, 0.0, 0.0);
        let b = Vec3::from_f32(0.0, 1.0, 0.0);
        let c = a.cross(b);
        // (1,0,0) × (0,1,0) = (0,0,1)
        assert_eq!(c.x.to_f32(), 0.0);
        assert_eq!(c.y.to_f32(), 0.0);
        assert_eq!(c.z.to_f32(), 1.0);
    }

    #[test]
    fn test_length_squared() {
        let v = Vec3::from_f32(2.0, 3.0, 6.0);
        let len_sq = v.length_squared();
        // 2^2 + 3^2 + 6^2 = 4 + 9 + 36 = 49
        assert_eq!(len_sq.to_f32(), 49.0);
    }

    #[test]
    fn test_normalize() {
        let v = Vec3::from_f32(3.0, 0.0, 4.0);
        // Length is 5, so normalized should be (0.6, 0, 0.8)
        let n = v.normalize();

        // Check length is approximately 1
        let len = n.length();
        assert!((len.to_f32() - 1.0).abs() < 0.01); // Within tolerance
    }

    #[test]
    fn test_distance() {
        let a = Vec3::from_f32(0.0, 0.0, 0.0);
        let b = Vec3::from_f32(3.0, 0.0, 4.0);
        let dist = a.distance(b);
        // Distance should be 5
        assert_eq!(dist.to_f32(), 5.0);
    }

    #[test]
    fn test_mul_comp() {
        let a = Vec3::from_f32(2.0, 3.0, 4.0);
        let b = Vec3::from_f32(5.0, 6.0, 7.0);
        let c = a.mul_comp(b);
        assert_eq!(c.x.to_f32(), 10.0);
        assert_eq!(c.y.to_f32(), 18.0);
        assert_eq!(c.z.to_f32(), 28.0);
    }

    #[test]
    fn test_zero_one() {
        let z = Vec3::zero();
        assert_eq!(z.x.to_f32(), 0.0);
        assert_eq!(z.y.to_f32(), 0.0);
        assert_eq!(z.z.to_f32(), 0.0);

        let o = Vec3::one();
        assert_eq!(o.x.to_f32(), 1.0);
        assert_eq!(o.y.to_f32(), 1.0);
        assert_eq!(o.z.to_f32(), 1.0);
    }
}
