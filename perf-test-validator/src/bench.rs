use perf_tests_common;
use perf_tests_common::pixel_vm::{self, OpCode, FIXED_SHIFT};
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

fn benchmark_pixel_vm() {
    const VM_WIDTH: usize = 16;
    const VM_HEIGHT: usize = 16;
    const VM_SIZE: usize = VM_WIDTH * VM_HEIGHT;

    // Create buffers (grayscale, fixed-point)
    let input = vec![0i32; VM_SIZE];
    let mut output = vec![0i32; VM_SIZE];

    // Example program from plan:
    // v = perlin3(x, y, time)
    // v = cos(v)
    // v = v < 0.5 ? 0 : 1
    // return v
    let program = vec![
        // v = perlin3(x, y, time)
        OpCode::LoadX,
        OpCode::LoadY,
        OpCode::LoadTime,
        OpCode::Perlin3,
        // v = cos(v)
        OpCode::Cos,
        // v = v < 0.5 ? 0 : 1
        OpCode::Push(1 << (FIXED_SHIFT - 1)), // 0.5 in fixed-point
        OpCode::JumpLt(3),                    // if v < 0.5, jump to push 0
        OpCode::Push(1 << FIXED_SHIFT),       // push 1
        OpCode::Return,
        OpCode::Push(0), // push 0
        OpCode::Return,
    ];

    let mut total_micros = 0u64;
    let mut frame_count = 0u32;

    let test_start = Instant::now();

    // Run for 1 second
    while test_start.elapsed() < TEST_DURATION {
        let frame_start = Instant::now();
        let time = pixel_vm::fixed_from_f32(frame_count as f32 * 0.016); // ~60 FPS time step
        pixel_vm::execute_program(&input, &mut output, &program, VM_WIDTH, VM_HEIGHT, time);
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
        "pixel_vm (16x16 grayscale): {}µs/frame, {} FPS ({} frames)",
        avg_micros, fps, frame_count
    );

    // Show a sample of the output for verification
    println!(
        "  Sample output (first 4 pixels): {:?}",
        &output[0..4]
            .iter()
            .map(|&v| pixel_vm::fixed_to_f32(v))
            .collect::<Vec<_>>()
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

    println!("\n--- Stack-based VM Benchmark ---");
    benchmark_pixel_vm();

    println!("\nNote: ESP32-C3 will be ~10-50x slower than these numbers");
}
