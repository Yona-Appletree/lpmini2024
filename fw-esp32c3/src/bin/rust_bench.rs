#![no_std]
#![no_main]

use defmt::info;
use embassy_time::Instant;
use engine_core::test_engine::demo_program::create_demo_scene;
use engine_core::test_engine::scene::SceneRuntime;
use engine_core::test_engine::RuntimeOptions;
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use lpscript::math::{Fixed, ToFixed};
use panic_rtt_target as _;

esp_bootloader_esp_idf::esp_app_desc!();

const TEST_DURATION_MS: u64 = 1000;

async fn benchmark_16x16() {
    let config = create_demo_scene(16, 16);
    let options = RuntimeOptions::new(16, 16);
    let mut scene = SceneRuntime::new(config, options).expect("Failed to create scene");
    let mut frame_count = 0u32;
    let mut total_us = 0u64;
    let test_start = Instant::now();

    while test_start.elapsed().as_millis() < TEST_DURATION_MS {
        let frame_start = Instant::now();
        let time = (frame_count as f32 * 0.01).to_fixed();

        scene.render(time, 1).expect("Render failed");
        total_us += frame_start.elapsed().as_micros();
        frame_count += 1;
    }

    let avg_us = total_us / frame_count as u64;
    let fps = if avg_us > 0 { 1_000_000 / avg_us } else { 0 };

    info!("16x16: {}us/frame ({} FPS)", avg_us, fps);
}

#[esp_hal_embassy::main]
async fn main(_spawner: embassy_executor::Spawner) {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Allocate heap - 204KB total (140KB main DRAM + 64KB DRAM2)
    esp_alloc::heap_allocator!(size: 140 * 1024);
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    embassy_time::Timer::after(embassy_time::Duration::from_millis(100)).await;

    info!("Embassy initialized!");
    info!("Test Engine Benchmark - 16x16");
    info!("");

    benchmark_16x16().await;

    info!("");
    info!("Complete");

    loop {
        // Keep alive
    }
}
