/// Circular panel LED mappings (concentric rings)
use super::{LedMap, LedMapping};
use crate::math::FIXED_ONE;
use core::cmp::max;

impl LedMapping {
    /// Create a circular panel mapping with concentric rings
    /// 
    /// The panel consists of a center LED plus concentric rings.
    /// LEDs in each ring are wired clockwise starting at angle 0 (right side).
    /// Rings are wired from innermost to outermost.
    /// 
    /// # Arguments
    /// * `ring_counts` - LED count for each ring (excluding center which is always 1)
    /// * `center_x` - X coordinate of center in pixels
    /// * `center_y` - Y coordinate of center in pixels  
    /// * `max_radius` - Radius of outermost ring in pixels
    pub fn circular_panel(ring_counts: &[usize], center_x: f32, center_y: f32, max_radius: f32) -> Self {
        let mut maps = [LedMap::new(0, 0); 128];
        let mut led_idx = 0;
        
        let num_rings = ring_counts.len() + 1; // +1 for center LED
        
        // Ring 0: Center LED (1 LED at center)
        let center_fixed_x = max(0, (center_x * FIXED_ONE as f32) as i32);
        let center_fixed_y = max(0, (center_y * FIXED_ONE as f32) as i32);
        maps[led_idx] = LedMap::new_fixed(center_fixed_x, center_fixed_y);
        led_idx += 1;
        
        // Outer rings
        for (ring_idx, &led_count) in ring_counts.iter().enumerate() {
            // Calculate radius for this ring (evenly spaced from 0 to max_radius)
            let radius = max_radius * (ring_idx + 1) as f32 / (num_rings - 1) as f32;
            
            for i in 0..led_count {
                if led_idx >= 128 {
                    break;
                }
                
                // Angle for this LED (clockwise from 0)
                // First LED at angle 0 (right side), then clockwise
                let angle = (i as f32 / led_count as f32) * 2.0 * 3.14159265;
                
                #[cfg(not(feature = "use-libm"))]
                {
                    let x = center_x + radius * angle.cos();
                    let y = center_y + radius * angle.sin();
                    let x_fixed = (x * FIXED_ONE as f32) as i32;
                    let y_fixed = (y * FIXED_ONE as f32) as i32;
                    maps[led_idx] = LedMap::new_fixed(x_fixed, y_fixed);
                }
                
                #[cfg(feature = "use-libm")]
                {
                    let x = center_x + radius * libm::cosf(angle);
                    let y = center_y + radius * libm::sinf(angle);
                    let x_fixed = (x * FIXED_ONE as f32) as i32;
                    let y_fixed = (y * FIXED_ONE as f32) as i32;
                    maps[led_idx] = LedMap::new_fixed(x_fixed, y_fixed);
                }
                
                led_idx += 1;
            }
        }
        
        LedMapping { maps }
    }

    /// Create mapping for standard 7-ring circular panel
    /// 
    /// Rings: 1 (center), 8, 12, 16, 20, 24, 32 LEDs
    /// Total: 113 LEDs
    pub fn circular_panel_7ring(width: usize, height: usize) -> Self {
        let ring_counts = [8, 12, 16, 20, 24, 32];
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let max_radius = center_x.min(center_y) - 1.0;
        Self::circular_panel(&ring_counts, center_x, center_y, max_radius)
    }
}

