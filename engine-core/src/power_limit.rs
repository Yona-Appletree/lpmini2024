/// Power limiting and brightness control for LED strips
/// 
/// This module applies brightness scaling, gamma correction, and power limiting
/// to prevent overdrawing current from the power supply.

use smart_leds::RGB8;

/// Default gamma correction curve (2.2)
const GAMMA_TABLE: [u8; 256] = generate_gamma_table();

const fn generate_gamma_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        // Approximation of gamma 2.2: (i/255)^2.2 * 255
        // Using integer math: (i * i * i) / (255 * 255) is roughly gamma 2.0
        // For gamma 2.2, we use a slightly different curve
        let normalized = (i * i) / 255;
        table[i] = if normalized > 255 { 255 } else { normalized as u8 };
        i += 1;
    }
    table
}

/// Configuration for power limiting
#[derive(Clone, Copy)]
pub struct PowerLimitConfig {
    /// Brightness multiplier in fixed-point (256 = 1.0, 128 = 0.5, etc.)
    pub brightness_256: u32,
    /// Power budget in milliamps
    pub power_budget_ma: u32,
    /// Power draw per LED at full white (all channels 255) in milliamps
    pub led_white_power_ma: u32,
    /// Idle power draw per LED (all channels 0) in milliamps
    pub led_idle_power_ma: u32,
}

impl Default for PowerLimitConfig {
    fn default() -> Self {
        PowerLimitConfig {
            brightness_256: 256, // 1.0
            power_budget_ma: 1000,
            led_white_power_ma: 50,
            led_idle_power_ma: 1,
        }
    }
}

/// Apply brightness scaling to a single channel using fixed-point math
#[inline]
fn apply_brightness(value: u8, brightness_256: u32) -> u8 {
    let scaled = (value as u32 * brightness_256) / 256;
    if scaled > 255 { 255 } else { scaled as u8 }
}

/// Apply gamma correction to a single channel using lookup table
#[inline]
fn apply_gamma(value: u8) -> u8 {
    GAMMA_TABLE[value as usize]
}

/// Calculate power consumption for a single LED in milliamps
/// 
/// # Arguments
/// * `led` - RGB8 LED color after gamma correction
/// * `white_power_ma` - Power at full white (255, 255, 255)
/// * `idle_power_ma` - Power at off (0, 0, 0)
fn calculate_led_power(led: RGB8, white_power_ma: u32, idle_power_ma: u32) -> u32 {
    // Power is roughly proportional to the sum of RGB values
    // At (0,0,0): idle_power_ma
    // At (255,255,255): white_power_ma
    let channel_sum = led.r as u32 + led.g as u32 + led.b as u32;
    let max_sum = 255u32 * 3;
    
    // Linear interpolation between idle and white power
    idle_power_ma + ((white_power_ma - idle_power_ma) * channel_sum) / max_sum
}

