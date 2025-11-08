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

    esp_alloc::heap_allocator!(size: 140 * 1024);
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

    info!("Starting horizontal line test");
    info!("Hard-coded LED indices from visualizer");

    let delay = Delay::new();
    let mut led_buffer = alloc::vec![RGB8 { r: 0, g: 0, b: 0 }; NUM_LEDS];

    let _rmt_tx = rmt_ws2811_driver::rmt_ws2811_init(rmt, peripherals.GPIO4, NUM_LEDS)
        .expect("Failed to initialize WS2811 driver");

    // Hard-coded LED indices that should form a horizontal line (from visualizer)
    let horizontal_leds = [0, 1, 5, 9, 15, 21, 29, 47, 57, 69, 81, 97];

    info!(
        "Turning on {} LEDs for horizontal line",
        horizontal_leds.len()
    );

    // Set these specific LEDs to white
    for &led_idx in horizontal_leds.iter() {
        if led_idx < NUM_LEDS {
            led_buffer[led_idx] = RGB8 {
                r: BRIGHTNESS,
                g: BRIGHTNESS,
                b: BRIGHTNESS,
            };
            info!("LED {} set to white", led_idx);
        }
    }

    // Convert to bytes for RMT driver
    let mut output_bytes = alloc::vec![0u8; NUM_LEDS * 3];
    for (i, led) in led_buffer.iter().enumerate() {
        output_bytes[i * 3] = led.r;
        output_bytes[i * 3 + 1] = led.g;
        output_bytes[i * 3 + 2] = led.b;
    }

    // Write to LEDs
    rmt_ws2811_driver::rmt_ws2811_write_bytes(&output_bytes);

    info!("Horizontal line displayed - should see a straight line");
    info!("Holding pattern forever...");

    // Hold the pattern forever
    loop {
        delay.delay_millis(1000);
    }
}
