use crate::demo_program::create_demo_scene;
use crate::scene::SceneRuntime;
/// Test scene - shared between ESP32 and host
/// This defines the standard test program and scene configuration
use crate::test_engine::{Fixed, RuntimeOptions};

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 16;
pub const LED_COUNT: usize = 128;

/// Scene data containing runtime state (backward compatibility wrapper)
pub struct SceneData {
    runtime: SceneRuntime,
}

impl SceneData {
    /// Create a new scene with the standard demo configuration
    pub fn new() -> Self {
        let config = create_demo_scene(WIDTH, HEIGHT, LED_COUNT);
        let options = RuntimeOptions::new(WIDTH, HEIGHT);
        let runtime = SceneRuntime::new(config, options).expect("Valid scene config");

        SceneData { runtime }
    }

    // Expose runtime fields for visualizer
    pub fn pipeline(&self) -> &crate::test_engine::FxPipeline {
        &self.runtime.pipeline
    }

    pub fn pipeline_mut(&mut self) -> &mut crate::test_engine::FxPipeline {
        &mut self.runtime.pipeline
    }

    pub fn mapping(&self) -> &crate::test_engine::LedMapping {
        &self.runtime.mapping
    }

    pub fn led_output(&self) -> &[u8] {
        &self.runtime.led_output
    }

    pub fn led_output_mut(&mut self) -> &mut [u8] {
        &mut self.runtime.led_output
    }
}

/// Render a single frame of the test scene
#[inline(never)]
pub fn render_test_scene(scene: &mut SceneData, time: Fixed) {
    scene.runtime.render(time, 1).expect("Render failed");
}
