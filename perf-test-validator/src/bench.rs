use perf_tests_common;
use perf_tests_common::test_engine::{fixed_from_f32, OpCode, FIXED_SHIFT};
use perf_tests_common::test_engine::{render_frame, LedMapping, Palette};
use std::time::{Duration, Instant};

const WIDTH: usize = 64;
const HEIGHT: usize = 64;
const BUFFER_SIZE: usize = WIDTH * HEIGHT * 3;
const TEST_DURATION: Duration = Duration::from_secs(1);

fn benchmark(name: &str, render_fn: perf_tests_common::RenderFn) {
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut total_micros = 0u64;
    let mut frame_count = 0u32;

    let test_start = Instant::now();

    // Run for 1 second
    while test_start.elapsed() < TEST_DURATION {
        let frame_start = Instant::now();
        render_fn(&mut buffer, frame_count as f32, WIDTH, HEIGHT);
        total_micros += frame_start.elapsed().as_micros() as u64;
        frame_count += 1;
    }

    let avg_micros = total_micros / frame_count as u64;
    let fps = if avg_micros > 0 {
        1_000_000 / avg_micros
    } else {
        0
    };

    println!(
        "{}: {}µs/frame, {} FPS ({} frames)",
        name, avg_micros, fps, frame_count
    );
}

fn benchmark_test_engine() {
    const WIDTH: usize = 16;
    const HEIGHT: usize = 16;
    const BUFFER_SIZE: usize = WIDTH * HEIGHT;
    const LED_COUNT: usize = 128;

    // Create all buffers
    let mut greyscale_buffer = vec![0i32; BUFFER_SIZE];
    let input_buffer = vec![0i32; BUFFER_SIZE];
    let mut rgb_2d_buffer = vec![0u8; BUFFER_SIZE * 3];
    let mut led_output = vec![0u8; LED_COUNT * 3];

    // Create palette (rainbow)
    let palette = Palette::rainbow();

    // Create LED mapping (serpentine for realistic LED strip)
    // Maps 128 LEDs to first 8 rows of 16x16 grid
    let mapping = LedMapping::serpentine_16x8();

    // Example program: animated perlin noise with color threshold
    // v = perlin3(x, y, time)
    // v = cos(v)
    // v = (v + 1) / 2  // normalize to 0..1
    let program = vec![
        OpCode::LoadX,
        OpCode::LoadY,
        OpCode::LoadTime,
        OpCode::Perlin3,
        OpCode::Cos,
        // Normalize from -1..1 to 0..1
        OpCode::Push(1 << FIXED_SHIFT), // push 1
        OpCode::Add,                    // v + 1
        OpCode::Push(2 << FIXED_SHIFT), // push 2
        OpCode::Div,                    // (v + 1) / 2
        OpCode::Return,
    ];

    let mut total_micros = 0u64;
    let mut frame_count = 0u32;

    let test_start = Instant::now();

    // Run for 1 second
    while test_start.elapsed() < TEST_DURATION {
        let frame_start = Instant::now();
        let time = fixed_from_f32(frame_count as f32 * 0.016);

        // Full pipeline: VM -> RGB -> LED mapping
        render_frame(
            &mut greyscale_buffer,
            &input_buffer,
            &mut rgb_2d_buffer,
            &mut led_output,
            &program,
            &palette,
            &mapping,
            WIDTH,
            HEIGHT,
            time,
        );

        total_micros += frame_start.elapsed().as_micros() as u64;
        frame_count += 1;
    }

    let avg_micros = total_micros / frame_count as u64;
    let fps = if avg_micros > 0 {
        1_000_000 / avg_micros
    } else {
        0
    };

    println!(
        "test_engine (16x16 -> 128 LEDs): {}µs/frame, {} FPS ({} frames)",
        avg_micros, fps, frame_count
    );

    // Show a sample of the LED output for verification
    println!(
        "  Sample LED output (first LED RGB): [{}, {}, {}]",
        led_output[0], led_output[1], led_output[2]
    );
}

fn main() {
    println!("Running performance benchmarks on HOST");
    println!("Matrix: {}x{} RGB, 1s per test\n", WIDTH, HEIGHT);

    benchmark(
        "perlin3_float_libm",
        perf_tests_common::perlin3_float_libm::render_frame,
    );
    benchmark(
        "perlin3_float_approx",
        perf_tests_common::perlin3_float_approx::render_frame,
    );
    benchmark(
        "perlin3_fixed",
        perf_tests_common::perlin3_fixed::render_frame,
    );
    benchmark(
        "perlin3_decimal",
        perf_tests_common::perlin3_decimal::render_frame,
    );
    benchmark(
        "perlin3_fixed_crate",
        perf_tests_common::perlin3_fixed_crate::render_frame,
    );

    println!("\n--- Test Engine (Full Pipeline) ---");
    benchmark_test_engine();

    println!("\nNote: ESP32-C3 will be ~10-50x slower than these numbers");
}
