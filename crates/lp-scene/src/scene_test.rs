#[cfg(test)]
mod tests {
    use lp_math::dec32::Dec32;

    use crate::nodes::lfo::lfo_input::LfoInput;
    use crate::scene::LpScene;
    use crate::scene_config::LpSceneConfig;

    #[test]
    fn test_scene_with_lfo() {
        // Create a scene config with one LFO node
        let mut config = LpSceneConfig::new();
        config.add_lfo_node(
            "lfo1",
            LfoInput::new(
                1000,        // period_ms
                Dec32::ZERO, // min
                Dec32::ONE,  // max
            ),
        );

        // Create a scene from the config
        let mut scene = LpScene::from_config(&config).unwrap();

        // Run a couple of frames
        // Helper to extract Dec32 from LpValueRef
        fn extract_dec32(value_ref: lp_data::kind::value::LpValueRef) -> Dec32 {
            match value_ref {
                lp_data::kind::value::LpValueRef::Dec32(fixed_ref) => {
                    // SAFETY: We know this is a Dec32 because it's in the Dec32 variant
                    unsafe {
                        *(fixed_ref as *const dyn lp_data::kind::value::LpValue as *const Dec32)
                    }
                }
                _ => panic!("Expected Dec32 output"),
            }
        }

        // Frame 0: should be at phase 0.0, sine output is 0 (in [-1,1]), maps to 0.5 in [0,1]
        scene.update_frame(0).unwrap();
        let output0 = scene.get_node_output("lfo1", "output").unwrap();
        let output_f32_0 = extract_dec32(output0).to_f32();
        // At time 0, sine is 0, which maps to 0.5 in [0,1] range
        assert!(
            (output_f32_0 - 0.5).abs() < 0.1,
            "Expected output near 0.5 at frame 0, got {}",
            output_f32_0
        );

        // Frame 1: at 250ms with 1000ms period, phase is 0.25, sine should be near 1.0
        scene.update_frame(250).unwrap();
        let output1 = scene.get_node_output("lfo1", "output").unwrap();
        let output_f32_1 = extract_dec32(output1).to_f32();
        // At 250ms with 1000ms period, phase is 0.25, sine should be near 1.0
        assert!(
            (output_f32_1 - 1.0).abs() < 0.2,
            "Expected output near 1.0 at 250ms, got {}",
            output_f32_1
        );

        // Frame 2: at 500ms, phase is 0.5, sine is 0 (in [-1,1]), maps to 0.5 in [0,1]
        scene.update_frame(500).unwrap();
        let output2 = scene.get_node_output("lfo1", "output").unwrap();
        let output_f32_2 = extract_dec32(output2).to_f32();
        // At 500ms, phase is 0.5, sine is 0, which maps to 0.5 in [0,1] range
        assert!(
            (output_f32_2 - 0.5).abs() < 0.1,
            "Expected output near 0.5 at 500ms, got {}",
            output_f32_2
        );
    }
}
