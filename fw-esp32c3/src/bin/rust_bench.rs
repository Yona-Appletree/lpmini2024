#![no_std]
#![no_main]

extern crate alloc;

use defmt::info;
use embassy_time::Instant;
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use panic_rtt_target as _;
use perf_tests_common::RenderFn;

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

// Configurable matrix size
const MATRIX_WIDTH: usize = 16;
const MATRIX_HEIGHT: usize = 16;
const BYTES_PER_PIXEL: usize = 3; // RGB
const BUFFER_SIZE: usize = MATRIX_WIDTH * MATRIX_HEIGHT * BYTES_PER_PIXEL;

const TEST_DURATION_MS: u64 = 1000; // Run each test for 1 second

/// Run a performance test for a specific render function
async fn run_test(name: &str, render_fn: RenderFn, buffer: &mut [u8]) {
    let mut frame_count = 0u32;
    let mut total_us = 0u64;
    let test_start = Instant::now();

    // Run for 1 second
    while test_start.elapsed().as_millis() < TEST_DURATION_MS {
        let frame_start = Instant::now();
        render_fn(buffer, frame_count as f32, MATRIX_WIDTH, MATRIX_HEIGHT);
        let frame_elapsed = frame_start.elapsed();
        total_us += frame_elapsed.as_micros();
        frame_count += 1;
    }

    let avg_us = total_us / frame_count as u64;
    let avg_fps = if avg_us > 0 { 1_000_000 / avg_us } else { 0 };
    
    info!("{}: {}µs/frame, {} FPS ({} frames)", name, avg_us, avg_fps, frame_count);
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

    // Give RTT time to initialize
    embassy_time::Timer::after(embassy_time::Duration::from_millis(100)).await;

    info!("Embassy initialized!");
    info!("");
    info!("Performance Tests: {}x{} RGB, {}ms per test", MATRIX_WIDTH, MATRIX_HEIGHT, TEST_DURATION_MS);
    info!("");

    let mut buffer = [0u8; BUFFER_SIZE];

    // Test one at a time to debug RTT issues
    info!("Starting tests...");
    
    run_test("perlin3_fixed", perf_tests_common::perlin3_fixed::render_frame, &mut buffer).await;
    
    info!("Test 1 complete, waiting...");
    embassy_time::Timer::after(embassy_time::Duration::from_millis(500)).await;
    
    run_test("perlin3_decimal", perf_tests_common::perlin3_decimal::render_frame, &mut buffer).await;

    info!("");

    loop {
        // Keep alive
    }
}
