/// Comprehensive tests for LED mappings
#[cfg(test)]
mod mapping_tests {
    use crate::test_engine::{apply_2d_mapping, MappingConfig};

    #[test]
    fn test_circular_panel_led_count() {
        let config = MappingConfig::CircularPanel7Ring {
            width: 16,
            height: 16,
        };
        let led_count = config.led_count();
        println!("Circular panel 7ring has {} LEDs", led_count);
        assert_eq!(
            led_count, 113,
            "CircularPanel7Ring should have 113 LEDs (1+8+12+16+20+24+32)"
        );
    }

    #[test]
    fn test_mapping_gradient() {
        let config = MappingConfig::CircularPanel7Ring {
            width: 8,
            height: 8,
        };
        let led_count = config.led_count();
        let mapping = config.build();

        // Create gradient input (left=black, right=white)
        let mut input_rgb = vec![0u8; 8 * 8 * 3];
        for y in 0..8 {
            for x in 0..8 {
                let brightness = (x * 255 / 7) as u8;
                let idx = (y * 8 + x) * 3;
                input_rgb[idx] = brightness;
                input_rgb[idx + 1] = brightness;
                input_rgb[idx + 2] = brightness;
            }
        }

        let mut output = vec![0u8; led_count * 3];
        apply_2d_mapping(&input_rgb, &mut output, &mapping, 8, 8);

        // Output should have variation
        let first_brightness = output[0];
        let has_variation = output.chunks(3).any(|rgb| rgb[0] != first_brightness);
        assert!(
            has_variation,
            "Mapping should produce varied output for gradient input"
        );
    }

    #[test]
    fn test_different_sizes() {
        // Test various sizes to ensure mapping scales
        for size in [4, 8, 16, 32] {
            let config = MappingConfig::CircularPanel7Ring {
                width: size,
                height: size,
            };
            let led_count = config.led_count();
            println!("{}x{} circular panel has {} LEDs", size, size, led_count);
            assert!(led_count > 0, "Size {}x{} should have LEDs", size, size);
        }
    }
}
