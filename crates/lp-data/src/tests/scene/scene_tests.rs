//! Tests for scene graph traversal.
use crate::kind::{record::record_dyn::RecordShapeDyn, value::LpValueBox};
use crate::tests::scene::{
    lfo::{LfoConfig, LfoNode, LfoWaveform},
    print_lp_value::print_lp_value_to_string,
};
use core::ptr::NonNull;

extern crate alloc;
use crate::kind::record::record_value::RecordValue;
use crate::kind::record::RecordValueDyn;
use lp_math::fixed::ToFixed;
use lp_pool::{enter_global_alloc_allowance, lp_box_dyn, LpMemoryPool, LpString};

fn setup_pool() -> LpMemoryPool {
    let mut memory = [0u8; 16384];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() }
}

/// Build a demo scene with a module containing nodes.
/// Must be called within a pool context.
fn build_demo_scene() -> Result<RecordValueDyn, lp_pool::AllocError> {
    // Create a simple scene with a module containing nodes
    let mut nodes = RecordValueDyn::new(RecordShapeDyn::new());

    // Create an LFO node
    let lfo_node = LfoNode::new(LfoConfig {
        period: 2.0f32.to_fixed(),
        waveform: LfoWaveform::Sine,
    });

    // Convert LfoNode to LpValueBox
    // LfoNode implements RecordValue, so we box it as a RecordValue
    // The value is moved into pool memory, preventing double free
    let lfo_boxed = lp_box_dyn!(lfo_node, dyn RecordValue)?;
    let lfo_value_box = LpValueBox::from(lfo_boxed);

    let name = LpString::try_from_str("test")?;
    nodes
        .add_field(name, lfo_value_box)
        .map_err(|_| lp_pool::AllocError::PoolExhausted)?;

    Ok(nodes)
}

#[test]
fn test_record_metadata() {
    // Test that derived RecordValue types have correct metadata
    let lfo_config = LfoConfig {
        period: 2.0f32.to_fixed(),
        waveform: LfoWaveform::Square,
    };

    use crate::kind::record::record_value::RecordValue;
    let record_shape = RecordValue::shape(&lfo_config);
    let meta = record_shape.meta();
    assert_eq!(
        meta.name(),
        "LfoConfig",
        "LfoConfig should have name 'LfoConfig'"
    );

    let lfo_node = LfoNode::new(lfo_config);
    let record_shape = RecordValue::shape(&lfo_node);
    let meta = record_shape.meta();
    assert_eq!(meta.name(), "LfoNode", "LfoNode should have name 'LfoNode'");
}

#[test]
fn test_scene_traversal() {
    let _allow = enter_global_alloc_allowance();
    let pool = setup_pool();
    pool.run(|| {
        // Create a scene with nodes
        let nodes = build_demo_scene()?;

        // Convert nodes to LpValueBox for traversal
        let nodes_boxed = lp_box_dyn!(nodes, dyn RecordValue)?;
        let nodes_value_box = LpValueBox::from(nodes_boxed);

        // Traverse and print the scene
        let output = LpMemoryPool::with_global_alloc(|| {
            let mut output = String::new();

            // Verify nodes structure
            let nodes_ref = match &nodes_value_box {
                LpValueBox::Record(r) => r.as_ref(),
                _ => panic!("Expected Record"),
            };
            // For dynamic records, the shape might have 0 fields but the value has fields
            // Try to access the first field to verify it exists
            assert!(
                nodes_ref.get_field_by_index(0).is_ok(),
                "Should have at least 1 field"
            );

            // We can't easily get the field name from the value, but we can verify the field exists

            // Get the LFO node from the scene
            let lfo_value_ref = nodes_ref
                .get_field_by_index(0)
                .expect("Should have test field");

            if let crate::kind::value::LpValueRef::Record(lfo_ref) = lfo_value_ref {
                let lfo_shape = RecordValue::shape(lfo_ref);
                let lfo_meta = lfo_shape.meta();
                assert_eq!(
                    lfo_meta.name(),
                    "LfoNode",
                    "LfoNode should have name 'LfoNode'"
                );
                assert_eq!(lfo_shape.field_count(), 2, "LfoNode should have 2 fields");

                // Verify LfoConfig has waveform field
                if let Ok(config_ref) = lfo_ref.get_field_by_index(0) {
                    if let crate::kind::value::LpValueRef::Record(config_record) = config_ref {
                        let config_shape = RecordValue::shape(config_record);
                        assert_eq!(
                            config_shape.field_count(),
                            2,
                            "LfoConfig should have 2 fields"
                        );
                        if let Some(waveform_field) = config_shape.find_field("waveform") {
                            // Verify waveform field exists
                            assert_eq!(waveform_field.name(), "waveform");
                        }
                    }
                }

                // Verify field names
                if let Some(field0) = lfo_shape.get_field(0) {
                    assert_eq!(field0.name(), "config", "First field should be 'config'");
                }
                if let Some(field1) = lfo_shape.get_field(1) {
                    assert_eq!(field1.name(), "output", "Second field should be 'output'");
                }
            }

            // Print the scene graph and capture output
            output.push_str("Scene graph:\n");
            let printed = print_lp_value_to_string(nodes_value_box, 0);
            output.push_str(&printed);
            output
        });

        // Verify the output matches expected format
        let expected_lines = vec![
            "Scene graph:",
            "Record (anonymous)",
            "  test: Record(LfoNode)",
            "    config: Record(LfoConfig)",
            "      period: Fixed(2)",
            "      waveform: Enum(LfoWaveform)::Sine",
            "    output: Fixed(0)",
        ];

        let output_lines: Vec<&str> = output.lines().collect();
        for (i, expected_line) in expected_lines.iter().enumerate() {
            assert!(
                output_lines.get(i).map(|s| s.trim()) == Some(expected_line.trim()),
                "Line {} mismatch: expected '{}', got '{}'",
                i,
                expected_line,
                output_lines.get(i).unwrap_or(&"<missing>")
            );
        }

        // Also verify it contains key elements
        assert!(
            output.contains("LfoNode"),
            "Output should contain 'LfoNode'"
        );
        assert!(
            output.contains("LfoConfig"),
            "Output should contain 'LfoConfig'"
        );
        assert!(
            output.contains("waveform"),
            "Output should contain 'waveform'"
        );
        assert!(output.contains("Sine"), "Output should contain 'Sine'");

        Ok::<(), lp_pool::AllocError>(())
    })
    .unwrap();
}

