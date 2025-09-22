//! # RMT Interrupt-Driven Streaming Demo
//!
//! This demonstrates the proper approach for streaming large numbers of LEDs
//! using the ESP32-C3 RMT peripheral with interrupt-based buffer management.
//!
//! ## Key Concepts Demonstrated:
//!
//! 1. **Double Buffering**: Split RMT buffer into two halves for seamless streaming
//! 2. **Threshold Interrupts**: Refill buffer when hardware reaches 50% transmission
//! 3. **Continuous Transmission**: Hardware never stops - prevents LED latching
//! 4. **Memory Efficiency**: Minimal 48-word buffer (1 block) handles unlimited LEDs via streaming
//! 5. **Zero-Copy Updates**: Direct RMT memory updates during transmission
//!
//! ## Production Implementation Notes:
//!
//! In a production system, you would:
//! - Bind `rmt_interrupt_handler` to the actual RMT interrupt vector  
//! - Enable hardware threshold and end interrupts
//! - Let hardware automatically call interrupts (no manual polling)
//! - Stream from larger LED arrays by chunking through the small buffer
//!
//! This demo simulates the interrupt behavior for demonstration purposes.

use core::ptr::addr_of_mut;

use defmt::info;
use esp_hal::clock::Clocks;
use esp_hal::gpio::interconnect::PeripheralOutput;
use esp_hal::gpio::Level;
use esp_hal::interrupt::{self, InterruptHandler, Priority};
use esp_hal::rmt::{
    Channel, Error as RmtError, PulseCode, RawChannelAccess, TxChannel, TxChannelConfig,
    TxChannelCreator, TxChannelInternal,
};
use esp_hal::Blocking;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::RGB8;

// Configuration constants
const NUM_LEDS: usize = 250; // Total number of LEDs in the strip

// Buffer size for 8 LEDs worth of data (double buffered)
// Using memsize(4) = 192 words. 8 LEDs = 192 words exactly, 4 LEDs per half
const BUFFER_LEDS: usize = 8;
const HALF_BUFFER_LEDS: usize = BUFFER_LEDS / 2;
const BITS_PER_LED: usize = 3 * 8;
const HALF_BUFFER_SIZE: usize = (BUFFER_LEDS * BITS_PER_LED) / 2;
const FULL_BUFFER_SIZE: usize = BUFFER_LEDS * BITS_PER_LED;

// Global state for interrupt handling
static mut RMT_BUFFER: [u32; FULL_BUFFER_SIZE] = [0; FULL_BUFFER_SIZE];
static mut LED_DATA_BUFFER: [RGB8; NUM_LEDS] = [RGB8 { r: 0, g: 0, b: 0 }; NUM_LEDS];
static mut FRAME_COUNTER: usize = 0;
static mut LED_COUNTER: usize = 0; // Track current LED position in the strip
static mut INTERRUPT_ACTIVE: bool = false;
static mut RMT_PTR: *mut u32 = 0 as *mut u32;

static mut RMT_STATS_COUNT: i32 = 0;
static mut RMT_STATS_SUM: i32 = 0;

const SRC_CLOCK_MHZ: u32 = 80;
const PULSE_ZERO: u32 = // Zero
    pulseCode(
        Level::High,
        ((SK68XX_T0H_NS * SRC_CLOCK_MHZ) / 1000) as u16,
        Level::Low,
        ((SK68XX_T0L_NS * SRC_CLOCK_MHZ) / 1000) as u16,
    );

// One
const PULSE_ONE: u32 = pulseCode(
    Level::High,
    ((SK68XX_T1H_NS * SRC_CLOCK_MHZ) / 1000) as u16,
    Level::Low,
    ((SK68XX_T1L_NS * SRC_CLOCK_MHZ) / 1000) as u16,
);

// Latch
const PULSE_LATCH: u32 = pulseCode(Level::Low, 3000u16, Level::Low, 3000u16);

const fn pulseCode(level1: Level, length1: u16, level2: Level, length2: u16) -> u32 {
    let level1 = (level_bit(level1)) | (length1 as u32 & 0b111_1111_1111_1111);
    let level2 = (level_bit(level2)) | (length2 as u32 & 0b111_1111_1111_1111);
    level1 | (level2 << 16)
}

const fn level_bit(level: Level) -> u32 {
    match level {
        Level::Low => 0u32,
        Level::High => 1u32 << 15,
    }
}

