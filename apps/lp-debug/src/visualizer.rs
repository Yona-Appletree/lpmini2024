use std::collections::VecDeque;

/// Simple visualizer for the test engine
/// Shows the grayscale buffer, RGB buffer, and LED output in real-time
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
    text::Text,
};
use engine_core::test_engine::demo_program::create_demo_scene;
use engine_core::test_engine::scene::SceneRuntime;
use engine_core::test_engine::test_scene::{render_test_scene, SceneData, HEIGHT, WIDTH};
use engine_core::test_engine::{LedMapping, RuntimeOptions};
use lp_gfx::lp_script::dec32::{Dec32, ToDec32};
use minifb::{Key, Window, WindowOptions};

const SCALE: usize = 16;
const STATS_BAR_HEIGHT: usize = 80;
const NUM_BUFFERS: usize = 2; // Greyscale + RGB
const WINDOW_WIDTH: usize = (WIDTH * SCALE * NUM_BUFFERS) + (WIDTH * SCALE); // Buffers + LED output
const WINDOW_HEIGHT: usize = (HEIGHT * SCALE) + STATS_BAR_HEIGHT;
const FRAMETIME_WINDOW_SECONDS: f64 = 10.0;
const HISTOGRAM_BUCKETS: usize = 24;

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

    // Create demo scene
    let config = create_demo_scene(WIDTH, HEIGHT);
    let options = RuntimeOptions::new(WIDTH, HEIGHT);
    let runtime = SceneRuntime::new(config, options).expect("Valid scene config");
    let mut scene = SceneData::from_runtime(runtime);

    let mut _frame_count = 0u32;
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
        (64, 2211.0),   // 8x8
        (144, 4616.0),  // 12x12
        (256, 7968.0),  // 16x16
        (400, 12287.0), // 20x20
        (576, 17568.0), // 24x24
    ];

    // Compute linear regression: esp32_us = slope * pixels + intercept
    fn compute_esp32_model() -> (f32, f32) {
        let n = ESP32_BENCHMARKS.len() as f32;
        let sum_x: f32 = ESP32_BENCHMARKS.iter().map(|(p, _)| *p as f32).sum();
        let sum_y: f32 = ESP32_BENCHMARKS.iter().map(|(_, us)| *us).sum();
        let sum_xy: f32 = ESP32_BENCHMARKS.iter().map(|(p, us)| *p as f32 * us).sum();
        let sum_x2: f32 = ESP32_BENCHMARKS
            .iter()
            .map(|(p, _)| (*p as f32) * (*p as f32))
            .sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;
        (slope, intercept)
    }

    let (esp32_us_per_pixel, esp32_base_us) = compute_esp32_model();

    // Frametime tracking for last 10 seconds
    struct FrametimeTracker {
        samples: VecDeque<(std::time::Instant, f64)>, // (timestamp, frametime_ms)
    }

    impl FrametimeTracker {
        fn new() -> Self {
            FrametimeTracker {
                samples: VecDeque::new(),
            }
        }

        fn add_sample(&mut self, timestamp: std::time::Instant, frametime_ms: f64) {
            self.samples.push_back((timestamp, frametime_ms));
            // Remove samples older than 10 seconds
            let cutoff = timestamp - std::time::Duration::from_secs_f64(FRAMETIME_WINDOW_SECONDS);
            while let Some(&(ts, _)) = self.samples.front() {
                if ts < cutoff {
                    self.samples.pop_front();
                } else {
                    break;
                }
            }
        }

        fn calculate_stats(&self) -> (f64, f64, f64, f64, f64, f64, f64) {
            // (avg, std_dev, p0, p10, p50, p99, p100)
            if self.samples.is_empty() {
                return (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            }

            let mut frametimes: Vec<f64> = self.samples.iter().map(|(_, ft)| *ft).collect();
            frametimes.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let count = frametimes.len() as f64;
            let avg = frametimes.iter().sum::<f64>() / count;

            let variance = frametimes.iter().map(|ft| (ft - avg).powi(2)).sum::<f64>() / count;
            let std_dev = variance.sqrt();

            let p0_idx = 0;
            let p10_idx = (count * 0.10) as usize;
            let p50_idx = (count * 0.50) as usize;
            let p99_idx = (count * 0.99) as usize;
            let p100_idx = frametimes.len() - 1;

            let p0 = frametimes[p0_idx];
            let p10 = frametimes[p10_idx.min(frametimes.len() - 1)];
            let p50 = frametimes[p50_idx.min(frametimes.len() - 1)];
            let p99 = frametimes[p99_idx.min(frametimes.len() - 1)];
            let p100 = frametimes[p100_idx];

            (avg, std_dev, p0, p10, p50, p99, p100)
        }

        fn calculate_histogram(&self) -> (Vec<usize>, f64, f64) {
            // Returns (buckets, min_us, max_us)
            if self.samples.is_empty() {
                return (vec![0; HISTOGRAM_BUCKETS], 1.0f64, 4000.0f64);
            }

            // Find min and max from actual data
            let mut min_us = f64::INFINITY;
            let mut max_us = 0.0f64;

            for (_, frametime_ms) in &self.samples {
                let frametime_us = frametime_ms * 1000.0;
                if frametime_us < min_us {
                    min_us = frametime_us;
                }
                if frametime_us > max_us {
                    max_us = frametime_us;
                }
            }

            // Ensure we have a valid range
            if min_us >= max_us || min_us <= 0.0 {
                return (vec![0; HISTOGRAM_BUCKETS], 1.0f64, 4000.0f64);
            }

            // Add 10% padding on each side for better visualization
            let range = max_us - min_us;
            let padding = range * 0.1;
            let min_us_padded = (min_us - padding).max(0.1f64); // Ensure positive
            let max_us_padded = max_us + padding;

            // Use logarithmic scaling
            let log_min = min_us_padded.ln();
            let log_max = max_us_padded.ln();
            let log_range = log_max - log_min;

            let mut buckets = vec![0; HISTOGRAM_BUCKETS];

            for (_, frametime_ms) in &self.samples {
                let frametime_us = frametime_ms * 1000.0;

                // Map to logarithmic bucket
                let log_val = frametime_us.ln();
                let normalized = (log_val - log_min) / log_range;
                let bucket_idx = (normalized * HISTOGRAM_BUCKETS as f64) as usize;
                let bucket_idx = bucket_idx.min(HISTOGRAM_BUCKETS - 1);
                buckets[bucket_idx] += 1;
            }

            (buckets, min_us_padded, max_us_padded)
        }
    }

    let mut frametime_tracker = FrametimeTracker::new();
    let mut last_frame_time = std::time::Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = std::time::Instant::now();
        let delta = frame_start.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = frame_start;

        scene_time += delta * 0.5;
        let time = scene_time.to_dec32();

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

        // Iterate through pipeline buffers and render according to their format
        let mut x_offset = 0;
        for i in 0..scene.pipeline().buffers.len() {
            if let Some(buf) = scene.pipeline().get_buffer(i) {
                match buf.last_format {
                    engine_core::test_engine::BufferFormat::ImageGrey => {
                        let greyscale = scene.pipeline().get_greyscale_dec32(i);
                        draw_greyscale(&greyscale, &mut buffer, x_offset, 0, SCALE);
                        x_offset += WIDTH * SCALE;
                    }
                    engine_core::test_engine::BufferFormat::ImageRgb => {
                        let rgb_bytes = scene.pipeline().get_rgb_bytes(i);
                        draw_rgb_2d(&rgb_bytes, &mut buffer, x_offset, 0, SCALE);
                        x_offset += WIDTH * SCALE;
                    }
                }
            }
        }

        // Draw LED output after all buffers
        let led_offset_x = x_offset;
        let led_count = scene.led_count();
        draw_leds(scene.led_output(), &mut buffer, led_offset_x, 0, SCALE);

        // Draw debug overlay on RGB buffer (buffer 1)
        let rgb_buffer_offset = WIDTH * SCALE; // Greyscale is at 0, RGB is at WIDTH*SCALE
        draw_led_debug_overlay(
            &mut buffer,
            scene.mapping(),
            led_count,
            rgb_buffer_offset,
            led_offset_x,
            0,
            SCALE,
        );

        // Predict ESP32 performance for current canvas size
        let pixels = WIDTH * HEIGHT;
        let esp32_predicted_us = esp32_us_per_pixel * pixels as f32 + esp32_base_us;
        let (avg_ms, std_dev_ms, p0_ms, p10_ms, p50_ms, p99_ms, p100_ms) =
            frametime_tracker.calculate_stats();
        let (histogram, hist_min_us, hist_max_us) = frametime_tracker.calculate_histogram();
        draw_stats_bar(
            &mut buffer,
            engine_us_avg,
            ui_us_avg,
            esp32_predicted_us,
            led_count,
            avg_ms,
            std_dev_ms,
            p0_ms,
            p10_ms,
            p50_ms,
            p99_ms,
            p100_ms,
            &histogram,
            hist_min_us,
            hist_max_us,
        );

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        _frame_count += 1;
        let full_frame_us = frame_start.elapsed().as_micros() as u64;
        total_ui_us += full_frame_us;

        // Track engine frametime for statistics
        let engine_frametime_ms = engine_us as f64 / 1000.0;
        frametime_tracker.add_sample(frame_start, engine_frametime_ms);
    }
}

