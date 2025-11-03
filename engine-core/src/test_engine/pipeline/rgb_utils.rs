/// RGB packing/unpacking utilities for 32-bit buffers
use crate::math::{Fixed, FIXED_SHIFT, FIXED_ONE};
use core::cmp::{max, min};

/// Pack RGB into 0x00RRGGBB format
#[inline(always)]
pub fn pack_rgb(r: u8, g: u8, b: u8) -> i32 {
    ((r as i32) << 16) | ((g as i32) << 8) | (b as i32)
}

/// Unpack RGB from 0x00RRGGBB format
#[inline(always)]
pub fn unpack_rgb(val: i32) -> (u8, u8, u8) {
    let r = ((val >> 16) & 0xFF) as u8;
    let g = ((val >> 8) & 0xFF) as u8;
    let b = (val & 0xFF) as u8;
    (r, g, b)
}

/// Convert greyscale fixed-point to i32 (stores as-is in lower bits)
#[inline(always)]
pub fn grey_to_i32(grey: Fixed) -> i32 {
    grey
}

/// Extract greyscale from i32 (reads as-is from lower bits)
#[inline(always)]
pub fn i32_to_grey(val: i32) -> Fixed {
    val
}

/// Convert greyscale fixed-point to RGB (grey, grey, grey) packed as i32
#[inline(always)]
pub fn grey_to_rgb_i32(grey: Fixed) -> i32 {
    let clamped = if grey < 0 { 0 } else if grey > FIXED_ONE { FIXED_ONE } else { grey };
    let byte_val = ((clamped >> (FIXED_SHIFT - 8)) & 0xFF) as u8;
    pack_rgb(byte_val, byte_val, byte_val)
}

#[cfg(all(test, not(feature = "use-libm")))]
mod tests {
    use super::*;
    use super::super::super::vm::fixed_from_f32;

    #[test]
    fn test_pack_unpack_rgb() {
        let packed = pack_rgb(255, 128, 64);
        assert_eq!(packed, 0x00FF8040);
        
        let (r, g, b) = unpack_rgb(packed);
        assert_eq!(r, 255);
        assert_eq!(g, 128);
        assert_eq!(b, 64);
    }

    #[test]
    fn test_grey_conversion() {
        let grey = fixed_from_f32(0.5);
        let i32_val = grey_to_i32(grey);
        let back = i32_to_grey(i32_val);
        assert_eq!(back, grey);
    }

    #[test]
    #[ignore] // TODO: Fix for integer-only min/max
    fn test_grey_to_rgb() {
        // 0.0 should be black
        let black = grey_to_rgb_i32(0);
        let (r, g, b) = unpack_rgb(black);
        assert_eq!((r, g, b), (0, 0, 0));
        
        // 1.0 should be white
        let white = grey_to_rgb_i32(FIXED_ONE);
        let (r, g, b) = unpack_rgb(white);
        assert_eq!((r, g, b), (255, 255, 255));
        
        // 0.5 should be mid-grey
        let grey = grey_to_rgb_i32(FIXED_ONE / 2);
        let (r, g, b) = unpack_rgb(grey);
        assert_eq!((r, g, b), (127, 127, 127));
    }
}

