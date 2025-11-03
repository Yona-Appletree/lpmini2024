/// Test scene - shared between ESP32 and host
/// This defines the standard test program and scene configuration
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use crate::test_engine::{
    fixed_from_f32, render_frame, Fixed, LedMapping, LoadSource, OpCode, Palette,
};

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 16;
pub const LED_COUNT: usize = 128;
pub const BUFFER_SIZE: usize = WIDTH * HEIGHT;

/// Scene data containing all buffers and configuration
pub struct SceneData {
    pub greyscale_buffer: Vec<Fixed>,
    pub input_buffer: Vec<Fixed>,
    pub rgb_2d_buffer: Vec<u8>,
    pub led_output: Vec<u8>,
    pub palette: Palette,
    pub mapping: LedMapping,
    pub program: Vec<OpCode>,
}

impl SceneData {
    /// Create a new scene with the standard test configuration
    pub fn new() -> Self {
        // Create palette and mapping
        let palette = Palette::rainbow();
        let mapping = LedMapping::circular_panel_7ring(WIDTH, HEIGHT);

        // Test program: perlin noise with 3 octaves, zoom, and cosine smoothing
        let program = vec![
            OpCode::Load(LoadSource::XNorm),   // Normalized x (0..1)
            OpCode::Push(fixed_from_f32(0.3)), // Zoom factor
            OpCode::Mul,                       // Scale x down
            OpCode::Load(LoadSource::YNorm),   // Normalized y (0..1)
            OpCode::Push(fixed_from_f32(0.3)), // Zoom factor
            OpCode::Mul,                       // Scale y down
            OpCode::Load(LoadSource::Time),    // Time (scrolls the z-axis)
            OpCode::Perlin3(3),                // Generate perlin noise with 3 octaves
            OpCode::Cos,                       // Apply cosine (outputs 0..1)
            OpCode::Return,
        ];

        SceneData {
            greyscale_buffer: vec![0; BUFFER_SIZE],
            input_buffer: vec![0; BUFFER_SIZE],
            rgb_2d_buffer: vec![0u8; BUFFER_SIZE * 3],
            led_output: vec![0u8; LED_COUNT * 3],
            palette,
            mapping,
            program,
        }
    }
}

/// Render a single frame of the test scene
#[inline(never)]
pub fn render_test_scene(scene: &mut SceneData, time: Fixed) {
    render_frame(
        &mut scene.greyscale_buffer,
        &scene.input_buffer,
        &mut scene.rgb_2d_buffer,
        &mut scene.led_output,
        &scene.program,
        &scene.palette,
        &scene.mapping,
        WIDTH,
        HEIGHT,
        time,
    );
}
