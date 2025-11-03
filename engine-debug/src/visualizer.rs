/// Simple visualizer for the test engine
/// Shows the grayscale buffer, RGB buffer, and LED output in real-time
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
    text::Text,
};
use engine_core::test_engine::fixed_to_f32;
use engine_core::test_scene::{render_test_scene, SceneData, LED_COUNT, WIDTH, HEIGHT};
use engine_core::test_engine::{fixed_from_f32, LedMapping};
use minifb::{Key, Window, WindowOptions};

const SCALE: usize = 20; // Pixels per cell
const STATS_BAR_HEIGHT: usize = 30; // Black bar at bottom for stats

// Window layout: [Grayscale 16x16] [RGB 16x16] [LEDs 128x1] [Stats Bar]
const WINDOW_WIDTH: usize = (WIDTH * SCALE) + (WIDTH * SCALE) + (LED_COUNT * SCALE / 8);
const WINDOW_HEIGHT: usize = (HEIGHT * SCALE) + STATS_BAR_HEIGHT;

fn main() {
    let mut window = Window::new(
        "Test Engine Visualizer - [Grayscale] [RGB] [LEDs]",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);
    
    let mut scene = SceneData::new();
    let mut frame_count = 0u32;
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
    
    // Performance tracking
    let mut total_engine_us = 0u64;
    let mut total_ui_us = 0u64;
    let mut frames_for_avg = 0u32;
    let mut engine_us_avg = 0.0;
    let mut ui_us_avg = 0.0;
    let mut scene_time = 0.0f32;
    
    // ESP32 benchmark data: (pixels, esp32_us)
    const ESP32_BENCHMARKS: [(usize, f32); 5] = [
        (64, 2211.0),      // 8x8
        (144, 4616.0),     // 12x12
        (256, 7968.0),     // 16x16
        (400, 12287.0),    // 20x20
        (576, 17568.0),    // 24x24
    ];
    
    // Compute linear regression: esp32_us = slope * pixels + intercept
    fn compute_esp32_model() -> (f32, f32) {
        let n = ESP32_BENCHMARKS.len() as f32;
        let sum_x: f32 = ESP32_BENCHMARKS.iter().map(|(p, _)| *p as f32).sum();
        let sum_y: f32 = ESP32_BENCHMARKS.iter().map(|(_, us)| *us).sum();
        let sum_xy: f32 = ESP32_BENCHMARKS.iter().map(|(p, us)| *p as f32 * us).sum();
        let sum_x2: f32 = ESP32_BENCHMARKS.iter().map(|(p, _)| (*p as f32) * (*p as f32)).sum();
        
        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;
        (slope, intercept)
    }
    
    let (esp32_us_per_pixel, esp32_base_us) = compute_esp32_model();

    let mut last_frame_time = std::time::Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = std::time::Instant::now();
        let delta = frame_start.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = frame_start;
        
        scene_time += delta * 0.5;
        let time = fixed_from_f32(scene_time);
        
        // Time just the engine render
        let engine_start = std::time::Instant::now();
        render_test_scene(&mut scene, time);
        let engine_us = engine_start.elapsed().as_micros() as u64;
        
        // Track performance
        total_engine_us += engine_us;
        frames_for_avg += 1;
        if frames_for_avg >= 60 {
            engine_us_avg = total_engine_us as f32 / frames_for_avg as f32;
            ui_us_avg = total_ui_us as f32 / frames_for_avg as f32;
            total_engine_us = 0;
            total_ui_us = 0;
            frames_for_avg = 0;
        }

        buffer.fill(0xFF000000);

        draw_greyscale(&scene.greyscale_buffer, &mut buffer, 0, 0, SCALE);
        draw_rgb_2d(&scene.rgb_2d_buffer, &mut buffer, WIDTH * SCALE, 0, SCALE);
        draw_leds(&scene.led_output, &mut buffer, (WIDTH * SCALE) + (WIDTH * SCALE), 0, SCALE);
        draw_led_debug_overlay(&mut buffer, &scene.mapping, WIDTH * SCALE, (WIDTH * SCALE) + (WIDTH * SCALE), 0, SCALE);
        
        // Predict ESP32 performance for current canvas size
        let pixels = WIDTH * HEIGHT;
        let esp32_predicted_us = esp32_us_per_pixel * pixels as f32 + esp32_base_us;
        draw_stats_bar(&mut buffer, engine_us_avg, ui_us_avg, esp32_predicted_us);

        window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

        frame_count += 1;
        let full_frame_us = frame_start.elapsed().as_micros() as u64;
        total_ui_us += full_frame_us;
    }
}

