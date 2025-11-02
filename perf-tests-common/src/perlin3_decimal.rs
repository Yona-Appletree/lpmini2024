/// Perlin3 noise using a Decimal wrapper type
/// This provides clean syntax and can be swapped between fixed/float implementations
use crate::sin_table::SIN_TABLE_I32 as SIN_TABLE;


// === Decimal Type Definition ===
// Change this to switch between fixed-point and float

/// Decimal type - currently using fixed-point 16.16
#[derive(Copy, Clone, Debug)]
pub struct Decimal(pub(crate) i32); // pub(crate) for tests

const SHIFT: i32 = 16;
const ONE: i32 = 1 << SHIFT;

impl Decimal {
    #[inline(always)]
    pub fn from_f32(f: f32) -> Self {
        Decimal((f * ONE as f32) as i32)
    }

    #[inline(always)]
    pub fn from_int(i: i32) -> Self {
        Decimal(i << SHIFT)
    }

    #[inline(always)]
    pub fn to_u8(self) -> u8 {
        ((self.0 >> SHIFT).clamp(0, 255)) as u8
    }

    /// Get the raw fixed-point value (for testing)
    #[inline(always)]
    pub fn to_f32(self) -> f32 {
        self.0 as f32 / ONE as f32
    }

    #[inline(always)]
    pub fn sin(self) -> Self {
        // Normalize to 0-2π and lookup
        const TWO_PI: i64 = 411775;
        let normalized = ((self.0 as i64).rem_euclid(TWO_PI)) as i32;
        let index = ((normalized as i64 * 256) / TWO_PI) as usize & 0xFF;
        Decimal(SIN_TABLE[index])
    }

    #[inline(always)]
    pub fn cos(self) -> Self {
        const PI_DIV_2: i32 = 102944;
        Decimal(self.0 + PI_DIV_2).sin()
    }

    #[inline(always)]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Decimal(self.0.clamp(min.0, max.0))
    }
}

// === Operator Overloading for Clean Syntax ===

use core::ops::{Add, Div, Mul, Sub};

impl Add for Decimal {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Decimal(self.0 + rhs.0)
    }
}

impl Sub for Decimal {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Decimal(self.0 - rhs.0)
    }
}

impl Mul for Decimal {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Decimal(((self.0 as i64 * rhs.0 as i64) >> SHIFT) as i32)
    }
}

impl Mul<i32> for Decimal {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: i32) -> Self {
        Decimal(self.0 * rhs)
    }
}

impl Div for Decimal {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Decimal(((self.0 as i64 * ONE as i64) / rhs.0 as i64) as i32)
    }
}

impl Div<i32> for Decimal {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: i32) -> Self {
        Decimal(self.0 / rhs)
    }
}

// === Constants ===

impl Decimal {
    pub const ZERO: Self = Decimal(0);
    pub const ONE: Self = Decimal(ONE);
}

// === Perlin3 Implementation ===

#[inline(always)]
fn perlin3(x: Decimal, y: Decimal, z: Decimal) -> Decimal {
    let freq1 = Decimal::ONE;
    let freq2 = Decimal::ONE * 2;
    let freq3 = Decimal::ONE * 4;

    let n1 = (x * freq1 + z).sin() * (y * freq1).cos();
    let n2 = (x * freq2 - z).sin() * (y * freq2).cos() / 2;
    let n3 = (x * freq3 + y + z).sin() / 4;

    let sum = n1 + n2 + n3;

    // Divide by 1.75 ≈ multiply by 0.5714
    sum * Decimal((0.5714 * ONE as f32) as i32)
}

#[inline(never)]
pub fn render_frame(buffer: &mut [u8], time: f32, width: usize, height: usize) {
    let width_d = Decimal::from_int(width as i32);
    let height_d = Decimal::from_int(height as i32);
    let time_d = Decimal::from_f32(time * 0.001);

    for y in 0..height {
        for x in 0..width {
            let x_d = Decimal::from_int(x as i32);
            let y_d = Decimal::from_int(y as i32);

            // Clean, readable math!
            let nx = (x_d * 4) / width_d;
            let ny = (y_d * 4) / height_d;
            let nz = time_d;

            let noise = perlin3(nx, ny, nz);

            // Convert from -1.0 to 1.0 range to 0-255
            let normalized = (noise + Decimal::ONE) * Decimal::from_int(128);
            let value = normalized
                .clamp(Decimal::ZERO, Decimal::from_int(255))
                .to_u8();

            let idx = (y * width + x) * 3;
            buffer[idx] = value;
            buffer[idx + 1] = value;
            buffer[idx + 2] = value;
        }
    }
}
