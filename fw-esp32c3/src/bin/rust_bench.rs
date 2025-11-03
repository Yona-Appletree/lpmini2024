#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec;

use defmt::info;
use embassy_time::Instant;
use engine_core::test_engine::{fixed_from_f32, render_frame, LedMapping, LoadSource, OpCode, Palette};
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use panic_rtt_target as _;

esp_bootloader_esp_idf::esp_app_desc!();

const TEST_DURATION_MS: u64 = 1000;
const LED_COUNT: usize = 128;

async fn benchmark_size(width: usize, height: usize) {
    let buffer_size = width * height;
    
    let mut greyscale_buffer = vec![0i32; buffer_size];
    let input_buffer = vec![0i32; buffer_size];
    let mut rgb_2d_buffer = vec![0u8; buffer_size * 3];
    let mut led_output = vec![0u8; LED_COUNT * 3];
    
    let palette = Palette::rainbow();
    let mapping = LedMapping::spiral(3, width, height);
    
    let program = vec![
        OpCode::Load(LoadSource::XNorm),
        OpCode::Push(fixed_from_f32(0.3)),
        OpCode::Mul,
        OpCode::Load(LoadSource::YNorm),
        OpCode::Push(fixed_from_f32(0.3)),
        OpCode::Mul,
        OpCode::Load(LoadSource::Time),
        OpCode::Perlin3(3),
        OpCode::Cos,
        OpCode::Return,
    ];
    
    let mut frame_count = 0u32;
    let mut total_us = 0u64;
    let test_start = Instant::now();

    while test_start.elapsed().as_millis() < TEST_DURATION_MS {
        let frame_start = Instant::now();
        let time = fixed_from_f32(frame_count as f32 * 0.01);
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
        total_us += frame_start.elapsed().as_micros();
        frame_count += 1;
    }

    let avg_us = total_us / frame_count as u64;
    let fps = if avg_us > 0 { 1_000_000 / avg_us } else { 0 };

    info!("{}x{}: {}us/frame ({} FPS)", width, height, avg_us, fps);
}

#[esp_hal_embassy::main]
async fn main(_spawner: embassy_executor::Spawner) {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Allocate heap
    esp_alloc::heap_allocator!(size: 64 * 1024);
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    embassy_time::Timer::after(embassy_time::Duration::from_millis(100)).await;

    info!("Embassy initialized!");
    info!("Test Engine Benchmark - multiple resolutions");
    info!("");

    benchmark_size(8, 8).await;
    embassy_time::Timer::after(embassy_time::Duration::from_millis(200)).await;
    
    benchmark_size(12, 12).await;
    embassy_time::Timer::after(embassy_time::Duration::from_millis(200)).await;
    
    benchmark_size(16, 16).await;
    embassy_time::Timer::after(embassy_time::Duration::from_millis(200)).await;
    
    benchmark_size(20, 20).await;
    embassy_time::Timer::after(embassy_time::Duration::from_millis(200)).await;
    
    benchmark_size(24, 24).await;

    info!("");

    loop {
        // Keep alive
    }
}
