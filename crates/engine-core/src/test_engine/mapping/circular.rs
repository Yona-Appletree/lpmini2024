/// Circular panel LED mappings (concentric rings)
use super::{LedMap, LedMapping};
use lp_script::fixed::trig::{cos, sin};
use lp_script::fixed::{Fixed, FIXED_ONE, FIXED_SHIFT};

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
    pub fn circular_panel(
        ring_counts: &[usize],
        center_x_px: usize,
        center_y_px: usize,
        max_radius_px: usize,
    ) -> Self {
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

                // Angle in radians (0..2π for full circle)
                // i / led_count gives position around circle (0..1)
                // Multiply by TAU (2π) to convert to radians
                let normalized_angle = (i as i32 * FIXED_ONE) / led_count as i32;
                let angle_radians =
                    Fixed(((normalized_angle as i64 * Fixed::TAU.0 as i64) >> FIXED_SHIFT) as i32);

                // Use fixed-point sin/cos (they expect radians, return -1..1 in Fixed)
                let cos_val = cos(angle_radians).0;
                let sin_val = sin(angle_radians).0;

                // sin/cos already return -1..1, use directly
                // x = center_x + radius * cos, y = center_y + radius * sin
                let x_offset = ((radius_fixed as i64 * cos_val as i64) >> FIXED_SHIFT) as i32;
                let y_offset = ((radius_fixed as i64 * sin_val as i64) >> FIXED_SHIFT) as i32;

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
        let max_radius = if width < height {
            width / 2
        } else {
            height / 2
        };
        Self::circular_panel(&ring_counts, center_x, center_y, max_radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_mapping_forms_complete_circle() {
        // Create a small circular panel
        let mapping = LedMapping::circular_panel_7ring(16, 16);

        // The outermost ring has 32 LEDs (indices 81-112)
        // They should form a complete circle
        let center_x = 8.0;
        let center_y = 8.0;

        let mut angles = Vec::new();
        for led_idx in 81..113 {
            let led_map = mapping.maps[led_idx];
            let x = led_map.pos.x.to_f32();
            let y = led_map.pos.y.to_f32();

            let dx = x - center_x;
            let dy = y - center_y;
            let angle = dy.atan2(dx);
            angles.push(angle);
        }

        // Sort angles
        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Check that we have angles spanning close to full circle
        let min_angle = angles[0];
        let max_angle = angles[angles.len() - 1];
        let span = max_angle - min_angle;

        println!(
            "Angle span: {} radians ({} degrees)",
            span,
            span.to_degrees()
        );
        println!(
            "Min angle: {} radians ({} degrees)",
            min_angle,
            min_angle.to_degrees()
        );
        println!(
            "Max angle: {} radians ({} degrees)",
            max_angle,
            max_angle.to_degrees()
        );

        // Should span close to 2π (full circle)
        // Allow some tolerance for first/last LED gap
        let expected_span = std::f32::consts::TAU;
        assert!(
            (span - expected_span).abs() < 0.5,
            "Outermost ring should span full circle (2π ≈ 6.28 radians), but spans {} radians",
            span
        );
    }

    #[test]
    fn test_circular_mapping_angles_use_radians() {
        // Test that the trig functions are being called with radians, not 0..1 range
        // by checking that cos(0) gives position at +X axis (right side)

        let ring_counts = [8]; // Single ring with 8 LEDs
        let center_x = 8;
        let center_y = 8;
        let radius = 4;

        let mapping = LedMapping::circular_panel(&ring_counts, center_x, center_y, radius);

        // LED 1 (first LED after center) should be at angle 0, which is +X axis (right side)
        let led1 = mapping.maps[1];
        let x = led1.pos.x.to_f32();
        let y = led1.pos.y.to_f32();

        println!("LED 1 position: ({}, {})", x, y);
        println!(
            "Expected: x > center_x ({}), y ≈ center_y ({})",
            center_x, center_y
        );

        // At angle 0:
        // - cos(0) = 1, so x should be center_x + radius
        // - sin(0) = 0, so y should be center_y
        assert!(
            x > center_x as f32,
            "LED at angle 0 should be to the RIGHT of center (x={} should be > {})",
            x,
            center_x
        );

        assert!(
            (y - center_y as f32).abs() < 0.5,
            "LED at angle 0 should be at center Y (y={} should be ≈ {})",
            y,
            center_y
        );
    }
}