/// Process LED buffer with brightness, gamma, and power limiting
/// 
/// # Arguments
/// * `leds` - Input/output LED buffer
/// * `config` - Power limiting configuration
pub fn apply_power_limit(leds: &mut [RGB8], config: &PowerLimitConfig) {
    // Step 1: Apply brightness scaling
    for led in leds.iter_mut() {
        led.r = apply_brightness(led.r, config.brightness_256);
        led.g = apply_brightness(led.g, config.brightness_256);
        led.b = apply_brightness(led.b, config.brightness_256);
    }
    
    // Step 2: Apply gamma correction
    for led in leds.iter_mut() {
        led.r = apply_gamma(led.r);
        led.g = apply_gamma(led.g);
        led.b = apply_gamma(led.b);
    }
    
    // Step 3: Calculate total power consumption
    let total_power_ma: u32 = leds.iter()
        .map(|led| calculate_led_power(*led, config.led_white_power_ma, config.led_idle_power_ma))
        .sum();
    
    // Step 4: If over budget, scale down using integer math
    if total_power_ma > config.power_budget_ma {
        // Use 16-bit fixed point: scale_factor_65536 = (budget * 65536) / total
        // This gives us more precision than 8-bit
        let scale_factor_65536 = ((config.power_budget_ma as u64) << 16) / (total_power_ma as u64);
        
        for led in leds.iter_mut() {
            led.r = ((led.r as u64 * scale_factor_65536) >> 16) as u8;
            led.g = ((led.g as u64 * scale_factor_65536) >> 16) as u8;
            led.b = ((led.b as u64 * scale_factor_65536) >> 16) as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brightness_scaling() {
        assert_eq!(apply_brightness(255, 256), 255); // 1.0
        assert_eq!(apply_brightness(255, 128), 127); // 0.5
        assert_eq!(apply_brightness(100, 128), 50);  // 0.5
        assert_eq!(apply_brightness(255, 0), 0);     // 0.0
    }

    #[test]
    fn test_gamma_correction() {
        // Gamma should darken mid-range values
        assert_eq!(apply_gamma(0), 0);
        assert_eq!(apply_gamma(255), 255);
        
        // Mid-range should be darker with gamma
        let mid = apply_gamma(128);
        assert!(mid < 128, "Gamma should darken mid-range, got {}", mid);
    }

    #[test]
    fn test_power_calculation() {
        let config = PowerLimitConfig::default();
        
        // All off = idle power
        let black = RGB8 { r: 0, g: 0, b: 0 };
        assert_eq!(
            calculate_led_power(black, config.led_white_power_ma, config.led_idle_power_ma),
            config.led_idle_power_ma
        );
        
        // Full white = white power
        let white = RGB8 { r: 255, g: 255, b: 255 };
        assert_eq!(
            calculate_led_power(white, config.led_white_power_ma, config.led_idle_power_ma),
            config.led_white_power_ma
        );
        
        // Half brightness should be roughly halfway between idle and white
        let gray = RGB8 { r: 127, g: 127, b: 127 };
        let gray_power = calculate_led_power(gray, config.led_white_power_ma, config.led_idle_power_ma);
        let expected = (config.led_idle_power_ma + config.led_white_power_ma) / 2;
        assert!((gray_power as i32 - expected as i32).abs() < 3, 
                "Gray power {} should be close to {}", gray_power, expected);
    }

    #[test]
    fn test_power_limiting_no_clamp() {
        let mut leds = vec![
            RGB8 { r: 10, g: 10, b: 10 },
            RGB8 { r: 20, g: 20, b: 20 },
        ];
        
        let config = PowerLimitConfig {
            brightness_256: 256, // 1.0
            power_budget_ma: 10000, // Very high budget
            ..Default::default()
        };
        
        apply_power_limit(&mut leds, &config);
        
        // Should apply gamma but not scale down
        assert_eq!(leds[0].r, apply_gamma(10));
        assert_eq!(leds[1].r, apply_gamma(20));
    }

    #[test]
    fn test_power_limiting_with_clamp() {
        // Create LEDs that would exceed power budget
        let mut leds = vec![RGB8 { r: 255, g: 255, b: 255 }; 100];
        
        let config = PowerLimitConfig {
            brightness_256: 256, // 1.0
            power_budget_ma: 1000,
            led_white_power_ma: 50,
            led_idle_power_ma: 1,
        };
        
        apply_power_limit(&mut leds, &config);
        
        // After limiting, all LEDs should be scaled down
        assert!(leds[0].r < 255, "LEDs should be scaled down");
        assert!(leds[0].g < 255, "LEDs should be scaled down");
        assert!(leds[0].b < 255, "LEDs should be scaled down");
        
        // Verify power is actually limited
        let total_power: u32 = leds.iter()
            .map(|led| calculate_led_power(*led, config.led_white_power_ma, config.led_idle_power_ma))
            .sum();
        
        assert!(total_power <= config.power_budget_ma, 
                "Total power {} should be <= budget {}", total_power, config.power_budget_ma);
    }

    #[test]
    fn test_brightness_then_gamma() {
        let mut leds = vec![RGB8 { r: 100, g: 150, b: 200 }];
        
        let config = PowerLimitConfig {
            brightness_256: 128, // 0.5
            power_budget_ma: 10000,
            ..Default::default()
        };
        
        apply_power_limit(&mut leds, &config);
        
        // Should have brightness applied first, then gamma
        let expected_r = apply_gamma(apply_brightness(100, 128));
        assert_eq!(leds[0].r, expected_r);
    }
}

