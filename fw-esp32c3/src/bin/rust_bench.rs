#![no_std]
#![no_main]

extern crate alloc;

use defmt::info;
use embassy_time::Instant;
use engine_core::test_engine::fixed_from_f32;
use engine_core::test_scene::{render_test_scene, SceneData};
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use panic_rtt_target as _;

esp_bootloader_esp_idf::esp_app_desc!();

const TEST_DURATION_MS: u64 = 1000;

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
    info!("Test Engine Benchmark - {}ms test", TEST_DURATION_MS);
    info!("");

    let mut scene = SceneData::new();
    let mut frame_count = 0u32;
    let mut total_us = 0u64;
    let test_start = Instant::now();

    while test_start.elapsed().as_millis() < TEST_DURATION_MS {
        let frame_start = Instant::now();
        let time = fixed_from_f32(frame_count as f32 * 0.01);
        render_test_scene(&mut scene, time);
        total_us += frame_start.elapsed().as_micros();
        frame_count += 1;
    }

    let avg_us = total_us / frame_count as u64;
    let fps = if avg_us > 0 { 1_000_000 / avg_us } else { 0 };

    info!(
        "test_engine: {}µs/frame, {} FPS ({} frames)",
        avg_us, fps, frame_count
    );

    info!("");

    loop {
        // Keep alive
    }
}
