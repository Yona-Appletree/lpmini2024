/// RGB packing/unpacking utilities for 32-bit buffers
use lp_script::dec32::{Dec32, ToDec32};

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

/// Convert greyscale dec32-point to i32 (stores as-is in lower bits)
#[inline(always)]
pub fn grey_to_i32(grey: Dec32) -> i32 {
    grey.0
}

/// Extract greyscale from i32 (reads as-is from lower bits)
#[inline(always)]
pub fn i32_to_grey(val: i32) -> Dec32 {
    Dec32(val)
}

/// Convert greyscale dec32-point to RGB (grey, grey, grey) packed as i32
#[inline(always)]
pub fn grey_to_rgb_i32(grey: Dec32) -> i32 {
    let clamped: Dec32 = grey.clamp(Dec32::ZERO, Dec32::ONE);
    // Convert to 0-255 range: clamped * 255
    let byte_val: Dec32 = clamped * 255i32.to_dec32();
    let byte_val_u8 = byte_val.to_i32().clamp(0, 255) as u8;
    pack_rgb(byte_val_u8, byte_val_u8, byte_val_u8)
}

#[cfg(all(test, not(feature = "use-libm")))]
mod tests {
    use lp_script::dec32::ToDec32;

    use super::*;

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
        let grey = 0.5f32.to_dec32();
        let i32_val = grey_to_i32(grey);
        let back = i32_to_grey(i32_val);
        assert_eq!(back, grey);
    }

    #[test]
    fn test_grey_to_rgb() {
        // 0.0 should be black
        let black = grey_to_rgb_i32(Dec32::ZERO);
        let (r, g, b) = unpack_rgb(black);
        assert_eq!((r, g, b), (0, 0, 0));

        // 1.0 should be white
        let white = grey_to_rgb_i32(Dec32::ONE);
        let (r, g, b) = unpack_rgb(white);
        assert_eq!((r, g, b), (255, 255, 255));

        // 0.5 should be mid-grey
        let grey = grey_to_rgb_i32(Dec32::HALF);
        let (r, g, b) = unpack_rgb(grey);
        assert_eq!((r, g, b), (127, 127, 127));
    }
}
