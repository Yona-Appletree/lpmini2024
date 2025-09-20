#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

extern crate alloc;
// use esp_wifi::ble::controller::BleConnector;
use core::slice::from_ref;

use bt_hci::controller::ExternalController;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::rmt::Rmt;
use esp_hal::spi::master::Spi;
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{spi, Blocking};
use esp_hal_smartled::{buffer_size_async, SmartLedsAdapterAsync};
use panic_rtt_target as _;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::{brightness, gamma, SmartLedsWriteAsync, RGB8};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const NUM_LEDS: usize = 240;

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

    let mut spi = Spi::new(
        peripherals.SPI2,
        spi::master::Config::default().with_frequency(Rate::from_khz(3_800)),
    )
    .unwrap()
    .with_sck(peripherals.GPIO0)
    .with_mosi(peripherals.GPIO4)
    .with_miso(peripherals.GPIO2);

    let mut color = Hsv {
        hue: 0,
        sat: 255,
        val: 127,
    };
    let mut data = [RGB8::new(255, 0, 0); NUM_LEDS];
    let level = 100;

    // TODO: Spawn some tasks
    let _ = spawner;

    let mut counter = 0u8;

    loop {
        counter = counter.wrapping_add(1);

        // Fill all LEDs with a rainbow pattern
        for (i, led_data) in data.iter_mut().enumerate() {
            color.hue = counter.wrapping_add(i as u8);

            // Convert from the HSV color space (where we can easily transition from one
            // color to the other) to the RGB color space that we can then send to the LED
            *led_data = hsv2rgb(color);
        }

        // When sending to the LEDs, we do a gamma correction first (see smart_leds
        // documentation for details) and then limit the brightness to 10 out of 255 so
        // that the output is not too bright.
        spi_write_rgb(&mut spi, &data).unwrap();

        Timer::after(Duration::from_millis(10)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}

/// Inspired by https://github.com/smart-leds-rs/ws2812-spi-rs
fn spi_write_rgb(spi: &mut Spi<Blocking>, data: &[RGB8]) -> Result<(), spi::Error> {
    for pixel in data {
        spi_write_byte(spi, pixel.r)?;
        spi_write_byte(spi, pixel.g)?;
        spi_write_byte(spi, pixel.b)?;
    }
    Ok(())
}

/// Write a single byte for ws2812 devices
fn spi_write_byte(spi: &mut Spi<Blocking>, mut data: u8) -> Result<(), spi::Error> {
    // Send two bits in one spi byte. High time first, then the low time
    // The maximum for T0H is 500ns, the minimum for one bit 1063 ns.
    // These result in the upper and lower spi frequency limits
    let patterns = [0b1000_1000, 0b1000_1110, 0b11101000, 0b11101110];
    for _ in 0..4 {
        let bits = (data & 0b1100_0000) >> 6;
        spi.write(from_ref(&patterns[bits as usize]))?;
        data = data << 2;
    }
    Ok(())
}

fn spi_end_frame(spi: &mut Spi<Blocking>) -> Result<(), spi::Error> {
    // Should be > 300μs, so for an SPI Freq. of 3.8MHz, we have to send at least 1140 low bits or 140 low bytes
    for _ in 0..140 {
        spi.write(from_ref(&0))?;
    }
    Ok(())
}