#[cfg(feature = "serde_json")]
#[test]
fn test_scene_serialization() {
    use serde_json;

    let _allow = enter_global_alloc_allowance();
    let pool = setup_pool();
    pool.run(|| {
        let scene = build_demo_scene().expect("Failed to build scene");

        // Serialize to JSON
        let json = serde_json::to_string(&scene).expect("Failed to serialize scene");
        println!("Serialized scene: {}", json);

        // Verify JSON structure contains the test node
        assert!(json.contains("test"), "JSON should contain 'test' field");

        Ok::<(), lp_pool::AllocError>(())
    })
    .unwrap();
}

#[cfg(feature = "serde_json")]
#[test]
fn test_lfo_node_serialization() {
    use serde_json;

    // Test serialization of individual LFO node
    let lfo_node = LfoNode::new(LfoConfig {
        period: 2.0f32.to_fixed(),
        waveform: LfoWaveform::Triangle,
    });

    // Serialize to JSON
    let json = serde_json::to_string(&lfo_node).expect("Failed to serialize LFO node");
    println!("Serialized LFO node: {}", json);

    // Verify JSON structure
    assert!(json.contains("config"));
    assert!(json.contains("period"));
    assert!(json.contains("waveform"));
    assert!(json.contains("output"));
    // Verify enum is serialized as string
    assert!(json.contains("\"Triangle\"") || json.contains("Triangle"));
}

#[cfg(feature = "serde_json")]
#[test]
fn test_lfo_node_deserialization() {
    use serde_json;

    let original_node = LfoNode::new(LfoConfig {
        period: 2.0f32.to_fixed(),
        waveform: LfoWaveform::Sawtooth,
    });

    // Serialize and deserialize
    let json = serde_json::to_string(&original_node).expect("Failed to serialize");
    let deserialized_node: LfoNode = serde_json::from_str(&json).expect("Failed to deserialize");

    // Verify round-trip
    assert_eq!(
        original_node.config.period.to_f32(),
        deserialized_node.config.period.to_f32()
    );
    assert_eq!(
        original_node.config.waveform,
        deserialized_node.config.waveform
    );
    assert_eq!(
        original_node.output.to_f32(),
        deserialized_node.output.to_f32()
    );
}

#[cfg(feature = "serde_json")]
#[test]
fn test_lfo_node_round_trip() {
    use serde_json;

    let original_node = LfoNode::new(LfoConfig {
        period: 2.0f32.to_fixed(),
        waveform: LfoWaveform::Sine,
    });

    // Round-trip through JSON
    let json = serde_json::to_string(&original_node).expect("Failed to serialize");
    let round_tripped_node: LfoNode = serde_json::from_str(&json).expect("Failed to deserialize");

    // Verify all fields match
    assert_eq!(
        original_node.config.period.to_f32(),
        round_tripped_node.config.period.to_f32(),
        "Period should match after round-trip"
    );
    assert_eq!(
        original_node.config.waveform, round_tripped_node.config.waveform,
        "Waveform should match after round-trip"
    );
    assert_eq!(
        original_node.output.to_f32(),
        round_tripped_node.output.to_f32(),
        "Output should match after round-trip"
    );
}
