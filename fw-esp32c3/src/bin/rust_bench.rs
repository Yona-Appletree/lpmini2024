#![no_std]
#![no_main]

extern crate alloc;

use defmt::info;
use embassy_time::Instant;
use esp_hal::clock::{Clock, CpuClock};
use esp_hal::timer::systimer::SystemTimer;
use panic_rtt_target as _;

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

const MATRIX_WIDTH: usize = 16;
const MATRIX_HEIGHT: usize = 16;
const BYTES_PER_PIXEL: usize = 3; // RGB
const BUFFER_SIZE: usize = MATRIX_WIDTH * MATRIX_HEIGHT * BYTES_PER_PIXEL;

#[esp_hal_embassy::main]
async fn main(_spawner: embassy_executor::Spawner) {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Allocate heap
    esp_alloc::heap_allocator!(size: 64 * 1024);
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    info!("===========================================");
    info!("Pure Rust LED Matrix Rendering Benchmark");
    info!("===========================================");
    info!("Target: ESP32-C3 @ {} MHz", CpuClock::max().mhz());
    info!(
        "Matrix: {}x{} RGB ({} bytes)",
        MATRIX_WIDTH, MATRIX_HEIGHT, BUFFER_SIZE
    );
    info!("");

    // Create buffer for the LED matrix
    let mut buffer = [0u8; BUFFER_SIZE];

    info!("Starting benchmark...");
    info!("");

    // Benchmark loop
    let mut frame_count = 0u32;
    let mut total_us = 0u64;
    let mut min_us = u64::MAX;
    let mut max_us = 0u64;
    let start_time = Instant::now();

    loop {
        let frame_start = Instant::now();

        // Render the frame - this is what we're benchmarking
        render_frame(&mut buffer, frame_count as f32);

        let frame_elapsed = frame_start.elapsed();
        let frame_us = frame_elapsed.as_micros();

        total_us += frame_us;
        frame_count += 1;
        min_us = min_us.min(frame_us);
        max_us = max_us.max(frame_us);

        // Log performance stats every 10 frames
        if frame_count % 10 == 0 {
            let avg_us = total_us / frame_count as u64;
            let fps = if avg_us > 0 { 1_000_000 / avg_us } else { 0 };
            info!(
                "Frame {}: {}µs | avg: {}µs | ~{} FPS",
                frame_count, frame_us, avg_us, fps
            );
        }

        // Stop after 100 frames for benchmarking
        if frame_count >= 100 {
            info!("");
            info!("===========================================");
            info!("Benchmark Complete!");
            info!("===========================================");
            let total_time = start_time.elapsed().as_millis();
            let avg_us = total_us / frame_count as u64;
            let avg_fps = if avg_us > 0 { 1_000_000 / avg_us } else { 0 };

            info!("Total frames:    {}", frame_count);
            info!("Total time:      {}ms", total_time);
            info!("Min frame time:  {}µs", min_us);
            info!("Max frame time:  {}µs", max_us);
            info!("Avg frame time:  {}µs", avg_us);
            info!("Average FPS:     ~{} FPS", avg_fps);
            info!("Buffer size:     {} bytes", BUFFER_SIZE);
            info!("Pixels/frame:    {}", MATRIX_WIDTH * MATRIX_HEIGHT);

            // Calculate some useful metrics
            let ns_per_pixel = (avg_us * 1000) / (MATRIX_WIDTH * MATRIX_HEIGHT) as u64;
            info!("Time/pixel:      {}ns", ns_per_pixel);

            // Memory bandwidth estimate
            let bytes_per_sec = (BUFFER_SIZE as u64 * 1_000_000) / avg_us;
            info!("Write bandwidth: {} KB/s", bytes_per_sec / 1024);

            info!("===========================================");
            info!("");
            info!("Note: This is PURE RUST performance.");
            info!("Scripting engines will be slower.");
            info!("Typical overhead: 5-50x depending on engine.");
            info!("");
            break;
        }
    }

    info!("Benchmark ended, entering idle loop");
    loop {
        // Keep the program alive
    }
}

/// Simple perlin-like noise function
/// This uses sine/cosine for smooth animated patterns
#[inline(always)]
fn perlin3(x: f32, y: f32, z: f32) -> f32 {
    // Simple smooth noise using sine functions
    // This gives us something computationally similar to perlin
    let freq1 = 1.0;
    let freq2 = 2.0;
    let freq3 = 4.0;

    let n1 = libm::sinf(x * freq1 + z) * libm::cosf(y * freq1);
    let n2 = libm::sinf(x * freq2 - z) * libm::cosf(y * freq2) * 0.5;
    let n3 = libm::sinf(x * freq3 + y + z) * 0.25;

    (n1 + n2 + n3) / 1.75
}

/// Render function that fills the buffer with perlin noise
/// This simulates what a script would do
#[inline(never)] // Don't inline so we can see it in profiling
fn render_frame(buffer: &mut [u8], time: f32) {
    const WIDTH: f32 = MATRIX_WIDTH as f32;
    const HEIGHT: f32 = MATRIX_HEIGHT as f32;

    for y in 0..MATRIX_HEIGHT {
        for x in 0..MATRIX_WIDTH {
            // Normalize coordinates for perlin noise
            let nx = (x as f32 / WIDTH) * 4.0;
            let ny = (y as f32 / HEIGHT) * 4.0;
            let nz = time * 0.001;

            // Get perlin noise value (-1 to 1, typically)
            let noise = perlin3(nx, ny, nz);

            // Convert to 0-255 range
            let value = ((noise + 1.0) * 0.5 * 255.0) as u8;

            // Calculate buffer index (RGB = 3 bytes per pixel)
            let idx = (y * MATRIX_WIDTH + x) * BYTES_PER_PIXEL;

            // Set RGB values (grayscale for this demo)
            buffer[idx] = value; // R
            buffer[idx + 1] = value; // G
            buffer[idx + 2] = value; // B
        }
    }
}
