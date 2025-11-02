use perf_tests_common;
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
    let fps = if avg_micros > 0 { 1_000_000 / avg_micros } else { 0 };
    
    println!("{}: {}µs/frame, {} FPS ({} frames)", name, avg_micros, fps, frame_count);
}

fn main() {
    println!("Running performance benchmarks on HOST");
    println!("Matrix: {}x{} RGB, 1s per test\n", WIDTH, HEIGHT);
    
    benchmark("perlin3_float_libm", perf_tests_common::perlin3_float_libm::render_frame);
    benchmark("perlin3_float_approx", perf_tests_common::perlin3_float_approx::render_frame);
    benchmark("perlin3_fixed", perf_tests_common::perlin3_fixed::render_frame);
    benchmark("perlin3_fixed_crate", perf_tests_common::perlin3_fixed_crate::render_frame);
    
    println!("\nNote: ESP32-C3 will be ~10-50x slower than these numbers");
}