// Convert RGB8 color to RMT pulse codes (24 pulses: G-R-B order)
fn rgb_to_rmt_pulses(color: RGB8) -> [u32; BITS_PER_LED] {
    let mut pulses = [0u32; BITS_PER_LED];
    let bytes = [color.g, color.r, color.b]; // WS2812 uses GRB order

    for (byte_idx, &byte_val) in bytes.iter().enumerate() {
        for bit_idx in 0..8 {
            let pulse_idx = byte_idx * 8 + bit_idx;
            let bit_set = (byte_val & (0x80 >> bit_idx)) != 0;
            pulses[pulse_idx] = if bit_set { PULSE_ONE } else { PULSE_ZERO };
        }
    }

    pulses
}

// Generate rainbow pattern for LED buffer
fn generate_rainbow_pattern(buffer: &mut [RGB8; NUM_LEDS], frame_offset: u8) {
    let mut hsv = smart_leds::hsv::Hsv {
        hue: 0,
        sat: 255,
        val: 32,
    };

    for (i, led) in buffer.iter_mut().enumerate() {
        hsv.hue = (((i as u32 * 255 / NUM_LEDS as u32) + frame_offset as u32) % 255) as u8;
        *led = hsv2rgb(hsv);
    }
}

// LED timing constants for WS2812/SK6812
const SK68XX_CODE_PERIOD: u32 = 1250; // 800kHz
const SK68XX_T0H_NS: u32 = 400;
const SK68XX_T0L_NS: u32 = SK68XX_CODE_PERIOD - SK68XX_T0H_NS;
const SK68XX_T1H_NS: u32 = 850;
const SK68XX_T1L_NS: u32 = SK68XX_CODE_PERIOD - SK68XX_T1H_NS;

const SK68XX_LATCH_NS: u32 = 50_000;

fn create_rmt_config() -> TxChannelConfig {
    TxChannelConfig::default()
        .with_clk_divider(1)
        .with_idle_output_level(Level::Low)
        .with_carrier_modulation(false)
        .with_idle_output(true)
        .with_memsize(4) // Use all 4 memory blocks - 192 words
}

// RMT interrupt handler - this is where the magic happens
extern "C" fn rmt_interrupt_handler() {
    unsafe {
        if !INTERRUPT_ACTIVE {
            return;
        }

        let rmt = esp_hal::peripherals::RMT::regs();
        let rmt_start = (esp_hal::peripherals::RMT::ptr() as usize + 0x400) as *mut u32;
        // Read the current hardware read position for debugging
        let hw_pos_start = rmt.ch_tx_status(0).read().mem_raddr_ex().bits();

        let is_halfway = hw_pos_start > HALF_BUFFER_SIZE as u16;

        let is_threshold = rmt.int_raw().read().ch_tx_thr_event(0).bit();

        // We only expect threshold interrupts in this approach
        if !is_threshold {
            defmt::info!("Unexpected interrupt - no threshold event!");
            return;
        }

        // Clear the threshold interrupt
        rmt.int_clr().write(|w| w.ch_tx_thr_event(0).set_bit());

        // Determine what happened based on our expectation and reconfigure for next interrupt
        if is_halfway {
            // We were expecting halfway, so this threshold interrupt = halfway
            // defmt::info!("HALFWAY interrupt fired! HW read pos: {}", hw_read_pos);
            // Reconfigure for end-of-buffer detection
            rmt.ch_tx_lim(0)
                .modify(|_, w| unsafe { w.tx_lim().bits(0) });
        } else {
            // We were expecting end-of-buffer, so this threshold interrupt = end of buffer
            // defmt::info!("BUFFER END interrupt fired! HW read pos: {}", hw_read_pos);
            // Reconfigure for halfway detection
            rmt.ch_tx_lim(0)
                .modify(|_, w| unsafe { w.tx_lim().bits(HALF_BUFFER_SIZE as u16) });
        };
        // Clear any error interrupts
        if rmt.int_raw().read().ch_tx_err(0).bit() {
            rmt.int_clr().write(|w| w.ch_tx_err(0).set_bit());
        }

        // rmt_start.write_volatile(PULSE_ONE);
        // for i in 1..(RMT_BUFFER.len() - BITS_PER_LED) {
        //     rmt_start.add(i).write_volatile(PULSE_ZERO);
        // }
        // if (FRAME_COUNTER < 100) {
        //     for i in (RMT_BUFFER.len() - BITS_PER_LED)..RMT_BUFFER.len() {
        //         rmt_start.add(i).write_volatile(PULSE_ONE)
        //     }
        // } else {
        //     for i in (RMT_BUFFER.len() - BITS_PER_LED)..RMT_BUFFER.len() {
        //         rmt_start.add(i).write_volatile(PULSE_LATCH)
        //     }
        //     FRAME_COUNTER = 0;
        // }
        // FRAME_COUNTER += 1;

        let buffer_base = if is_halfway {
            // Fill the first half while hardware transmits second half
            rmt_start
        } else {
            // Fill the second half while hardware transmits first half
            rmt_start.add(HALF_BUFFER_SIZE)
        };

        for i in 0..HALF_BUFFER_LEDS {
            let led_base = buffer_base.add(i * BITS_PER_LED);

            if LED_COUNTER >= NUM_LEDS {
                // End of LED strip - fill with latch/reset pulses
                for j in 0..BITS_PER_LED {
                    led_base.add(j).write_volatile(PULSE_LATCH);
                }
                LED_COUNTER = 0;
                FRAME_COUNTER += 1;

                // Generate new rainbow pattern for next frame
                generate_rainbow_pattern(&mut LED_DATA_BUFFER, FRAME_COUNTER as u8);
            } else {
                // Get RGB color from LED data buffer
                let color = LED_DATA_BUFFER[LED_COUNTER];
                let pulses = rgb_to_rmt_pulses(color);

                // Write the pulses to RMT buffer
                for j in 0..BITS_PER_LED {
                    led_base.add(j).write_volatile(pulses[j]);
                }

                LED_COUNTER += 1;
            }
        }

        let hw_pos_end = rmt.ch_tx_status(0).read().mem_raddr_ex().bits();

        RMT_STATS_SUM += (hw_pos_end as i32) - (hw_pos_start as i32);
        RMT_STATS_COUNT += 1;
    }
}

