/// Fixed-point arithmetic (16.16 format)
/// 
/// Core type and conversion utilities for fixed-point math.

pub type Fixed = i32;

/// Fixed-point constants
pub const SHIFT: i32 = 16;
pub const ONE: Fixed = 1 << SHIFT;
pub const HALF: Fixed = ONE / 2;
pub const ZERO: Fixed = 0;

/// Convert integer to fixed-point
#[inline(always)]
pub const fn from_int(n: i32) -> Fixed {
    n << SHIFT
}

/// Convert fixed-point to integer (truncate)
#[inline(always)]
pub const fn to_int(f: Fixed) -> i32 {
    f >> SHIFT
}

/// Convert f32 to fixed-point
#[inline]
pub fn from_f32(f: f32) -> Fixed {
    (f * 65536.0) as Fixed
}

/// Convert fixed-point to f32
#[inline]
pub fn to_f32(f: Fixed) -> f32 {
    (f as f32) / 65536.0
}

/// Fixed-point multiplication
#[inline(always)]
pub fn mul(a: Fixed, b: Fixed) -> Fixed {
    ((a as i64 * b as i64) >> SHIFT) as i32
}

/// Fixed-point division
#[inline(always)]
pub fn div(a: Fixed, b: Fixed) -> Fixed {
    if b == 0 {
        return 0;
    }
    (((a as i64) << SHIFT) / b as i64) as i32
}

/// Get fractional part
#[inline(always)]
pub fn frac(f: Fixed) -> Fixed {
    f & (ONE - 1)
}

/// Floor (round down to integer)
#[inline(always)]
pub fn floor(f: Fixed) -> Fixed {
    f & !(ONE - 1)
}

/// Ceiling (round up to integer)
#[inline(always)]
pub fn ceil(f: Fixed) -> Fixed {
    let frac_part = frac(f);
    if frac_part > 0 {
        floor(f) + ONE
    } else {
        f
    }
}

/// Clamp value between min and max
#[inline(always)]
pub fn clamp(val: Fixed, min: Fixed, max: Fixed) -> Fixed {
    val.max(min).min(max)
}

/// Linear interpolation
/// lerp(a, b, t) = a + (b - a) * t
#[inline(always)]
pub fn lerp(a: Fixed, b: Fixed, t: Fixed) -> Fixed {
    a + mul(b - a, t)
}

/// Sign function: returns -1, 0, or 1
#[inline(always)]
pub fn sign(a: Fixed) -> Fixed {
    if a > 0 { ONE } else if a < 0 { -ONE } else { ZERO }
}

/// Saturate: clamp to 0..1
#[inline(always)]
pub fn saturate(a: Fixed) -> Fixed {
    clamp(a, ZERO, ONE)
}

/// Integer square root for fixed-point
pub fn sqrt(a: Fixed) -> Fixed {
    if a <= 0 {
        return 0;
    }
    
    let mut result = 0i32;
    let mut bit = 1i32 << 30;
    
    while bit > a {
        bit >>= 2;
    }
    
    while bit != 0 {
        if a >= result + bit {
            result = (result >> 1) + bit;
        } else {
            result >>= 1;
        }
        bit >>= 2;
    }
    
    result << (SHIFT / 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conversions() {
        assert_eq!(from_int(5), 5 << 16);
        assert_eq!(to_int(from_int(5)), 5);
        
        let f = from_f32(1.5);
        assert!((to_f32(f) - 1.5).abs() < 0.001);
    }
    
    #[test]
    fn test_arithmetic() {
        let a = from_int(2);
        let b = from_int(3);
        assert_eq!(to_int(mul(a, b)), 6);
        assert_eq!(to_int(div(a, b)), 0);
        assert_eq!(to_int(div(b, a)), 1);
    }
    
    #[test]
    fn test_frac() {
        let f = from_f32(1.25);
        assert_eq!(to_f32(frac(f)), 0.25);
    }
}

