/// Quick benchmark for test scene at multiple resolutions
use engine_core::test_engine::{fixed_from_f32, render_frame, LedMapping, LoadSource, OpCode, Palette, Fixed};
use std::time::Instant;

extern crate alloc;
use alloc::vec;

const FRAME_COUNT: u32 = 1000;
const LED_COUNT: usize = 128;

fn benchmark_size(width: usize, height: usize) {
    let buffer_size = width * height;
    
    // Create buffers
    let mut greyscale_buffer = vec![0i32; buffer_size];
    let input_buffer = vec![0i32; buffer_size];
    let mut rgb_2d_buffer = vec![0u8; buffer_size * 3];
    let mut led_output = vec![0u8; LED_COUNT * 3];
    
    // Create palette and mapping
    let palette = Palette::rainbow();
    let mapping = LedMapping::spiral(3, width, height);
    
    // Same program as SceneData
    let program = vec![
        OpCode::Load(LoadSource::XNorm),
        OpCode::Push(Fixed::from(fixed_from_f32(0.3))),
        OpCode::Mul,
        OpCode::Load(LoadSource::YNorm),
        OpCode::Push(Fixed::from(fixed_from_f32(0.3))),
        OpCode::Mul,
        OpCode::Load(LoadSource::Time),
        OpCode::Perlin3(3),
        OpCode::Cos,
        OpCode::Return,
    ];
    
    let start = Instant::now();
    
    for i in 0..FRAME_COUNT {
        let time = fixed_from_f32(i as f32 * 0.01);
        render_frame(
            &mut greyscale_buffer,
            &input_buffer,
            &mut rgb_2d_buffer,
            &mut led_output,
            &program,
            &palette,
            &mapping,
            width,
            height,
            time,
        );
    }
    
    let elapsed = start.elapsed();
    let total_us = elapsed.as_micros() as u64;
    let avg_us = total_us / FRAME_COUNT as u64;
    let fps = if avg_us > 0 { 1_000_000 / avg_us } else { 0 };
    
    println!("{}x{}: {}us/frame ({} FPS)", width, height, avg_us, fps);
}

fn main() {
    println!("Test Engine Benchmark (host)");
    println!("Running {} frames at multiple resolutions...\n", FRAME_COUNT);
    
    benchmark_size(8, 8);
    benchmark_size(12, 12);
    benchmark_size(16, 16);
    benchmark_size(20, 20);
    benchmark_size(24, 24);
}
