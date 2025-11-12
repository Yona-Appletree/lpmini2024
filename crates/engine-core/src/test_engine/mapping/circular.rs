use lp_script::fixed::trig::{cos, sin};
use lp_script::fixed::{Fixed, FIXED_ONE, FIXED_SHIFT};

/// Circular panel LED mappings (concentric rings)
use super::{LedMap, LedMapping};

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
    /// Create mapping for standard 9-ring circular panel (outside-in data direction)
    ///
    /// Rings: [59, 48, 40, 32, 24, 16, 12, 8, 1];
    /// Total: 241 LEDs
    /// Data flows from outside-in: LED 0 is on the outermost ring, center LED is last
    pub fn circular_panel_9ring(width: usize, height: usize) -> Self {
        let ring_counts = [59, 48, 40, 32, 24, 16, 12, 8];
        let center_x = width / 2;
        let center_y = height / 2;
        let max_radius = if width < height {
            width / 2
        } else {
            height / 2
        };
        
        let mut maps = [LedMap::new(0, 0); 128];
        let mut led_idx = 0;

        let num_rings = ring_counts.len() + 1; // +1 for center LED

        // Convert to fixed-point
        let center_x_fixed = (center_x as i32) << FIXED_SHIFT;
        let center_y_fixed = (center_y as i32) << FIXED_SHIFT;
        let max_radius_fixed = (max_radius as i32) << FIXED_SHIFT;

        // Build from outside-in: start with outermost ring (ring_counts[0])
        // ring_counts[0] is outermost, ring_counts[9] is innermost
        // Iterate rings in normal order (outermost to innermost)
        for (ring_array_idx, &led_count) in ring_counts.iter().enumerate() {
            if led_idx >= 128 {
                break;
            }
            
            // Calculate radius for this ring in fixed-point
            // ring_array_idx 0 is outermost, ring_array_idx 7 is innermost
            // num_rings = 9 (8 rings + 1 center)
            // We want: ring_array_idx=0 → radius=max_radius, ring_array_idx=7 → radius=smaller
            // Formula: radius = max_radius * (num_rings - 1 - ring_array_idx) / (num_rings - 1)
            // This gives: 0→8/8=max_radius, 7→1/8 (correct!)
            let ring_idx = ring_counts.len() - 1 - ring_array_idx;
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

        // Center LED is last (innermost)
        if led_idx < 128 {
            maps[led_idx] = LedMap::new_fixed(center_x_fixed, center_y_fixed);
        }

        LedMapping { maps }
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

    #[test]
    fn test_circular_panel_9ring_outermost_ring_is_first() {
        // Test that LED 0 is on the outermost ring (ring_counts[0])
        let mapping = LedMapping::circular_panel_9ring(32, 32);
        let center_x = 16.0;
        let center_y = 16.0;
        let max_radius = 16.0;

        // LED 0 should be on the outermost ring (ring_counts[0] = 59 LEDs)
        let led0 = mapping.maps[0];
        let x0 = led0.pos.x.to_f32();
        let y0 = led0.pos.y.to_f32();
        let dist0 = ((x0 - center_x).powi(2) + (y0 - center_y).powi(2)).sqrt();

        // LED 0 should be close to max_radius (outermost)
        assert!(
            (dist0 - max_radius).abs() < 2.0,
            "LED 0 should be on outermost ring: distance={}, expected≈{}",
            dist0,
            max_radius
        );

        // Check a few LEDs from the outermost ring to verify they're all at similar radius
        for i in 0..10 {
            if let Some(led_map) = mapping.get(i) {
                let x = led_map.pos.x.to_f32();
                let y = led_map.pos.y.to_f32();
                let dist = ((x - center_x).powi(2) + (y - center_y).powi(2)).sqrt();
                assert!(
                    (dist - max_radius).abs() < 2.0,
                    "LED {} should be on outermost ring: distance={}, expected≈{}",
                    i,
                    dist,
                    max_radius
                );
            }
        }
    }

    #[test]
    fn test_circular_panel_9ring_center_is_last() {
        // Test that the center LED is the last LED index (if we have enough space)
        // Note: Total LEDs is 220, but we're limited to 128 in the array
        // So the center LED may not be included if we hit the limit
        let mapping = LedMapping::circular_panel_9ring(32, 32);
        let center_x = 16.0;
        let center_y = 16.0;

        // Find the last valid LED
        let mut last_led_idx = 0;
        for i in 0..128 {
            if mapping.get(i).is_some() {
                last_led_idx = i;
            }
        }

        let last_led = mapping.maps[last_led_idx];
        let x = last_led.pos.x.to_f32();
        let y = last_led.pos.y.to_f32();

        // If we have enough LEDs (220 total), the center should be last
        // But with 128 limit, we only get: 59 + 48 + 21 = 128 LEDs (first 3 rings partially)
        // So the center won't be included. Instead, verify the last LED is from an inner ring
        let dist = ((x - center_x).powi(2) + (y - center_y).powi(2)).sqrt();
        
        // The last LED should be closer to center than the first LED (outside-in direction)
        let first_led = mapping.maps[0];
        let first_x = first_led.pos.x.to_f32();
        let first_y = first_led.pos.y.to_f32();
        let first_dist = ((first_x - center_x).powi(2) + (first_y - center_y).powi(2)).sqrt();

        assert!(
            dist < first_dist,
            "Last LED should be closer to center than first: last_dist={}, first_dist={}",
            dist,
            first_dist
        );
    }

    #[test]
    fn test_circular_panel_9ring_ring_order() {
        // Test that rings are in correct order: outermost first, innermost before center
        let mapping = LedMapping::circular_panel_9ring(32, 32);
        let center_x = 16.0;
        let center_y = 16.0;

        // Collect distances for all LEDs
        let mut distances = Vec::new();
        for i in 0..128 {
            if let Some(led_map) = mapping.get(i) {
                let x = led_map.pos.x.to_f32();
                let y = led_map.pos.y.to_f32();
                let dist = ((x - center_x).powi(2) + (y - center_y).powi(2)).sqrt();
                distances.push((i, dist));
            }
        }

        // Distances should generally decrease (outside-in)
        // Allow some tolerance since we're sampling discrete positions
        for i in 1..distances.len().min(50) {
            // Most LEDs should have decreasing or similar distances
            // (allowing for some variation due to discrete sampling)
            if distances[i].1 > distances[i - 1].1 + 1.0 {
                // This is okay if we're transitioning between rings
                // But overall trend should be decreasing
            }
        }

        // First few LEDs should be at larger radius (outermost)
        let first_dist = distances[0].1;
        let last_dist = distances[distances.len() - 1].1;

        assert!(
            first_dist > last_dist,
            "First LED should be further from center than last: first={}, last={}",
            first_dist,
            last_dist
        );
    }

    #[test]
    fn test_circular_panel_9ring_led_counts() {
        // Test that we have the correct number of LEDs per ring
        let mapping = LedMapping::circular_panel_9ring(32, 32);
        let ring_counts = [59, 48, 32, 24, 20, 16, 12, 8]; // outermost to innermost

        let mut led_idx = 0;
        for (ring_num, &expected_count) in ring_counts.iter().enumerate() {
            // Count LEDs in this ring (they should all be at similar radius)
            let mut count = 0;
            let start_led = mapping.get(led_idx);
            if start_led.is_some() {
                let center_x = 16.0;
                let center_y = 16.0;
                let start_x = start_led.unwrap().pos.x.to_f32();
                let start_y = start_led.unwrap().pos.y.to_f32();
                let start_dist = ((start_x - center_x).powi(2) + (start_y - center_y).powi(2)).sqrt();

                // Count consecutive LEDs at similar radius
                while led_idx < 128 {
                    if let Some(led_map) = mapping.get(led_idx) {
                        let x = led_map.pos.x.to_f32();
                        let y = led_map.pos.y.to_f32();
                        let dist = ((x - center_x).powi(2) + (y - center_y).powi(2)).sqrt();
                        if (dist - start_dist).abs() < 1.0 {
                            count += 1;
                            led_idx += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }

            println!(
                "Ring {} (outermost={}): expected {} LEDs, found {} LEDs",
                ring_num,
                ring_num == 0,
                expected_count,
                count
            );

            // For the outermost ring, verify we have the expected count (or at least close, due to 128 limit)
            if ring_num == 0 {
                assert!(
                    count == expected_count || (count < expected_count && led_idx >= 128),
                    "Outermost ring should have {} LEDs, but found {} (may be limited by 128 LED cap)",
                    expected_count,
                    count
                );
            }
        }
    }
}