fn draw_greyscale(
    greyscale: &[i32],
    buffer: &mut [u32],
    offset_x: usize,
    offset_y: usize,
    scale: usize,
) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let grey_val = greyscale[y * WIDTH + x];
            let grey_f = fixed_to_f32(grey_val).clamp(0.0, 1.0);
            let grey_u8 = (grey_f * 255.0) as u8;
            let color = rgb_to_u32(grey_u8, grey_u8, grey_u8);

            // Fill scaled pixel
            for dy in 0..scale {
                for dx in 0..scale {
                    let px = offset_x + x * scale + dx;
                    let py = offset_y + y * scale + dy;
                    if px < WINDOW_WIDTH && py < WINDOW_HEIGHT {
                        buffer[py * WINDOW_WIDTH + px] = color;
                    }
                }
            }
        }
    }
}

fn draw_rgb_2d(rgb: &[u8], buffer: &mut [u32], offset_x: usize, offset_y: usize, scale: usize) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = (y * WIDTH + x) * 3;
            let r = rgb[idx];
            let g = rgb[idx + 1];
            let b = rgb[idx + 2];
            let color = rgb_to_u32(r, g, b);

            // Fill scaled pixel
            for dy in 0..scale {
                for dx in 0..scale {
                    let px = offset_x + x * scale + dx;
                    let py = offset_y + y * scale + dy;
                    if px < WINDOW_WIDTH && py < WINDOW_HEIGHT {
                        buffer[py * WINDOW_WIDTH + px] = color;
                    }
                }
            }
        }
    }
}

fn draw_leds(leds: &[u8], buffer: &mut [u32], offset_x: usize, offset_y: usize, scale: usize) {
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoTextStyle},
        pixelcolor::Rgb888,
        prelude::*,
        primitives::{Circle, PrimitiveStyle},
        text::Text,
    };
    
    let mut fb = Framebuffer::new(buffer, WINDOW_WIDTH, WINDOW_HEIGHT);
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(255, 255, 255));
    
    // Draw as filled circles using embedded-graphics
    for led_idx in 0..LED_COUNT.min(leds.len() / 3) {
        let idx = led_idx * 3;
        let r = leds[idx];
        let g = leds[idx + 1];
        let b = leds[idx + 2];

        // Position in 8x16 grid
        let x = led_idx % 16;
        let y = led_idx / 16;
        let center_x = (offset_x + x * scale + scale / 2) as i32;
        let center_y = (offset_y + y * scale + scale / 2) as i32;
        let diameter = scale as u32;

        // Draw filled circle
        Circle::new(Point::new(center_x - (diameter / 2) as i32, center_y - (diameter / 2) as i32), diameter)
            .into_styled(PrimitiveStyle::with_fill(Rgb888::new(r, g, b)))
            .draw(&mut fb)
            .ok();
        
        // Draw LED number
        let label = format!("{}", led_idx);
        let text_x = center_x - (label.len() as i32 * 3);
        let text_y = center_y + 3;
        Text::new(&label, Point::new(text_x, text_y), text_style)
            .draw(&mut fb)
            .ok();
    }
}

// Framebuffer adapter for embedded-graphics
struct Framebuffer<'a> {
    buffer: &'a mut [u32],
    width: usize,
    height: usize,
}

impl<'a> Framebuffer<'a> {
    fn new(buffer: &'a mut [u32], width: usize, height: usize) -> Self {
        Framebuffer {
            buffer,
            width,
            height,
        }
    }
}

impl DrawTarget for Framebuffer<'_> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            let x = coord.x as usize;
            let y = coord.y as usize;
            if x < self.width && y < self.height {
                self.buffer[y * self.width + x] = rgb_to_u32(color.r(), color.g(), color.b());
            }
        }
        Ok(())
    }
}

