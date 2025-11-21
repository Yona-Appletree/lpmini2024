#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

extern crate alloc;
use core::sync::atomic::{AtomicU32, Ordering};

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
// Engine imports
use engine_core::test_engine::demo_program::create_demo_scene;
use engine_core::test_engine::scene::SceneRuntime;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use fw_esp32c3::rmt_ws2811_driver;
use lp_script::dec32::Dec32;
use panic_rtt_target as _;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const WIDTH: usize = 16;
const HEIGHT: usize = 16;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Allocate heap in DRAM (regular heap) - can allocate more here
    esp_alloc::heap_allocator!(size: 140 * 1024);
    // DRAM2 is a dec32-size region (~64KB), keep this conservative
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    // Initialize RTT after heap setup to avoid memory conflicts
    rtt_target::rtt_init_defmt!();

    info!("Embassy initialized!");

    // Configure RMT (Remote Control Transceiver) peripheral globally
    let rmt: Rmt<'_, esp_hal::Blocking> = {
        let frequency: Rate = Rate::from_mhz(80);
        Rmt::new(peripherals.RMT, frequency)
    }
    .expect("Failed to initialize RMT");

    // Initialize WS2811 LED driver
    info!("Initializing WS2811 driver...");
    const test_count: usize = 16;
    let _rmt_tx = rmt_ws2811_driver::rmt_ws2811_init(rmt, peripherals.GPIO4, test_count)
        .expect("Failed to initialize WS2811 driver");

    info!("Starting render loop");

    // Runtime configuration
    const TIME_SPEED_256: u32 = 64; // 64/256 = 0.25 (4x slower)

    let start_time = Instant::now();
    let mut frame_count = 0u32;
    let mut last_fps_time = start_time;
    let mut last_fps_frame = 0u32;

    let mut test_buffer: [u8; test_count * 3] = [0; test_count * 3];

    loop {
        let frame_start = Instant::now();

        // Calculate time in dec32-point (seconds since start) with speed adjustment
        let elapsed_ms = frame_start.duration_since(start_time).as_millis() as u32;
        let adjusted_ms = (elapsed_ms * TIME_SPEED_256) / 256;
        // Convert ms to seconds in dec32-point: (ms * Dec32::ONE) / 1000
        let time = ((adjusted_ms as i64 * Dec32::ONE.0 as i64) / 1000) as i32;

        // Log FPS
        if frame_start.duration_since(last_fps_time).as_millis() >= 1000 {
            let frames_rendered = frame_count - last_fps_frame;
            let elapsed_ms = frame_start.duration_since(last_fps_time).as_millis();
            let _fps = (frames_rendered * 1000) / elapsed_ms as u32;

            info!("FPS: {}, Frame: {}", _fps, frame_count);

            last_fps_time = frame_start;
            last_fps_frame = frame_count;
        }

        fill_rainbow(&mut test_buffer, test_count);

        // Write to LEDs
        rmt_ws2811_driver::rmt_ws2811_wait_complete();
        Timer::after(Duration::from_millis(1)).await;
        rmt_ws2811_driver::rmt_ws2811_write_bytes(&test_buffer);

        frame_count += 1;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}

static TIME_COUNTER: AtomicU32 = AtomicU32::new(0);

fn fill_rainbow(buffer: &mut [u8], length: usize) {
    let raw_counter = TIME_COUNTER.load(Ordering::Relaxed); // 64x slower
    let time_counter = raw_counter / 64;

    for i in 0..length {
        let hue = ((time_counter + i as u32) as f32 / length as f32) % 1.0;
        let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);
        buffer[i * 3] = r >> 4;
        buffer[i * 3 + 1] = g >> 4;
        buffer[i * 3 + 2] = b >> 4;
    }
    TIME_COUNTER.store(raw_counter.wrapping_add(1), Ordering::Relaxed);
}

fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (u8, u8, u8) {
    let c = value * saturation;
    let h = hue * 6.0;
    let mut h_int = h as i32;
    let h_frac = h - h_int as f32;

    // Handle negative h_int (shouldn't happen with normalized hue, but be safe)
    if h_int < 0 {
        h_int += 6;
    }
    h_int %= 6;

    let x = c * (1.0 - (h_frac * 2.0 - 1.0).abs());
    let m = value - c;

    let (r, g, b) = match h_int {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    (
        ((r + m) * 255.0).min(255.0).max(0.0) as u8,
        ((g + m) * 255.0).min(255.0).max(0.0) as u8,
        ((b + m) * 255.0).min(255.0).max(0.0) as u8,
    )
}
