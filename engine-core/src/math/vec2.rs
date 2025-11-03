/// 2D vector for fixed-point coordinates
use super::dec::{Dec, Fixed, FIXED_SHIFT};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: Dec,
    pub y: Dec,
}

impl Vec2 {
    #[inline(always)]
    pub const fn new(x: Dec, y: Dec) -> Self {
        Vec2 { x, y }
    }

    #[inline(always)]
    pub const fn from_fixed(x: Fixed, y: Fixed) -> Self {
        Vec2 {
            x: Dec(x),
            y: Dec(y),
        }
    }

    #[inline(always)]
    pub fn from_f32(x: f32, y: f32) -> Self {
        Vec2 {
            x: Dec::from_f32(x),
            y: Dec::from_f32(y),
        }
    }

    #[inline(always)]
    pub fn from_int(x: i32, y: i32) -> Self {
        Vec2 {
            x: Dec::from_int(x),
            y: Dec::from_int(y),
        }
    }

    /// Create from pixel coordinates with center offset (pixel center is at +0.5)
    #[inline(always)]
    pub const fn from_pixel(x: usize, y: usize) -> Self {
        Vec2 {
            x: Dec(((x as i32) << FIXED_SHIFT) + (Dec::HALF.0)),
            y: Dec(((y as i32) << FIXED_SHIFT) + (Dec::HALF.0)),
        }
    }

    #[inline(always)]
    pub fn to_int_coords(self) -> (usize, usize) {
        ((self.x.0 >> FIXED_SHIFT) as usize, (self.y.0 >> FIXED_SHIFT) as usize)
    }
}

use core::ops::{Add, Sub};

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