pub fn rmt_interrupt_demo<'d, O>(
    mut rmt: esp_hal::rmt::Rmt<'d, Blocking>,
    pin: O,
) -> Result<(), RmtError>
where
    O: PeripheralOutput<'d>,
{
    // Set up the interrupt handler FIRST
    let handler = InterruptHandler::new(rmt_interrupt_handler, Priority::Priority1);
    rmt.set_interrupt_handler(handler);

    // Configure the RMT channel
    let config = create_rmt_config();
    let channel = rmt.channel0.configure_tx(pin, config)?;

    // Get timing parameters
    let src_clock = Clocks::get().apb_clock.as_mhz();

    // We need to explicitly enable threshold and end interrupts on the channel
    // The set_interrupt_handler only registers our handler, but doesn't enable events

    // Enable threshold and end interrupts directly on the RMT registers
    let rmt_regs = esp_hal::peripherals::RMT::regs();

    // CRITICAL: Set the transmission limit to half buffer size for threshold interrupt
    // This tells the hardware when to fire the threshold interrupt
    rmt_regs
        .ch_tx_lim(0)
        .modify(|_, w| unsafe { w.tx_lim().bits(HALF_BUFFER_SIZE as u16) });

    rmt_regs.int_ena().modify(|_, w| {
        w.ch_tx_thr_event(0).set_bit() // Enable threshold interrupt for channel 0
    });

    // Start continuous transmission with interrupts properly enabled!
    // The RMT hardware will now automatically call rmt_interrupt_handler
    // when threshold events occur (we dynamically change the threshold)

    unsafe {
        // Fill RMT buffer with initial latch pulses
        for i in 0..RMT_BUFFER.len() {
            RMT_BUFFER[i] = PULSE_LATCH;
        }

        INTERRUPT_ACTIVE = true;
    }

    let _transaction = channel.transmit_continuously(unsafe { &RMT_BUFFER })?;

    // Now the system runs entirely on hardware interrupts!
    // The RMT hardware continuously transmits while interrupts
    // seamlessly update the buffer halves with new animation data.
    // There are NO gaps in transmission - LEDs will never latch.

    loop {
        // In a real application, you could:
        // - Handle other tasks
        // - Check for errors
        // - Implement graceful shutdown
        // - Update animation parameters

        // For this demo, just sleep and let interrupts handle everything
        esp_hal::delay::Delay::new().delay_millis(1000);

        // Optional: Check frame counter and hardware read position to show it's working
        unsafe {
            let rmt_regs = esp_hal::peripherals::RMT::regs();

            let avg_bytes_per_frame = RMT_STATS_SUM / RMT_STATS_COUNT;
            RMT_STATS_SUM = 0;
            RMT_STATS_COUNT = 0;

            defmt::info!(
                "Frame: {}; LED: {}; avg_bytes_per_frame: {}",
                FRAME_COUNTER,
                LED_COUNTER,
                avg_bytes_per_frame
            );
        }
    }
}
