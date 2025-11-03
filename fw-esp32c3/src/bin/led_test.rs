#![no_std]
#![no_main]

extern crate alloc;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::riscv::asm::delay;
use esp_hal::rmt::Rmt;
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use fw_esp32c3::rmt_ws2811_driver;
use panic_rtt_target as _;
use smart_leds::RGB8;

esp_bootloader_esp_idf::esp_app_desc!();

const NUM_LEDS: usize = 113;
const BRIGHTNESS: u8 = 8; // Very low brightness for testing

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("LED Mapping Test - Starting");

    // Initialize RMT
    let rmt: Rmt<'_, esp_hal::Blocking> = {
        let frequency: Rate = Rate::from_mhz(80);
        Rmt::new(peripherals.RMT, frequency)
    }
    .expect("Failed to initialize RMT");

    // Initialize WS2811 LED driver
    info!("Initializing WS2811 driver with {} LEDs", NUM_LEDS);

    info!("Starting sequential LED test");
    info!("Pattern: Red, Blue, Green, Red, Blue, Green...");
    info!("100ms per LED");

    let mut delay = Delay::new();
    let mut led_buffer = alloc::vec![RGB8 { r: 0, g: 0, b: 0 }; NUM_LEDS];

    delay.delay_millis(10);

    let _rmt_tx = rmt_ws2811_driver::rmt_ws2811_init(rmt, peripherals.GPIO4, NUM_LEDS)
        .expect("Failed to initialize WS2811 driver");

    delay.delay_millis(10);

    loop {
        // Turn on LEDs one at a time
        for i in 0..NUM_LEDS {
            // Determine color based on position
            let color = match i % 3 {
                0 => RGB8 {
                    r: BRIGHTNESS,
                    g: 0,
                    b: 0,
                }, // Red
                1 => RGB8 {
                    r: 0,
                    g: 0,
                    b: BRIGHTNESS,
                }, // Blue
                2 => RGB8 {
                    r: 0,
                    g: BRIGHTNESS,
                    b: 0,
                }, // Green
                _ => RGB8 { r: 0, g: 0, b: 0 },
            };

            // Set this LED to its color
            led_buffer[i] = color;

            // Write to LEDs
            rmt_ws2811_driver::rmt_ws2811_wait_complete();

            // Wait 100ms before next LED
            delay.delay_millis(100);

            rmt_ws2811_driver::rmt_ws2811_write(&led_buffer);
        }

        // Reset all LEDs to off
        for i in 0..NUM_LEDS {
            led_buffer[i] = RGB8 { r: 0, g: 0, b: 0 };
        }
        rmt_ws2811_driver::rmt_ws2811_write(&led_buffer);
        rmt_ws2811_driver::rmt_ws2811_wait_complete();

        delay.delay_millis(500);
    }
}
