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

use esp_hal::clock::Clocks;
use esp_hal::gpio::interconnect::PeripheralOutput;
use esp_hal::gpio::Level;
use esp_hal::interrupt::{self, InterruptHandler, Priority};
use esp_hal::rmt::{
    Error as RmtError, PulseCode, TxChannel, TxChannelConfig, TxChannelCreator, TxChannelInternal,
};
use esp_hal::Blocking;

// Configuration constants
const NUM_LEDS: usize = 64; // Total number of LEDs in the strip

// Buffer size for 8 LEDs worth of data (double buffered)
// Using memsize(4) = 192 words. 8 LEDs = 192 words exactly, 4 LEDs per half
const BUFFER_LEDS: usize = 8;
const RMT_RAM_ONE_LED: usize = 3 * 8; // RGB * 8 bits
const HALF_BUFFER_SIZE: usize = (BUFFER_LEDS * RMT_RAM_ONE_LED) / 2; // Half buffer for double buffering
const FULL_BUFFER_SIZE: usize = BUFFER_LEDS * RMT_RAM_ONE_LED; // 192 words total

// Global state for interrupt handling
static mut RMT_BUFFER: [u32; FULL_BUFFER_SIZE] = [0; FULL_BUFFER_SIZE];
static mut FRAME_COUNTER: usize = 0;
static mut LED_COUNTER: usize = 0; // Track current LED position in the strip
static mut INTERRUPT_ACTIVE: bool = false;

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

// Convert a single RGB color to RMT pulse codes
#[inline(always)]
fn rgb_to_pulses(r: u8, g: u8, b: u8, buffer: &mut [u32], start_idx: usize) {
    let mut idx = start_idx;

    // Green first (WS2812 order is GRB)
    for bit_pos in [128, 64, 32, 16, 8, 4, 2, 1] {
        buffer[idx] = if g & bit_pos != 0 {
            PULSE_ONE
        } else {
            PULSE_ZERO
        };
        idx += 1;
    }

    // Red
    for bit_pos in [128, 64, 32, 16, 8, 4, 2, 1] {
        buffer[idx] = if r & bit_pos != 0 {
            PULSE_ONE
        } else {
            PULSE_ZERO
        };
        idx += 1;
    }

    // Blue
    for bit_pos in [128, 64, 32, 16, 8, 4, 2, 1] {
        buffer[idx] = if b & bit_pos != 0 {
            PULSE_ONE
        } else {
            PULSE_ZERO
        };
        idx += 1;
    }
}

// Generate animated RGB values
#[inline(always)]
fn generate_animated_rgb(led_index: usize, frame_counter: usize) -> (u8, u8, u8) {
    let r = if led_index == 0 { 255 } else { 0 };
    let g = 0;
    let b = 0;
    (r, g, b)
}

// Fill half of the buffer with animated LED data or latch signals
// Returns the updated LED counter after filling
#[inline(always)]
fn fill_half_buffer(
    buffer: &mut [u32],
    half_start: usize,
    frame_counter: usize,
    led_counter: &mut usize,
) {
    let leds_per_half = BUFFER_LEDS / 2;
    for led_idx in 0..leds_per_half {
        let led_buffer_offset = half_start + (led_idx * RMT_RAM_ONE_LED);

        if *led_counter >= NUM_LEDS {
            for i in 0..RMT_RAM_ONE_LED {
                buffer[led_buffer_offset + i] = PULSE_LATCH;
            }
            *led_counter = 0;
        } else {
            // Normal LED data
            rgb_to_pulses(127, 0, 0, buffer, led_buffer_offset);
            *led_counter += 1;
        }
    }
}

// Fill half buffer directly to RMT hardware memory (optimized for ISR)
#[inline(always)]
unsafe fn fill_half_buffer_direct(
    ram_ptr: *mut u32,
    frame_counter: usize,
    led_counter: &mut usize,
) {
    let leds_per_half = BUFFER_LEDS / 2;
    for led_idx in 0..leds_per_half {
        if *led_counter >= NUM_LEDS {
            // Insert latch signal - fill this LED slot with latch
            // Manual unrolling for maximum performance
            let base_offset = led_idx * RMT_RAM_ONE_LED;
            ram_ptr.add(base_offset + 0).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 1).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 2).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 3).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 4).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 5).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 6).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 7).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 8).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 9).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 10).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 11).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 12).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 13).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 14).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 15).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 16).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 17).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 18).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 19).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 20).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 21).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 22).write_volatile(PULSE_LATCH);
            ram_ptr.add(base_offset + 23).write_volatile(PULSE_LATCH);
            *led_counter = 0;
        } else {
            // Normal LED data - write directly to RMT memory
            rgb_to_pulses_direct(16, 0, 0, ram_ptr.add(led_idx * RMT_RAM_ONE_LED));
            *led_counter += 1;
        }
    }
}

