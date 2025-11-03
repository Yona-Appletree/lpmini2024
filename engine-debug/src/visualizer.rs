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

// Window layout: [Grayscale 16x16] [RGB 16x16] [LEDs 128x1]
const WINDOW_WIDTH: usize = (WIDTH * SCALE) + (WIDTH * SCALE) + (LED_COUNT * SCALE / 8);
const WINDOW_HEIGHT: usize = HEIGHT * SCALE;

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Slower time progression to avoid integer boundary issues
        let time = fixed_from_f32(frame_count as f32 * 0.003);
        render_test_scene(&mut scene, time);

        buffer.fill(0xFF000000);

        draw_greyscale(&scene.greyscale_buffer, &mut buffer, 0, 0, SCALE);
        draw_rgb_2d(&scene.rgb_2d_buffer, &mut buffer, WIDTH * SCALE, 0, SCALE);
        draw_leds(&scene.led_output, &mut buffer, (WIDTH * SCALE) + (WIDTH * SCALE), 0, SCALE);
        draw_led_debug_overlay(&mut buffer, &scene.mapping, WIDTH * SCALE, (WIDTH * SCALE) + (WIDTH * SCALE), 0, SCALE);
        
        // Draw time value for debugging
        draw_time_debug(&mut buffer, time, frame_count);

        // Update window
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        frame_count += 1;
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
    // Draw as 8 rows of 16 LEDs
    for led_idx in 0..LED_COUNT.min(leds.len() / 3) {
        let idx = led_idx * 3;
        let r = leds[idx];
        let g = leds[idx + 1];
        let b = leds[idx + 2];
        let color = rgb_to_u32(r, g, b);

        // Position in 8x16 grid
        let x = led_idx % 16;
        let y = led_idx / 16;

        // Fill scaled LED
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
    let mut fb = Framebuffer::new(buffer, WINDOW_WIDTH, WINDOW_HEIGHT);

    let circle_style_source = PrimitiveStyle::with_stroke(Rgb888::new(0, 255, 0), 2); // Green on source
    let circle_style_dest = PrimitiveStyle::with_stroke(Rgb888::new(255, 255, 0), 2); // Yellow on dest
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(255, 255, 255));
    let text_style_small = MonoTextStyle::new(&FONT_6X10, Rgb888::new(0, 0, 0)); // Black text on green circles

    for led_idx in 0..LED_COUNT {
        if let Some(map) = mapping.get(led_idx) {
            // Source position on RGB buffer (middle panel)
            let source_x = (rgb_offset_x + map.x * scale + scale / 2) as i32;
            let source_y = (offset_y + map.y * scale + scale / 2) as i32;

            // Destination position on LED strip (right panel)
            let dest_x = led_idx % 16;
            let dest_y = led_idx / 16;
            let dest_center_x = (led_offset_x + dest_x * scale + scale / 2) as i32;
            let dest_center_y = (offset_y + dest_y * scale + scale / 2) as i32;

            // Draw circle on source (RGB buffer) with LED number
            Circle::with_center(Point::new(source_x, source_y), (scale / 3) as u32)
                .into_styled(circle_style_source)
                .draw(&mut fb)
                .ok();

            // Draw LED number on source (green circle on RGB buffer)
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

            // Draw circle on destination (LED strip)
            Circle::with_center(Point::new(dest_center_x, dest_center_y), (scale / 2) as u32)
                .into_styled(circle_style_dest)
                .draw(&mut fb)
                .ok();

            // Draw LED number on destination (yellow circle on LED strip)
            Text::new(
                &text,
                Point::new(dest_center_x - text_width / 2, dest_center_y + 3),
                text_style,
            )
            .draw(&mut fb)
            .ok();
        }
    }
}

fn draw_time_debug(buffer: &mut [u32], time: i32, frame_count: u32) {
    let mut fb = Framebuffer::new(buffer, WINDOW_WIDTH, WINDOW_HEIGHT);
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(255, 255, 255));
    
    let time_f = fixed_to_f32(time);
    let text = format!("Frame: {}  Time: {:.3}", frame_count, time_f);
    Text::new(&text, Point::new(10, WINDOW_HEIGHT as i32 - 15), text_style)
        .draw(&mut fb)
        .ok();
}

#[inline(always)]
fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    // minifb expects 0RGB format
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
