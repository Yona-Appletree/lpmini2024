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

// Buffer size for 2 LEDs worth of data (double buffered)
// Using memsize(1) = 48 words. 2 LEDs = 48 words exactly, 1 LED per half
const BUFFER_LEDS: usize = 2;
const RMT_RAM_ONE_LED: usize = 3 * 8; // RGB * 8 bits
const HALF_BUFFER_SIZE: usize = (BUFFER_LEDS * RMT_RAM_ONE_LED) / 2; // Half buffer for double buffering
const FULL_BUFFER_SIZE: usize = BUFFER_LEDS * RMT_RAM_ONE_LED; // 48 words total

// Global state for interrupt handling
static mut RMT_BUFFER: [u32; FULL_BUFFER_SIZE] = [0; FULL_BUFFER_SIZE];
static mut FRAME_COUNTER: usize = 0;
static mut LED_COUNTER: usize = 0; // Track current LED position in the strip
static mut INTERRUPT_ACTIVE: bool = false;
static mut PULSE_CODES: (u32, u32) = (0, 0);

// LED timing constants for WS2812/SK6812
const SK68XX_CODE_PERIOD: u32 = 1250; // 800kHz
const SK68XX_T0H_NS: u32 = 400;
const SK68XX_T0L_NS: u32 = SK68XX_CODE_PERIOD - SK68XX_T0H_NS;
const SK68XX_T1H_NS: u32 = 850;
const SK68XX_T1L_NS: u32 = SK68XX_CODE_PERIOD - SK68XX_T1H_NS;

fn generate_pulse_codes(src_clock: u32) -> (u32, u32) {
    (
        PulseCode::new(
            Level::High,
            ((SK68XX_T0H_NS * src_clock) / 1000) as u16,
            Level::Low,
            ((SK68XX_T0L_NS * src_clock) / 1000) as u16,
        ),
        PulseCode::new(
            Level::High,
            ((SK68XX_T1H_NS * src_clock) / 1000) as u16,
            Level::Low,
            ((SK68XX_T1L_NS * src_clock) / 1000) as u16,
        ),
    )
}

fn create_rmt_config() -> TxChannelConfig {
    TxChannelConfig::default()
        .with_clk_divider(1)
        .with_idle_output_level(Level::Low)
        .with_carrier_modulation(false)
        .with_idle_output(true)
        .with_memsize(1) // Use minimum memory block size - 48 words
}

// Convert a single RGB color to RMT pulse codes
fn rgb_to_pulses(r: u8, g: u8, b: u8, buffer: &mut [u32], start_idx: usize, pulses: (u32, u32)) {
    let mut idx = start_idx;

    // Green first (WS2812 order is GRB)
    for bit_pos in [128, 64, 32, 16, 8, 4, 2, 1] {
        buffer[idx] = if g & bit_pos != 0 { pulses.1 } else { pulses.0 };
        idx += 1;
    }

    // Red
    for bit_pos in [128, 64, 32, 16, 8, 4, 2, 1] {
        buffer[idx] = if r & bit_pos != 0 { pulses.1 } else { pulses.0 };
        idx += 1;
    }

    // Blue
    for bit_pos in [128, 64, 32, 16, 8, 4, 2, 1] {
        buffer[idx] = if b & bit_pos != 0 { pulses.1 } else { pulses.0 };
        idx += 1;
    }
}

// Generate animated RGB values
fn generate_animated_rgb(led_index: usize, frame_counter: usize) -> (u8, u8, u8) {
    let ch_idx = led_index % 3;
    let ch_brightness = 127u8;
    let r = if ch_idx == 0 { ch_brightness } else { 0 };
    let g = if ch_idx == 1 { ch_brightness } else { 0 };
    let b = if ch_idx == 2 { ch_brightness } else { 0 };
    (r, g, b)
}

// Generate latch signal: 50µs low pulse
// At 80MHz clock with 1µs period, 50µs = 50 clock cycles
fn generate_latch_signal() -> u32 {
    // Create a long low pulse (50µs at 80MHz = 4000 clock cycles)
    // RMT can handle up to 32767 clock cycles per pulse
    use esp_hal::gpio::Level;
    use esp_hal::rmt::PulseCode;

    u32::new(Level::Low, 2000, Level::Low, 2000) // 50µs low pulse
}

