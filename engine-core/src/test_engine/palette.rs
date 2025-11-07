/// Palette-based RGB conversion
use lpscript::math::{Fixed, FIXED_SHIFT, FIXED_ONE};

/// RGB color representation
#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }
}

/// 16-entry palette for color mapping
#[derive(Clone)]
pub struct Palette {
    colors: [Rgb; 16],
}

impl Palette {
    /// Create a new palette from an array of colors
    pub fn new(colors: [Rgb; 16]) -> Self {
        Palette { colors }
    }

    /// Create a rainbow palette
    pub fn rainbow() -> Self {
        Palette {
            colors: [
                Rgb::new(255, 0, 0),     // Red
                Rgb::new(255, 64, 0),    // Red-Orange
                Rgb::new(255, 128, 0),   // Orange
                Rgb::new(255, 192, 0),   // Orange-Yellow
                Rgb::new(255, 255, 0),   // Yellow
                Rgb::new(192, 255, 0),   // Yellow-Green
                Rgb::new(128, 255, 0),   // Light Green
                Rgb::new(0, 255, 0),     // Green
                Rgb::new(0, 255, 128),   // Green-Cyan
                Rgb::new(0, 255, 255),   // Cyan
                Rgb::new(0, 128, 255),   // Cyan-Blue
                Rgb::new(0, 0, 255),     // Blue
                Rgb::new(128, 0, 255),   // Blue-Purple
                Rgb::new(192, 0, 255),   // Purple
                Rgb::new(255, 0, 192),   // Purple-Magenta
                Rgb::new(255, 0, 128),   // Magenta-Red
            ],
        }
    }

    /// Create a grayscale palette (black to white)
    pub fn grayscale() -> Self {
        let mut colors = [Rgb::new(0, 0, 0); 16];
        for i in 0..16 {
            let val = (i * 255 / 15) as u8;
            colors[i] = Rgb::new(val, val, val);
        }
        Palette { colors }
    }

    /// Get interpolated color for a value in range [0, 1] (fixed-point)
    #[inline(always)]
    pub fn get_color(&self, value: Fixed) -> Rgb {
        // Clamp value to 0..1 range
        let clamped = if value.0 < 0 { 
            0 
        } else if value.0 > FIXED_ONE { 
            FIXED_ONE 
        } else { 
            value.0 
        };
        
        // Map to palette range [0, 15]
        // value * 15.0 in fixed-point
        let scaled = ((clamped as i64 * 15) >> FIXED_SHIFT) as i32;
        let index = if scaled > 14 { 14 } else { scaled as usize }; // Max index is 14 for interpolation
        
        // Get fractional part for interpolation (0..FIXED_ONE)
        // frac = (value * 15) - floor(value * 15)
        let frac_fixed = (clamped * 15) - (index as i32 * FIXED_ONE);
        
        // Interpolate between current and next color using fixed-point
        // result = c1 + (c2 - c1) * frac
        let c1 = &self.colors[index];
        let c2 = &self.colors[index + 1];
        
        let r = c1.r as i32 + (((c2.r as i32 - c1.r as i32) * frac_fixed) >> FIXED_SHIFT);
        let g = c1.g as i32 + (((c2.g as i32 - c1.g as i32) * frac_fixed) >> FIXED_SHIFT);
        let b = c1.b as i32 + (((c2.b as i32 - c1.b as i32) * frac_fixed) >> FIXED_SHIFT);
        
        Rgb {
            r: r as u8,
            g: g as u8,
            b: b as u8,
        }
    }
}

/// Convert a grayscale buffer to RGB using a palette
/// 
/// # Arguments
/// * `greyscale` - Input grayscale buffer (fixed-point values 0..1)
/// * `rgb_buffer` - Output RGB buffer (3 bytes per pixel: R, G, B)
/// * `palette` - 16-entry palette for color mapping
pub fn rgb_buffer_from_greyscale(
    greyscale: &[Fixed],
    rgb_buffer: &mut [u8],
    palette: &Palette,
) {
    let pixel_count = greyscale.len();
    assert!(rgb_buffer.len() >= pixel_count * 3, "RGB buffer too small");
    
    for (i, &grey_value) in greyscale.iter().enumerate() {
        let color = palette.get_color(grey_value);
        let idx = i * 3;
        rgb_buffer[idx] = color.r;
        rgb_buffer[idx + 1] = color.g;
        rgb_buffer[idx + 2] = color.b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lpscript::math::ToFixed;

    #[test]
    fn test_palette_edges() {
        let palette = Palette::rainbow();
        
        // At 0.0, should get first color
        let c0 = palette.get_color(Fixed::ZERO);
        assert_eq!(c0.r, 255);
        assert_eq!(c0.g, 0);
        assert_eq!(c0.b, 0);
        
        // At 1.0, should get last color (or close to it)
        let c1 = palette.get_color(Fixed::ONE);
        // Should be close to the last color
        assert!(c1.r > 200);
    }

    #[test]
    fn test_rgb_conversion() {
        let palette = Palette::rainbow();
        let greyscale = vec![
            Fixed::ZERO,
            0.5f32.to_fixed(),
            Fixed::ONE,
        ];
        let mut rgb_buffer = vec![0u8; 9];
        
        rgb_buffer_from_greyscale(&greyscale, &mut rgb_buffer, &palette);
        
        // First pixel should be red (0.0 -> first palette entry)
        assert_eq!(rgb_buffer[0], 255);
        assert_eq!(rgb_buffer[1], 0);
        assert_eq!(rgb_buffer[2], 0);
        
        // Middle and last pixels should be non-zero and different from first
        assert!(rgb_buffer[3] != 255 || rgb_buffer[4] != 0 || rgb_buffer[5] != 0);
    }
}

