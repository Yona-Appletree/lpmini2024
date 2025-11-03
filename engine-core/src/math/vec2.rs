/// 2D vector for fixed-point coordinates
use super::fixed::{Fixed, SHIFT};
use super::conversions::ToFixed;
use core::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: Fixed,
    pub y: Fixed,
}

impl Vec2 {
    #[inline(always)]
    pub const fn new(x: Fixed, y: Fixed) -> Self {
        Vec2 { x, y }
    }

    #[inline(always)]
    pub fn from_f32(x: f32, y: f32) -> Self {
        Vec2 {
            x: x.to_fixed(),
            y: y.to_fixed(),
        }
    }

    #[inline(always)]
    pub fn from_i32(x: i32, y: i32) -> Self {
        Vec2 {
            x: x.to_fixed(),
            y: y.to_fixed(),
        }
    }

    /// Create from pixel coordinates with center offset (pixel center is at +0.5)
    #[inline(always)]
    pub const fn from_pixel(x: usize, y: usize) -> Self {
        Vec2 {
            x: Fixed(((x as i32) << SHIFT) + (Fixed::HALF.0)),
            y: Fixed(((y as i32) << SHIFT) + (Fixed::HALF.0)),
        }
    }

    #[inline(always)]
    pub fn to_int_coords(self) -> (usize, usize) {
        (
            (self.x.0 >> SHIFT) as usize,
            (self.y.0 >> SHIFT) as usize,
        )
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