// Fill half of the buffer with animated LED data or latch signals
fn fill_half_buffer(
    buffer: &mut [u32],
    half_start: usize,
    frame_counter: usize,
    led_offset: usize,
    pulses: (u32, u32),
) {
    let leds_per_half = BUFFER_LEDS / 2;
    for led_idx in 0..leds_per_half {
        let global_led_idx = (led_offset + led_idx) % (NUM_LEDS + 1); // +1 for latch position

        if global_led_idx == NUM_LEDS {
            // Insert latch signal - fill entire half with latch signal
            let latch_signal = generate_latch_signal();
            for i in 0..HALF_BUFFER_SIZE {
                buffer[half_start + i] = latch_signal;
            }
            return;
        } else {
            // Normal LED data
            let (r, g, b) = generate_animated_rgb(global_led_idx, frame_counter);
            let buffer_start = half_start + (led_idx * RMT_RAM_ONE_LED);
            rgb_to_pulses(r, g, b, buffer, buffer_start, pulses);
        }
    }
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

            // Determine which half of the buffer was just transmitted
            // and needs to be refilled
            let pulses = PULSE_CODES;

            // The threshold interrupt fires when we've transmitted the first half
            // Hardware is now transmitting the second half, so we can safely
            // refill the first half that was just transmitted
            fill_half_buffer(
                &mut *addr_of_mut!(RMT_BUFFER),
                0, // First half starts at 0
                frame,
                LED_COUNTER, // Current LED position in strip
                pulses,
            );

            // Advance LED counter by the number of LEDs in half buffer
            LED_COUNTER = (LED_COUNTER + (BUFFER_LEDS / 2)) % (NUM_LEDS + 1);

            // Update the RMT hardware memory with the new first half
            // This is safe because hardware is currently transmitting from second half
            let ram_base = (esp_hal::peripherals::RMT::ptr() as usize + 0x400) as *mut u32;

            for i in 0..HALF_BUFFER_SIZE {
                ram_base.add(i).write_volatile(RMT_BUFFER[i]);
            }
        }

        // Handle end interrupt (transmission completed one full cycle)
        if rmt.int_raw().read().ch_tx_end(0).bit() {
            // Clear end interrupt
            rmt.int_clr().write(|w| w.ch_tx_end(0).set_bit());

            // When transmission ends, we just finished the second half
            // Hardware is about to wrap and start from the first half again
            // So we need to refill the second half that was just transmitted
            let frame = FRAME_COUNTER;

            fill_half_buffer(
                &mut *addr_of_mut!(RMT_BUFFER),
                HALF_BUFFER_SIZE, // Second half starts here
                frame,
                LED_COUNTER, // Current LED position in strip
                PULSE_CODES,
            );

            // Advance LED counter by the number of LEDs in half buffer
            LED_COUNTER = (LED_COUNTER + (BUFFER_LEDS / 2)) % (NUM_LEDS + 1);

            // Update the RMT hardware memory with the new second half
            let ram_base = (esp_hal::peripherals::RMT::ptr() as usize + 0x400) as *mut u32;
            let second_half_start = ram_base.add(HALF_BUFFER_SIZE);

            for i in 0..HALF_BUFFER_SIZE {
                second_half_start
                    .add(i)
                    .write_volatile(RMT_BUFFER[HALF_BUFFER_SIZE + i]);
            }
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
    let pulses = generate_pulse_codes(src_clock);

    // Store pulse codes globally for interrupt handler
    unsafe {
        PULSE_CODES = pulses;
    }

    // Initialize LED counter and fill initial buffer with both halves for streaming start
    unsafe {
        LED_COUNTER = 0; // Start at beginning of LED strip

        // Fill first half (will show LED 0 initially)
        fill_half_buffer(&mut RMT_BUFFER, 0, 0, 0, pulses);
        // Fill second half (will show LED 1 initially)
        fill_half_buffer(
            &mut RMT_BUFFER,
            HALF_BUFFER_SIZE,
            0,
            1, // Start with LED 1 for second half
            pulses,
        );

        // Set LED counter to track that we've filled positions 0 and 1
        LED_COUNTER = 2;
        // Note: Using full 48-word buffer, no end marker needed
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
