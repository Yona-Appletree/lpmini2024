/// Fixed-point decimal type (16.16 format)

pub type Fixed = i32;

pub const FIXED_SHIFT: i32 = 16;
pub const FIXED_ONE: Fixed = 1 << FIXED_SHIFT;

/// Decimal fixed-point wrapper for clean arithmetic
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dec(pub Fixed);

impl Dec {
    pub const ZERO: Dec = Dec(0);
    pub const ONE: Dec = Dec(FIXED_ONE);
    pub const HALF: Dec = Dec(FIXED_ONE >> 1);

    #[inline(always)]
    pub const fn from_fixed(f: Fixed) -> Self {
        Dec(f)
    }

    #[inline(always)]
    pub fn from_f32(f: f32) -> Self {
        Dec((f * FIXED_ONE as f32) as Fixed)
    }

    #[inline(always)]
    pub fn from_int(i: i32) -> Self {
        Dec(i << FIXED_SHIFT)
    }

    #[inline(always)]
    pub fn to_f32(self) -> f32 {
        self.0 as f32 / FIXED_ONE as f32
    }

    #[inline(always)]
    pub fn to_fixed(self) -> Fixed {
        self.0
    }

    #[inline(always)]
    pub fn mul(self, other: Dec) -> Dec {
        Dec(((self.0 as i64 * other.0 as i64) >> FIXED_SHIFT) as Fixed)
    }

    #[inline(always)]
    pub fn div(self, other: Dec) -> Dec {
        if other.0 != 0 {
            Dec(((self.0 as i64 * FIXED_ONE as i64) / other.0 as i64) as Fixed)
        } else {
            Dec(0)
        }
    }

    #[inline(always)]
    pub fn clamp(self, min: Dec, max: Dec) -> Dec {
        Dec(self.0.clamp(min.0, max.0))
    }

    #[inline(always)]
    pub fn max(self, other: Dec) -> Dec {
        Dec(self.0.max(other.0))
    }

    #[inline(always)]
    pub fn min(self, other: Dec) -> Dec {
        Dec(self.0.min(other.0))
    }
}

use core::ops::{Add, Sub, Mul, Div, Neg};

impl Add for Dec {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Dec(self.0 + rhs.0)
    }
}

impl Sub for Dec {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Dec(self.0 - rhs.0)
    }
}

impl Mul for Dec {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        self.mul(rhs)
    }
}

impl Div for Dec {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        self.div(rhs)
    }
}

impl Neg for Dec {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Dec(-self.0)
    }
}

// Conversions for convenience
#[inline(always)]
pub fn fixed_from_f32(f: f32) -> Fixed {
    (f * FIXED_ONE as f32) as Fixed
}

#[inline(always)]
pub fn fixed_to_f32(f: Fixed) -> f32 {
    f as f32 / FIXED_ONE as f32
}

#[inline(always)]
pub fn fixed_mul(a: Fixed, b: Fixed) -> Fixed {
    ((a as i64 * b as i64) >> FIXED_SHIFT) as Fixed
}

#[inline(always)]
pub fn fixed_div(a: Fixed, b: Fixed) -> Fixed {
    if b != 0 {
        ((a as i64 * FIXED_ONE as i64) / b as i64) as Fixed
    } else {
        0
    }
}

