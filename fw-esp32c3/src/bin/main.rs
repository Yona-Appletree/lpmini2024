#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

extern crate alloc;
use bt_hci::controller::ExternalController;
// use esp_wifi::ble::controller::BleConnector;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::Level;
use esp_hal::rmt::{PulseCode, Rmt};
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use fw_esp32c3::rmt_ws2811_driver;
use panic_rtt_target as _;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::{brightness, gamma, SmartLedsWriteAsync, RGB8};
use smart_leds_trait::SmartLedsWrite;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const NUM_LEDS: usize = 64;

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

    let rng = esp_hal::rng::Rng::new(peripherals.RNG);
    let timer1 = TimerGroup::new(peripherals.TIMG0);

    // Configure RMT (Remote Control Transceiver) peripheral globally
    // <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/peripherals/rmt.html>
    let rmt: Rmt<'_, esp_hal::Blocking> = {
        let frequency: Rate = Rate::from_mhz(80);
        Rmt::new(peripherals.RMT, frequency)
    }
    .expect("Failed to initialize RMT");

    // Initialize WS2811 LED driver
    info!("Initializing WS2811 driver with {} LEDs...", NUM_LEDS);
    let _rmt_tx = rmt_ws2811_driver::rmt_ws2811_init(rmt, peripherals.GPIO4, NUM_LEDS)
        .expect("Failed to initialize WS2811 driver");

    info!("WS2811 driver initialized, starting LED loop");

    // Create simple test pattern
    let mut led_buffer = [RGB8 { r: 0, g: 0, b: 0 }; NUM_LEDS];
    let mut frame_counter = 0u32;

    loop {
        // Generate rainbow pattern
        let mut hsv = smart_leds::hsv::Hsv {
            hue: 0,
            sat: 255,
            val: 5,
        };

        for (i, led) in led_buffer.iter_mut().enumerate() {
            hsv.hue = (((i as u32 * 255 / NUM_LEDS as u32) + frame_counter) % 255) as u8;
            *led = hsv2rgb(hsv);
        }

        // Write to LEDs
        rmt_ws2811_driver::rmt_ws2811_write(&led_buffer);
        
        // Wait for transmission to complete
        rmt_ws2811_driver::rmt_ws2811_wait_complete();

        frame_counter = frame_counter.wrapping_add(1);

        if frame_counter % 20 == 0 {
            info!("Frame: {}", frame_counter);
        }

        // Small delay between frames
        Timer::after(Duration::from_millis(50)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
