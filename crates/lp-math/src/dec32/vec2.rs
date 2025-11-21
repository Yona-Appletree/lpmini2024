use core::ops::{Add, Div, Mul, Neg, Sub};

/// 2D vector for dec32-point coordinates
use super::conversions::ToDec32;
use super::dec32::Dec32;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: Dec32,
    pub y: Dec32,
}

impl Vec2 {
    #[inline(always)]
    pub const fn new(x: Dec32, y: Dec32) -> Self {
        Vec2 { x, y }
    }

    #[inline(always)]
    pub fn from_f32(x: f32, y: f32) -> Self {
        Vec2 {
            x: x.to_dec32(),
            y: y.to_dec32(),
        }
    }

    #[inline(always)]
    pub fn from_i32(x: i32, y: i32) -> Self {
        Vec2 {
            x: x.to_dec32(),
            y: y.to_dec32(),
        }
    }

    /// Create from pixel coordinates with center offset (pixel center is at +0.5)
    #[inline(always)]
    pub const fn from_pixel(x: usize, y: usize) -> Self {
        Vec2 {
            x: Dec32(((x as i32) << Dec32::SHIFT) + (Dec32::HALF.0)),
            y: Dec32(((y as i32) << Dec32::SHIFT) + (Dec32::HALF.0)),
        }
    }

    #[inline(always)]
    pub fn to_int_coords(self) -> (usize, usize) {
        (self.x.to_i32() as usize, self.y.to_i32() as usize)
    }

    /// Dot product
    #[inline(always)]
    pub fn dot(self, rhs: Self) -> Dec32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    /// Cross product (returns scalar in 2D, representing z-component of 3D cross product)
    #[inline(always)]
    pub fn cross(self, rhs: Self) -> Dec32 {
        (self.x * rhs.y) - (self.y * rhs.x)
    }

    /// Length squared (avoids sqrt)
    #[inline(always)]
    pub fn length_squared(self) -> Dec32 {
        self.dot(self)
    }

    /// Length (magnitude)
    #[inline(always)]
    pub fn length(self) -> Dec32 {
        use crate::dec32::advanced::sqrt;
        sqrt(self.length_squared())
    }

    /// Distance to another vector
    #[inline(always)]
    pub fn distance(self, other: Self) -> Dec32 {
        (self - other).length()
    }

    /// Normalize to unit vector
    #[inline(always)]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len.0 == 0 {
            return Vec2::new(Dec32::ZERO, Dec32::ZERO);
        }
        self / len
    }

    // Swizzle accessors (GLSL-style)
    #[inline(always)]
    pub fn x(self) -> Dec32 {
        self.x
    }

    #[inline(always)]
    pub fn y(self) -> Dec32 {
        self.y
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
    pub fn s(self) -> Dec32 {
        self.x
    }

    #[inline(always)]
    pub fn t(self) -> Dec32 {
        self.y
    }

    // 2-component swizzles (most common)
    #[inline(always)]
    pub fn xx(self) -> Vec2 {
        Vec2::new(self.x, self.x)
    }

    #[inline(always)]
    pub fn xy(self) -> Vec2 {
        self
    }

    // identity
    #[inline(always)]
    pub fn yx(self) -> Vec2 {
        Vec2::new(self.y, self.x)
    }

    #[inline(always)]
    pub fn yy(self) -> Vec2 {
        Vec2::new(self.y, self.y)
    }

    // RGBA variants
    #[inline(always)]
    pub fn rr(self) -> Vec2 {
        self.xx()
    }

    #[inline(always)]
    pub fn rg(self) -> Vec2 {
        self.xy()
    }

    #[inline(always)]
    pub fn gr(self) -> Vec2 {
        self.yx()
    }

    #[inline(always)]
    pub fn gg(self) -> Vec2 {
        self.yy()
    }

    // STPQ variants
    #[inline(always)]
    pub fn ss(self) -> Vec2 {
        self.xx()
    }

    #[inline(always)]
    pub fn st(self) -> Vec2 {
        self.xy()
    }

    #[inline(always)]
    pub fn ts(self) -> Vec2 {
        self.yx()
    }

    #[inline(always)]
    pub fn tt(self) -> Vec2 {
        self.yy()
    }

    /// Component-wise multiply
    #[inline(always)]
    pub fn mul_comp(self, rhs: Self) -> Self {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }

    /// Component-wise divide
    #[inline(always)]
    pub fn div_comp(self, rhs: Self) -> Self {
        Vec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

// Vector * Scalar
impl Mul<Dec32> for Vec2 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Dec32) -> Self {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

// Vector / Scalar
impl Div<Dec32> for Vec2 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Dec32) -> Self {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        Vec2::new(-self.x, -self.y)
    }
}
