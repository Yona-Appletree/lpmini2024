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
use embassy_time::{Duration, Instant, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use fw_esp32c3::rmt_ws2811_driver;
use panic_rtt_target as _;
use smart_leds::RGB8;

// Engine imports
use engine_core::demo_program::create_demo_scene;
use engine_core::power_limit::{apply_power_limit, PowerLimitConfig};
use engine_core::scene::SceneRuntime;
use engine_core::test_engine::{fixed_from_f32, RuntimeOptions};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const WIDTH: usize = 16;
const HEIGHT: usize = 16;

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    // Create the engine scene
    info!("Creating {}x{} scene...", WIDTH, HEIGHT);
    let scene_config = create_demo_scene(WIDTH, HEIGHT);
    let num_leds = scene_config.led_count();
    let options = RuntimeOptions::new(WIDTH, HEIGHT);
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
    const TARGET_FPS: u32 = 60;
    const FRAME_TIME_MS: u32 = 1000 / TARGET_FPS;
    const TIME_SPEED_256: u32 = 64; // 64/256 = 0.25 (4x slower)

    // Power limiting configuration
    let power_config = PowerLimitConfig {
        brightness_256: 32,    // ~12.5% brightness (32/256)
        power_budget_ma: 1000, // 1A budget
        led_white_power_ma: 50,
        led_idle_power_ma: 1,
    };

    let mut delay = Delay::new();
    let start_time = Instant::now();
    let mut frame_count = 0u32;
    let mut last_fps_time = start_time;
    let mut last_fps_frame = 0u32;

    loop {
        let frame_start = Instant::now();

        // Calculate time in fixed-point (seconds since start) with speed adjustment
        let elapsed_ms = frame_start.duration_since(start_time).as_millis() as u32;
        let adjusted_ms = (elapsed_ms * TIME_SPEED_256) / 256;
        let time = fixed_from_f32((adjusted_ms as f32) / 1000.0);

        // Render the scene (outputs to scene.led_output)
        scene.render(time, 1).expect("Render failed");

        // Convert engine output (u8 RGB bytes) to smart_leds::RGB8
        let mut led_buffer = alloc::vec![RGB8 { r: 0, g: 0, b: 0 }; num_leds];
        for i in 0..num_leds {
            let idx = i * 3;
            led_buffer[i] = RGB8 {
                r: scene.led_output[idx],
                g: scene.led_output[idx + 1],
                b: scene.led_output[idx + 2],
            };
        }

        // Apply power limiting and gamma correction
        apply_power_limit(&mut led_buffer, &power_config);

        // Wait for previous transmission to complete before writing new data
        rmt_ws2811_driver::rmt_ws2811_wait_complete();
        // Write to LEDs and start transmission
        rmt_ws2811_driver::rmt_ws2811_write(&led_buffer);

        frame_count += 1;

        // Log FPS every second
        if frame_start.duration_since(last_fps_time).as_millis() >= 1000 {
            let frames_rendered = frame_count - last_fps_frame;
            let elapsed_ms = frame_start.duration_since(last_fps_time).as_millis();
            let fps = (frames_rendered * 1000) / elapsed_ms as u32;

            info!("FPS: {}, Frame: {}", fps, frame_count);

            last_fps_time = frame_start;
            last_fps_frame = frame_count;
        }

        // Frame rate limiting
        let frame_time = frame_start.elapsed().as_millis() as u32;
        if frame_time < FRAME_TIME_MS {
            let sleep_ms = FRAME_TIME_MS - frame_time;
            Timer::after(Duration::from_millis(sleep_ms as u64)).await;
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
