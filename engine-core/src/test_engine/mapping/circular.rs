/// Circular panel LED mappings (concentric rings)
use super::{LedMap, LedMapping};
use crate::math::{Fixed, FIXED_ONE, FIXED_SHIFT, fixed_from_int};
use crate::math::trig::{sin, cos};
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
    /// * `center_x_px` - X coordinate of center in pixels (integer)
    /// * `center_y_px` - Y coordinate of center in pixels (integer)
    /// * `max_radius_px` - Radius of outermost ring in pixels (integer)
    pub fn circular_panel(ring_counts: &[usize], center_x_px: usize, center_y_px: usize, max_radius_px: usize) -> Self {
        let mut maps = [LedMap::new(0, 0); 128];
        let mut led_idx = 0;
        
        let num_rings = ring_counts.len() + 1; // +1 for center LED
        
        // Convert to fixed-point
        let center_x_fixed = (center_x_px as i32) << FIXED_SHIFT;
        let center_y_fixed = (center_y_px as i32) << FIXED_SHIFT;
        let max_radius_fixed = (max_radius_px as i32) << FIXED_SHIFT;
        
        // Ring 0: Center LED (1 LED at center)
        maps[led_idx] = LedMap::new_fixed(center_x_fixed, center_y_fixed);
        led_idx += 1;
        
        // Outer rings
        for (ring_idx, &led_count) in ring_counts.iter().enumerate() {
            // Calculate radius for this ring in fixed-point
            let radius_fixed = (max_radius_fixed * (ring_idx + 1) as i32) / (num_rings - 1) as i32;
            
            for i in 0..led_count {
                if led_idx >= 128 {
                    break;
                }
                
                // Angle in 0..1 range (normalized for our sin/cos)
                // i / led_count gives position around circle
                let angle_fixed = (i as i32 * FIXED_ONE) / led_count as i32;
                
                // Use fixed-point sin/cos (they return 0..1, we need -1..1)
                let cos_val = cos(angle_fixed);
                let sin_val = sin(angle_fixed);
                
                // Map from 0..1 to -1..1: (val * 2) - 1
                let cos_centered = (cos_val << 1) - FIXED_ONE;
                let sin_centered = (sin_val << 1) - FIXED_ONE;
                
                // x = center_x + radius * cos, y = center_y + radius * sin
                let x_offset = ((radius_fixed as i64 * cos_centered as i64) >> FIXED_SHIFT) as i32;
                let y_offset = ((radius_fixed as i64 * sin_centered as i64) >> FIXED_SHIFT) as i32;
                
                let x_fixed = center_x_fixed + x_offset;
                let y_fixed = center_y_fixed + y_offset;
                
                maps[led_idx] = LedMap::new_fixed(x_fixed, y_fixed);
                
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
        let center_x = width / 2;
        let center_y = height / 2;
        let max_radius = if width < height { width / 2 } else { height / 2 };
        Self::circular_panel(&ring_counts, center_x, center_y, max_radius)
    }
}