fn draw_greyscale(
    greyscale: &[Dec32],
    buffer: &mut [u32],
    offset_x: usize,
    offset_y: usize,
    scale: usize,
) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let grey_val = greyscale[y * WIDTH + x];
            let grey_f = grey_val.to_f32().clamp(0.0, 1.0);
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
    use embedded_graphics::mono_font::ascii::FONT_6X10;
    use embedded_graphics::mono_font::MonoTextStyle;
    use embedded_graphics::pixelcolor::Rgb888;
    use embedded_graphics::prelude::*;
    use embedded_graphics::primitives::{Circle, PrimitiveStyle};
    use embedded_graphics::text::Text;

    let mut fb = Framebuffer::new(buffer, WINDOW_WIDTH, WINDOW_HEIGHT);
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(255, 255, 255));

    let led_count = leds.len() / 3;

    // Draw as filled circles using embedded-graphics
    for led_idx in 0..led_count {
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
        Circle::new(
            Point::new(
                center_x - (diameter / 2) as i32,
                center_y - (diameter / 2) as i32,
            ),
            diameter,
        )
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
    led_count: usize,
    rgb_offset_x: usize,
    led_offset_x: usize,
    offset_y: usize,
    scale: usize,
) {
    let mut fb = Framebuffer::new(buffer, WINDOW_WIDTH, WINDOW_HEIGHT);

    let circle_style_source = PrimitiveStyle::with_stroke(Rgb888::new(0, 255, 0), 2); // Green on source
    let text_style_small = MonoTextStyle::new(&FONT_6X10, Rgb888::new(0, 0, 0)); // Black text on green circles

    for led_idx in 0..led_count {
        if let Some(map) = mapping.get(led_idx) {
            // Source position on RGB buffer (middle panel) - now with sub-pixel precision
            let x_pixels = map.pos.x.to_f32();
            let y_pixels = map.pos.y.to_f32();
            let source_x = (rgb_offset_x as f32 + x_pixels * scale as f32) as i32;
            let source_y = (offset_y as f32 + y_pixels * scale as f32) as i32;

            // Destination position on LED strip (right panel)
            let dest_x = led_idx % 16;
            let dest_y = led_idx / 16;
            let _dest_center_x = (led_offset_x + dest_x * scale + scale / 2) as i32;
            let _dest_center_y = (offset_y + dest_y * scale + scale / 2) as i32;

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

#[allow(clippy::too_many_arguments)]
fn draw_stats_bar(
    buffer: &mut [u32],
    engine_us: f32,
    ui_us: f32,
    esp32_predicted_us: f32,
    led_count: usize,
    avg_ms: f64,
    std_dev_ms: f64,
    p0_ms: f64,
    p10_ms: f64,
    p50_ms: f64,
    p99_ms: f64,
    p100_ms: f64,
    histogram: &[usize],
    hist_min_us: f64,
    hist_max_us: f64,
) {
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
    let info_text = format!("Canvas: {}x{}  Output: {} LEDs", WIDTH, HEIGHT, led_count);
    Text::new(
        &info_text,
        Point::new(10, (HEIGHT * SCALE + 10) as i32),
        text_style,
    )
    .draw(&mut fb)
    .ok();

    // Line 2: Performance stats
    let engine_fps = if engine_us > 0.0 {
        1_000_000.0 / engine_us
    } else {
        0.0
    };
    let esp32_fps = if esp32_predicted_us > 0.0 {
        1_000_000.0 / esp32_predicted_us
    } else {
        0.0
    };
    let ui_fps = if ui_us > 0.0 {
        1_000_000.0 / ui_us
    } else {
        0.0
    };

    let perf_text = format!(
        "Engine: {:.0}us ({:.0} FPS)  ESP32 predicted: {:.0} FPS  UI: {:.0} FPS",
        engine_us, engine_fps, esp32_fps, ui_fps
    );

    Text::new(
        &perf_text,
        Point::new(10, (HEIGHT * SCALE + 22) as i32),
        text_style_bright,
    )
    .draw(&mut fb)
    .ok();

    // Line 3: Engine frametime statistics (10s window)
    let frametime_text = format!(
        "Engine (10s): avg={:.2}ms std_dev={:.2}ms  p0={:.2}ms  p10={:.2}ms  p50={:.2}ms  p99={:.2}ms  p100={:.2}ms",
        avg_ms, std_dev_ms, p0_ms, p10_ms, p50_ms, p99_ms, p100_ms
    );

    Text::new(
        &frametime_text,
        Point::new(10, (HEIGHT * SCALE + 34) as i32),
        text_style,
    )
    .draw(&mut fb)
    .ok();

    // Line 4: Histogram
    draw_histogram(
        &mut fb,
        histogram,
        HEIGHT * SCALE + 46,
        text_style,
        hist_min_us,
        hist_max_us,
    );
}

fn draw_histogram(
    fb: &mut Framebuffer,
    buckets: &[usize],
    y_pos: usize,
    text_style: MonoTextStyle<'_, Rgb888>,
    min_us: f64,
    max_us: f64,
) {
    if buckets.is_empty() {
        return;
    }

    // Find max count for scaling
    let max_count = buckets.iter().max().copied().unwrap_or(1);
    if max_count == 0 {
        return;
    }

    // Build histogram string with ASCII characters
    let mut hist_chars = Vec::new();
    let max_bar_height = 20; // Maximum height in characters

    for &count in buckets.iter() {
        let bar_height = if max_count > 0 {
            (count as f32 / max_count as f32 * max_bar_height as f32) as usize
        } else {
            0
        };

        // Use ASCII characters for different heights
        let ch = if bar_height == 0 {
            ' '
        } else if bar_height < max_bar_height / 4 {
            '.'
        } else if bar_height < max_bar_height / 2 {
            ':'
        } else if bar_height < max_bar_height * 3 / 4 {
            '|'
        } else {
            '#'
        };

        hist_chars.push(ch);
    }

    // Create histogram string
    let hist_string: String = hist_chars.iter().collect();

    // Calculate bucket boundaries for labels (logarithmic scale)
    let log_min = min_us.ln();
    let log_max = max_us.ln();
    let log_range = log_max - log_min;

    let bucket_to_us = |idx: usize| -> f64 {
        let normalized = (idx as f64 + 0.5) / HISTOGRAM_BUCKETS as f64;
        let log_val = log_min + normalized * log_range;
        log_val.exp()
    };

    let start_us = bucket_to_us(0);
    let mid_us = bucket_to_us(HISTOGRAM_BUCKETS / 2);
    let end_us = bucket_to_us(HISTOGRAM_BUCKETS - 1);

    let hist_label = format!(
        "Hist: {}  {:.0}us-{:.0}us-{:.0}us",
        hist_string, start_us, mid_us, end_us
    );

    Text::new(&hist_label, Point::new(10, y_pos as i32), text_style)
        .draw(fb)
        .ok();
}

#[inline(always)]
fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    // minifb expects 0RGB format
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