impl OriginDimensions for Framebuffer<'_> {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

fn draw_led_debug_overlay(
    buffer: &mut [u32],
    mapping: &LedMapping,
    rgb_offset_x: usize,
    led_offset_x: usize,
    offset_y: usize,
    scale: usize,
) {
    use engine_core::math::fixed_to_f32;
    let mut fb = Framebuffer::new(buffer, WINDOW_WIDTH, WINDOW_HEIGHT);

    let circle_style_source = PrimitiveStyle::with_stroke(Rgb888::new(0, 255, 0), 2); // Green on source
    let text_style_small = MonoTextStyle::new(&FONT_6X10, Rgb888::new(0, 0, 0)); // Black text on green circles

    for led_idx in 0..LED_COUNT {
        if let Some(map) = mapping.get(led_idx) {
            // Source position on RGB buffer (middle panel) - now with sub-pixel precision
            let x_pixels = fixed_to_f32(map.pos.x.0);
            let y_pixels = fixed_to_f32(map.pos.y.0);
            let source_x = (rgb_offset_x as f32 + x_pixels * scale as f32) as i32;
            let source_y = (offset_y as f32 + y_pixels * scale as f32) as i32;

            // Destination position on LED strip (right panel)
            let dest_x = led_idx % 16;
            let dest_y = led_idx / 16;
            let dest_center_x = (led_offset_x + dest_x * scale + scale / 2) as i32;
            let dest_center_y = (offset_y + dest_y * scale + scale / 2) as i32;

            // Draw circle on source showing sampling area (diameter = scale, radius = scale/2)
            Circle::with_center(Point::new(source_x, source_y), scale as u32)
                .into_styled(circle_style_source)
                .draw(&mut fb)
                .ok();

            // Draw LED number on source
            let text = format!("{}", led_idx);
            let text_width = if led_idx < 10 {
                6
            } else if led_idx < 100 {
                12
            } else {
                18
            };
            Text::new(
                &text,
                Point::new(source_x - text_width / 2, source_y + 3),
                text_style_small,
            )
            .draw(&mut fb)
            .ok();
        }
    }
}

fn draw_stats_bar(buffer: &mut [u32], engine_us: f32, ui_us: f32, esp32_predicted_us: f32) {
    // Fill black bar at bottom
    let bar_y_start = HEIGHT * SCALE;
    for y in bar_y_start..(HEIGHT * SCALE + STATS_BAR_HEIGHT) {
        for x in 0..WINDOW_WIDTH {
            if y < WINDOW_HEIGHT && x < WINDOW_WIDTH {
                buffer[y * WINDOW_WIDTH + x] = 0xFF000000;
            }
        }
    }
    
    let mut fb = Framebuffer::new(buffer, WINDOW_WIDTH, WINDOW_HEIGHT);
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(200, 200, 200));
    let text_style_bright = MonoTextStyle::new(&FONT_6X10, Rgb888::new(255, 255, 255));
    
    // Line 1: Canvas info
    let info_text = format!("Canvas: {}x{}  Output: {} LEDs", WIDTH, HEIGHT, LED_COUNT);
    Text::new(&info_text, Point::new(10, (HEIGHT * SCALE + 10) as i32), text_style)
        .draw(&mut fb)
        .ok();
    
    // Line 2: Performance stats
    let engine_fps = if engine_us > 0.0 { 1_000_000.0 / engine_us } else { 0.0 };
    let esp32_fps = if esp32_predicted_us > 0.0 { 1_000_000.0 / esp32_predicted_us } else { 0.0 };
    let ui_fps = if ui_us > 0.0 { 1_000_000.0 / ui_us } else { 0.0 };
    
    let perf_text = format!(
        "Engine: {:.0}us ({:.0} FPS)  ESP32 predicted: {:.0} FPS  UI: {:.0} FPS",
        engine_us, engine_fps, esp32_fps, ui_fps
    );
    
    Text::new(&perf_text, Point::new(10, (HEIGHT * SCALE + 22) as i32), text_style_bright)
        .draw(&mut fb)
        .ok();
}

#[inline(always)]
fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    // minifb expects 0RGB format
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