// Convert RGB to RMT pulses and write directly to memory pointer
// Optimized with manual loop unrolling for maximum ISR performance
#[inline(always)]
unsafe fn rgb_to_pulses_direct(r: u8, g: u8, b: u8, ram_ptr: *mut u32) {
    // Manual unrolling for Green (GRB order for WS2812)
    ram_ptr
        .add(0)
        .write_volatile(if g & 128 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(1)
        .write_volatile(if g & 64 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(2)
        .write_volatile(if g & 32 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(3)
        .write_volatile(if g & 16 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(4)
        .write_volatile(if g & 8 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(5)
        .write_volatile(if g & 4 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(6)
        .write_volatile(if g & 2 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(7)
        .write_volatile(if g & 1 != 0 { PULSE_ONE } else { PULSE_ZERO });

    // Manual unrolling for Red
    ram_ptr
        .add(8)
        .write_volatile(if r & 128 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(9)
        .write_volatile(if r & 64 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(10)
        .write_volatile(if r & 32 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(11)
        .write_volatile(if r & 16 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(12)
        .write_volatile(if r & 8 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(13)
        .write_volatile(if r & 4 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(14)
        .write_volatile(if r & 2 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(15)
        .write_volatile(if r & 1 != 0 { PULSE_ONE } else { PULSE_ZERO });

    // Manual unrolling for Blue
    ram_ptr
        .add(16)
        .write_volatile(if b & 128 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(17)
        .write_volatile(if b & 64 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(18)
        .write_volatile(if b & 32 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(19)
        .write_volatile(if b & 16 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(20)
        .write_volatile(if b & 8 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(21)
        .write_volatile(if b & 4 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(22)
        .write_volatile(if b & 2 != 0 { PULSE_ONE } else { PULSE_ZERO });
    ram_ptr
        .add(23)
        .write_volatile(if b & 1 != 0 { PULSE_ONE } else { PULSE_ZERO });
}

// RMT interrupt handler - this is where the magic happens
extern "C" fn rmt_interrupt_handler() {
    unsafe {
        if !INTERRUPT_ACTIVE {
            return;
        }

        let rmt = esp_hal::peripherals::RMT::regs();

        // Check if this is a threshold interrupt for channel 0
        if rmt.int_raw().read().ch_tx_thr_event(0).bit() {
            // Clear the threshold interrupt immediately
            rmt.int_clr().write(|w| w.ch_tx_thr_event(0).set_bit());

            // Get current frame counter and increment for animation
            FRAME_COUNTER = FRAME_COUNTER.wrapping_add(1);
            let frame = FRAME_COUNTER;

            // The threshold interrupt fires when we've transmitted the first half
            // Hardware is now transmitting the second half, so we can safely
            // refill the first half that was just transmitted

            // Write directly to RMT hardware memory - NO double buffering!
            let ram_base = (esp_hal::peripherals::RMT::ptr() as usize + 0x400) as *mut u32;
            fill_half_buffer_direct(
                ram_base, // Write directly to first half of RMT memory
                frame,
                &mut LED_COUNTER,
            );
        }

        // Handle end interrupt (transmission completed one full cycle)
        if rmt.int_raw().read().ch_tx_end(0).bit() {
            // Clear end interrupt
            rmt.int_clr().write(|w| w.ch_tx_end(0).set_bit());

            // When transmission ends, we just finished the second half
            // Hardware is about to wrap and start from the first half again
            // So we need to refill the second half that was just transmitted
            let frame = FRAME_COUNTER;

            // Write directly to RMT hardware memory - NO double buffering!
            let ram_base = (esp_hal::peripherals::RMT::ptr() as usize + 0x400) as *mut u32;
            let second_half_start = ram_base.add(HALF_BUFFER_SIZE);
            fill_half_buffer_direct(
                second_half_start, // Write directly to second half of RMT memory
                frame,
                &mut LED_COUNTER,
            );
        }

        // Clear any error interrupts
        if rmt.int_raw().read().ch_tx_err(0).bit() {
            rmt.int_clr().write(|w| w.ch_tx_err(0).set_bit());
        }
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

    // Initialize LED counter and fill initial buffer with both halves for streaming start
    unsafe {
        // Fill first half (will show LEDs 0-3 initially)
        LED_COUNTER = 0;
        fill_half_buffer(&mut RMT_BUFFER, 0, 0, &mut LED_COUNTER);
        // Fill second half (will show LEDs 4-7 initially)
        fill_half_buffer(&mut RMT_BUFFER, HALF_BUFFER_SIZE, 0, &mut LED_COUNTER);

        // LED_COUNTER is now automatically set to the correct next position
        // Note: Using full 192-word buffer, no end marker needed
    }

    // Enable interrupts BEFORE starting transmission
    unsafe {
        INTERRUPT_ACTIVE = true;
    }

    // We need to explicitly enable threshold and end interrupts on the channel
    // The set_interrupt_handler only registers our handler, but doesn't enable events

    // Enable threshold and end interrupts directly on the RMT registers
    let rmt_regs = esp_hal::peripherals::RMT::regs();
    rmt_regs.int_ena().modify(|_, w| {
        w.ch_tx_thr_event(0).set_bit(); // Enable threshold interrupt for channel 0
        w.ch_tx_end(0).set_bit() // Enable end interrupt for channel 0
    });

    // Start continuous transmission with interrupts properly enabled!
    // The RMT hardware will now automatically call rmt_interrupt_handler
    // when threshold and end events occur

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

        // Optional: Check frame counter to show it's working
        unsafe {
            defmt::info!("Frame: {}; LED: {}", FRAME_COUNTER, LED_COUNTER);
        }
    }
}
