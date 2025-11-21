#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

extern crate alloc;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Instant;
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

    // Create the engine scene with demo pattern
    info!("Creating {}x{} demo scene...", WIDTH, HEIGHT);
    let scene_config = create_demo_scene(WIDTH, HEIGHT);
    let num_leds = scene_config.led_count();

    // Power limiting configuration
    let power_config = engine_core::test_engine::power_limit::PowerLimitConfig {
        brightness_256: 255,   // ~12.5% brightness (32/256)
        power_budget_ma: 1000, // 1A budget
        led_white_power_ma: 50,
        led_idle_power_ma: 1,
    };

    let options =
        engine_core::test_engine::RuntimeOptions::with_power_config(WIDTH, HEIGHT, power_config);
    let mut scene = SceneRuntime::new(scene_config, options).expect("Failed to create scene");

    info!("Scene created with {} LEDs", num_leds);

    // Configure RMT (Remote Control Transceiver) peripheral globally
    let rmt: Rmt<'_, esp_hal::Blocking> = {
        let frequency: Rate = Rate::from_mhz(80);
        Rmt::new(peripherals.RMT, frequency)
    }
    .expect("Failed to initialize RMT");

    // Initialize WS2811 LED driver
    info!("Initializing WS2811 driver...");
    let _rmt_tx = rmt_ws2811_driver::rmt_ws2811_init(rmt, peripherals.GPIO4, num_leds)
        .expect("Failed to initialize WS2811 driver");

    info!("Starting render loop");

    // Runtime configuration
    const TIME_SPEED_256: u32 = 64; // 64/256 = 0.25 (4x slower)

    let _delay = Delay::new();
    let start_time = Instant::now();
    let mut frame_count = 0u32;
    let mut last_fps_time = start_time;
    let mut last_fps_frame = 0u32;

    loop {
        let frame_start = Instant::now();

        // Calculate time in dec32-point (seconds since start) with speed adjustment
        let elapsed_ms = frame_start.duration_since(start_time).as_millis() as u32;
        let adjusted_ms = (elapsed_ms * TIME_SPEED_256) / 256;
        // Convert ms to seconds in dec32-point: (ms * Dec32::ONE) / 1000
        let time = ((adjusted_ms as i64 * Dec32::ONE.0 as i64) / 1000) as i32;

        // Render the scene (outputs to scene.led_output with power limiting applied)
        scene.render(Dec32(time), 1).expect("Render failed");

        // Log FPS every second
        if frame_start.duration_since(last_fps_time).as_millis() >= 1000 {
            let frames_rendered = frame_count - last_fps_frame;
            let elapsed_ms = frame_start.duration_since(last_fps_time).as_millis();
            let _fps = (frames_rendered * 1000) / elapsed_ms as u32;

            info!("FPS: {}, Frame: {}", _fps, frame_count);

            last_fps_time = frame_start;
            last_fps_frame = frame_count;
        }

        // Write to LEDs and start transmission (directly from scene buffer)
        rmt_ws2811_driver::rmt_ws2811_write_bytes(&scene.led_output);

        frame_count += 1;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
