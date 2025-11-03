use esp_hal::gpio::interconnect::PeripheralOutput;
use esp_hal::gpio::Level;
use esp_hal::interrupt::{InterruptHandler, Priority};
use esp_hal::rmt::{Error as RmtError, TxChannel, TxChannelConfig, TxChannelCreator};
use esp_hal::Blocking;
use smart_leds::hsv::hsv2rgb;
use smart_leds::RGB8;

// Configuration constants
const MAX_LEDS: usize = 250; // Maximum number of LEDs supported
static mut ACTUAL_NUM_LEDS: usize = 128; // Actual number of LEDs (set at init)

// Buffer size for 8 LEDs worth of data (double buffered)
// Using memsize(4) = 192 words. 8 LEDs = 192 words exactly, 4 LEDs per half
const BUFFER_LEDS: usize = 8;
const HALF_BUFFER_LEDS: usize = BUFFER_LEDS / 2;
const BITS_PER_LED: usize = 3 * 8;
const HALF_BUFFER_SIZE: usize = (BUFFER_LEDS * BITS_PER_LED) / 2;
const BUFFER_SIZE: usize = BUFFER_LEDS * BITS_PER_LED;

// Global state for interrupt handling
static mut RMT_BUFFER: [u32; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut LED_DATA_BUFFER: [RGB8; MAX_LEDS] = [RGB8 { r: 0, g: 0, b: 0 }; MAX_LEDS];
static mut FRAME_COUNTER: usize = 0;
static mut LED_COUNTER: usize = 0; // Track current LED position in the strip
static mut INTERRUPT_ENABLED: bool = false;
static mut RMT_PTR: *mut u32 = 0 as *mut u32;

static mut RMT_STATS_COUNT: i32 = 0;
static mut RMT_STATS_SUM: i32 = 0;
static mut FRAME_COMPLETE: bool = true; // Signal when frame transmission is complete

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

// Latch - WS2812 requires 50us+ low to latch
// At 80MHz: 50us = 4000 ticks, split as 2000+2000
const PULSE_LATCH: u32 = pulseCode(Level::Low, 2000u16, Level::Low, 2000u16);

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

/// Write a byte as RMT pulse-codes for a WS2812/SK6812 LED
#[inline(always)]
unsafe fn write_ws2811_byte(base_ptr: *mut u32, byte_value: u8, byte_offset: usize) {
    let ptr = base_ptr.add(byte_offset * 8);

    // Loop unrolled for performance

    let bit_pulse = |mask: u8| -> u32 {
        if byte_value & mask != 0 {
            PULSE_ONE
        } else {
            PULSE_ZERO
        }
    };

    ptr.add(0).write_volatile(bit_pulse(0x80));
    ptr.add(1).write_volatile(bit_pulse(0x40));
    ptr.add(2).write_volatile(bit_pulse(0x20));
    ptr.add(3).write_volatile(bit_pulse(0x10));
    ptr.add(4).write_volatile(bit_pulse(0x08));
    ptr.add(5).write_volatile(bit_pulse(0x04));
    ptr.add(6).write_volatile(bit_pulse(0x02));
    ptr.add(7).write_volatile(bit_pulse(0x01));
}

// Generate rainbow pattern for LED buffer (test pattern)
fn generate_rainbow_pattern(buffer: &mut [RGB8], num_leds: usize, frame_offset: u8) {
    let mut hsv = smart_leds::hsv::Hsv {
        hue: 0,
        sat: 255,
        val: 5,
    };

    for (i, led) in buffer.iter_mut().enumerate().take(num_leds) {
        hsv.hue = (((i as u32 * 255 / num_leds as u32) + frame_offset as u32) % 255) as u8;
        *led = hsv2rgb(hsv);
    }
}

// Start transmission of current LED buffer
unsafe fn start_transmission() {
    FRAME_COMPLETE = false;
    LED_COUNTER = 0;
    let rmt = esp_hal::peripherals::RMT::regs();

    // Stop current transmission
    rmt.ch_tx_conf0(0)
        .modify(|_, w| w.tx_conti_mode().clear_bit());
    let rmt_base = (esp_hal::peripherals::RMT::ptr() as usize + 0x400) as *mut u32;
    for j in 0..BUFFER_LEDS {
        rmt_base.add(j).write_volatile(0);
    }

    // Init the buffer
    write_half_buffer(true);
    write_half_buffer(false);

    // Clear interrupts
    rmt.int_clr().write(|w| {
        w.ch_tx_end(0).set_bit();
        w.ch_tx_err(0).set_bit();
        w.ch_tx_loop(0).set_bit();
        w.ch_tx_thr_event(0).set_bit()
    });

    // Enable interrupts
    rmt.int_ena().modify(|_, w| {
        w.ch_tx_thr_event(0).set_bit();
        w.ch_tx_end(0).set_bit();
        w.ch_tx_err(0).set_bit();
        w.ch_tx_loop(0).clear_bit()
    });

    // Set the threshold for halfway
    rmt.ch_tx_lim(0).modify(|_, w| {
        w.loop_count_reset().set_bit();
        w.tx_loop_cnt_en().set_bit();
        w.tx_loop_num().bits(0);

        w.tx_lim().bits(HALF_BUFFER_SIZE as u16)
    });

    // Configure
    rmt.ch_tx_conf0(0).modify(|_, w| {
        w.tx_conti_mode().clear_bit(); // single-shot
        w.mem_tx_wrap_en().set_bit(); // wrap
        w.conf_update().set_bit() // update
    });

    // Start
    rmt.ch_tx_conf0(0).modify(|_, w| {
        w.mem_rd_rst().set_bit();
        w.apb_mem_rst().set_bit();
        w.tx_start().set_bit()
    });

    rmt.ch_tx_conf0(0).modify(|_, w| w.conf_update().set_bit());

    // info!("transmission started");
}

// Check if current frame transmission is complete
unsafe fn is_frame_complete() -> bool {
    FRAME_COMPLETE
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
// CRITICAL: Keep this as fast as possible to prevent timing disruption
#[no_mangle]
extern "C" fn rmt_interrupt_handler() {
    unsafe {
        let rmt = esp_hal::peripherals::RMT::regs();

        let int_reg = rmt.int_raw().read();
        let is_end_int = int_reg.ch_tx_end(0).bit();
        let is_thresh_int = int_reg.ch_tx_thr_event(0).bit();
        let is_err_int = int_reg.ch_tx_err(0).bit();

        // Clear interrupts
        rmt.int_clr().write(|w| {
            w.ch_tx_end(0).set_bit();
            w.ch_tx_err(0).set_bit();
            w.ch_tx_loop(0).set_bit();
            w.ch_tx_thr_event(0).set_bit()
        });

        if is_err_int {
            // info!("error interrupt");
        }
        // End interrupt
        else if is_end_int {
            // info!("end interrupt");

            // Signal that we've reached the end of the frame
            // On the next interrupt, this will stop the transmission
            FRAME_COMPLETE = true;
            FRAME_COUNTER += 1;
        } else if is_thresh_int {
            // info!("loop: {}, threshold: {}", is_loop_int, is_thresh_int);

            // Current position of the buffer
            let hw_pos_start = rmt.ch_tx_status(0).read().mem_raddr_ex().bits();

            let is_halfway = hw_pos_start >= HALF_BUFFER_SIZE as u16;
            // info!("pos: {}", hw_pos_start);

            if is_halfway {
                // Set the threshold for end
                rmt.ch_tx_lim(0)
                    .modify(|_, w| w.tx_lim().bits(BUFFER_SIZE as u16));
            } else {
                // Set the threshold for halfway
                rmt.ch_tx_lim(0)
                    .modify(|_, w| w.tx_lim().bits(HALF_BUFFER_SIZE as u16));
            }

            if (write_half_buffer(is_halfway)) {
                // info!("end reached");
            }

            let hw_pos_end = rmt.ch_tx_status(0).read().mem_raddr_ex().bits();

            let bytes_elapsed = (hw_pos_end as i32) - (hw_pos_start as i32);
            RMT_STATS_SUM += bytes_elapsed;
            RMT_STATS_COUNT += 1;
        }
    }
}

unsafe fn write_half_buffer(is_first_half: bool) -> bool {
    let base_ptr = (esp_hal::peripherals::RMT::ptr() as usize + 0x400) as *mut u32;

    let half_ptr = base_ptr.add(if is_first_half { 0 } else { HALF_BUFFER_SIZE });

    for i in 0..HALF_BUFFER_LEDS {
        let led_ptr = half_ptr.add(i * BITS_PER_LED);

        if LED_COUNTER >= ACTUAL_NUM_LEDS {
            // Fill the rest of the buffer segment with zero
            for j in 0..BITS_PER_LED * (HALF_BUFFER_LEDS - i) {
                led_ptr
                    .add(j)
                    .write_volatile(if j == 0 { PULSE_LATCH } else { 0 });
            }

            return true;
        } else {
            // Get RGB color from LED data buffer and write directly to RMT buffer
            let color = LED_DATA_BUFFER[LED_COUNTER];

            // WS2812 uses GRB order
            write_ws2811_byte(led_ptr, color.g, 0); // Green first
            write_ws2811_byte(led_ptr, color.r, 1); // Red second
            write_ws2811_byte(led_ptr, color.b, 2); // Blue third

            LED_COUNTER += 1;
            // info!("led {}", LED_COUNTER);
        }
    }

    return false;
}

/// Initialize the WS2811/WS2812 LED driver
///
/// # Arguments
/// * `rmt` - RMT peripheral
/// * `pin` - GPIO pin for LED data output
/// * `num_leds` - Number of LEDs in the strip (max 250)
///
/// # Returns
/// Transaction handle that must be kept alive
pub fn rmt_ws2811_init<'d, O>(
    mut rmt: esp_hal::rmt::Rmt<'d, Blocking>,
    pin: O,
    num_leds: usize,
) -> Result<impl core::marker::Sized + 'd, RmtError>
where
    O: PeripheralOutput<'d>,
{
    unsafe {
        ACTUAL_NUM_LEDS = num_leds.min(MAX_LEDS);
    }

    // Set up the interrupt handler with max priority
    let handler = InterruptHandler::new(rmt_interrupt_handler, Priority::max());
    rmt.set_interrupt_handler(handler);

    // Configure the RMT channel
    let config = create_rmt_config();
    let channel = rmt.channel0.configure_tx(pin, config)?;

    // HACK: If we don't call transmit_continuously, things work, but the debug output stops
    //       working.
    //
    let dummy_buffer = [pulseCode(Level::Low, 1, Level::Low, 1)];
    let transaction = channel.transmit_continuously(&dummy_buffer)?;

    Ok(transaction)
}

/// Write LED data and start transmission
///
/// # Arguments
/// * `led_data` - RGB data for LEDs (must be at least num_leds length)
pub fn rmt_ws2811_write(led_data: &[RGB8]) {
    unsafe {
        // Copy LED data into internal buffer
        let num_to_copy = led_data.len().min(ACTUAL_NUM_LEDS);
        LED_DATA_BUFFER[..num_to_copy].copy_from_slice(&led_data[..num_to_copy]);

        // Start transmission
        start_transmission();
    }
}

/// Wait for the current frame transmission to complete
pub fn rmt_ws2811_wait_complete() {
    unsafe {
        while !is_frame_complete() {
            // Small delay to avoid busy waiting
            esp_hal::delay::Delay::new().delay_micros(10);
        }
    }
}
