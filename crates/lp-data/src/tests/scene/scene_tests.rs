//! Tests for scene graph traversal.
use crate::kind::record::record_dyn::RecordShapeDyn;
use crate::kind::value::LpValueBox;
use crate::tests::scene::print_lp_value::print_lp_value_to_string;
use crate::tests::scene::step_config::StepConfig;
use crate::tests::scene::test_node::{LfoWaveform, TestNode, TestNodeConfig};

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;

use lp_alloc::{enter_global_alloc_allowance, init_test_allocator, AllocLimitError as AllocError};
use lp_math::fixed::{Fixed, Mat3, ToFixed, Vec2, Vec3, Vec4};

use crate::kind::record::record_value::RecordValue;
use crate::kind::record::RecordValueDyn;
struct TestPool;

impl TestPool {
    fn run<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _guard = enter_global_alloc_allowance();
        f()
    }
}

fn setup_pool() -> TestPool {
    init_test_allocator();
    TestPool
}

/// Build a demo scene with a module containing nodes.
/// Must be called within a pool context.
fn build_demo_scene() -> Result<RecordValueDyn, AllocError> {
    // Create a simple scene with a module containing nodes
    let mut nodes = RecordValueDyn::new(RecordShapeDyn::new());

    // Create a test node with all primitive types
    let test_node = TestNode::new(TestNodeConfig {
        period: 2.0f32.to_fixed(),
        waveform: LfoWaveform::Sine,
        count: 42,
        enabled: true,
        position: Vec2::new(Fixed::ZERO, Fixed::ZERO),
        rotation: Vec3::new(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO),
        color: Vec4::new(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO, Fixed::ZERO),
        transform: Mat3::identity(),
        steps: vec![StepConfig::Expr {
            output: Fixed::ZERO,
            param_count: 0,
        }],
        values: vec![1, 2, 3, 4, 5],
        optional_count: Some(100),
    });

    // Convert TestNode to LpValueBox
    // TestNode implements RecordValue, so we box it as a RecordValue
    // The value is moved into pool memory, preventing double free
    let test_boxed: Box<dyn RecordValue> = Box::new(test_node);
    let test_value_box = LpValueBox::from(test_boxed);

    let name = "test".to_string();
    nodes
        .add_field(name, test_value_box)
        .map_err(|_| AllocError::SoftLimitExceeded)?;

    Ok(nodes)
}

#[test]
fn test_record_metadata() {
    // Test that derived RecordValue types have correct metadata
    let test_config = TestNodeConfig {
        period: 2.0f32.to_fixed(),
        waveform: LfoWaveform::Square,
        count: 0,
        enabled: false,
        position: Vec2::new(Fixed::ZERO, Fixed::ZERO),
        rotation: Vec3::new(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO),
        color: Vec4::new(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO, Fixed::ZERO),
        transform: Mat3::identity(),
        steps: vec![StepConfig::Palette {
            size: 256,
            brightness: Fixed::ONE,
        }],
        values: vec![],
        optional_count: None,
    };

    use crate::kind::record::record_value::RecordValue;
    let record_shape = RecordValue::shape(&test_config);
    let meta = record_shape.meta();
    assert_eq!(
        meta.name(),
        "TestNodeConfig",
        "TestNodeConfig should have name 'TestNodeConfig'"
    );
    assert_eq!(
        record_shape.field_count(),
        11,
        "TestNodeConfig should have 11 fields"
    );

    let test_node = TestNode::new(test_config);
    let record_shape = RecordValue::shape(&test_node);
    let meta = record_shape.meta();
    assert_eq!(
        meta.name(),
        "TestNode",
        "TestNode should have name 'TestNode'"
    );
}

#[test]
fn test_scene_traversal() {
    let _allow = enter_global_alloc_allowance();
    let pool = setup_pool();
    let output = pool
        .run(|| -> Result<String, AllocError> {
            // Create a scene with nodes
            let nodes = build_demo_scene()?;

            // Convert nodes to LpValueBox for traversal
            let nodes_boxed: Box<dyn RecordValue> = Box::new(nodes);
            let nodes_value_box = LpValueBox::from(nodes_boxed);

            // Traverse and print the scene
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
                    "TestNode",
                    "TestNode should have name 'TestNode'"
                );
                assert_eq!(lfo_shape.field_count(), 2, "TestNode should have 2 fields");

                // Verify LfoConfig has waveform field
                if let Ok(crate::kind::value::LpValueRef::Record(config_record)) =
                    lfo_ref.get_field_by_index(0)
                {
                    let config_shape = RecordValue::shape(config_record);
                    assert_eq!(
                        config_shape.field_count(),
                        11,
                        "TestNodeConfig should have 11 fields"
                    );
                    if let Some(waveform_field) = config_shape.find_field("waveform") {
                        // Verify waveform field exists
                        assert_eq!(waveform_field.name(), "waveform");
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
            Ok(output)
        })
        .unwrap();

    // Verify the output matches expected format
    let expected_lines = vec![
        "Scene graph:",
        "Record (anonymous)",
        "  test: Record(TestNode)",
        "    config: Record(TestNodeConfig)",
        "      period: Fixed(2)",
        "      waveform: EnumUnit(LfoWaveform)::Sine",
        "      count: Int32(42)",
        "      enabled: Bool(true)",
        "      position: Vec2(0, 0)",
        "      rotation: Vec3(0, 0, 0)",
        "      color: Vec4(0, 0, 0, 0)",
        "      transform: Mat3(1, 0, 0, 0, 1, 0, 0, 0, 1)",
        "      steps: Array[1]",
        "        [0]: Union(StepConfig)::Expr",
        "          value: Record(Expr)",
        "            output: Fixed(0)",
        "            param_count: Int32(0)",
        "      values: Array[5]",
        "        [0]: Int32(1)",
        "        [1]: Int32(2)",
        "        [2]: Int32(3)",
        "        [3]: Int32(4)",
        "        [4]: Int32(5)",
        "      optional_count: Option::Some",
        "        value: Int32(100)",
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
        output.contains("TestNode"),
        "Output should contain 'TestNode'"
    );
    assert!(
        output.contains("TestNodeConfig"),
        "Output should contain 'TestNodeConfig'"
    );
    assert!(
        output.contains("waveform"),
        "Output should contain 'waveform'"
    );
    assert!(output.contains("Sine"), "Output should contain 'Sine'");
    assert!(
        output.contains("Int32(42)"),
        "Output should contain 'Int32(42)'"
    );
    assert!(
        output.contains("Bool(true)"),
        "Output should contain 'Bool(true)'"
    );
    assert!(
        output.contains("Vec2(0, 0)"),
        "Output should contain 'Vec2(0, 0)'"
    );
    assert!(
        output.contains("Vec3(0, 0, 0)"),
        "Output should contain 'Vec3(0, 0, 0)'"
    );
    assert!(
        output.contains("Vec4(0, 0, 0, 0)"),
        "Output should contain 'Vec4(0, 0, 0, 0)'"
    );
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

        Ok::<(), AllocError>(())
    })
    .unwrap();
}

#[cfg(feature = "serde_json")]
#[test]
fn test_lfo_node_serialization() {
    use serde_json;

    // Test serialization of individual test node
    let test_node = TestNode::new(TestNodeConfig {
        period: 2.0f32.to_fixed(),
        waveform: LfoWaveform::Triangle,
        count: 100,
        enabled: true,
        position: Vec2::new(10.0f32.to_fixed(), 20.0f32.to_fixed()),
        rotation: Vec3::new(1.0f32.to_fixed(), 2.0f32.to_fixed(), 3.0f32.to_fixed()),
        color: Vec4::new(
            0.5f32.to_fixed(),
            0.6f32.to_fixed(),
            0.7f32.to_fixed(),
            1.0f32.to_fixed(),
        ),
        transform: Mat3::identity(),
        steps: vec![StepConfig::Blur {
            radius: 1.to_fixed(), // 1.0
        }],
        values: vec![10, 20],
        optional_count: Some(50),
    });

    // Serialize to JSON
    let json = serde_json::to_string(&test_node).expect("Failed to serialize test node");
    println!("Serialized test node: {}", json);

    // Verify JSON structure contains all fields
    assert!(json.contains("config"));
    assert!(json.contains("period"));
    assert!(json.contains("waveform"));
    assert!(json.contains("count"));
    assert!(json.contains("enabled"));
    assert!(json.contains("position"));
    assert!(json.contains("rotation"));
    assert!(json.contains("color"));
    assert!(json.contains("output"));
    // Verify enum is serialized as string
    assert!(json.contains("\"Triangle\"") || json.contains("Triangle"));
    // Verify primitive types are serialized correctly
    assert!(json.contains("100"), "JSON should contain count value");
    assert!(json.contains("true"), "JSON should contain enabled value");
}

#[cfg(feature = "serde_json")]
#[test]
fn test_lfo_node_deserialization() {
    use serde_json;

    let original_node = TestNode::new(TestNodeConfig {
        period: 2.0f32.to_fixed(),
        waveform: LfoWaveform::Sawtooth,
        count: 42,
        enabled: false,
        position: Vec2::new(1.0f32.to_fixed(), 2.0f32.to_fixed()),
        rotation: Vec3::new(3.0f32.to_fixed(), 4.0f32.to_fixed(), 5.0f32.to_fixed()),
        color: Vec4::new(
            0.1f32.to_fixed(),
            0.2f32.to_fixed(),
            0.3f32.to_fixed(),
            0.4f32.to_fixed(),
        ),
        transform: Mat3::identity(),
        steps: vec![StepConfig::Expr {
            output: Fixed::ZERO,
            param_count: 2,
        }],
        values: vec![1, 2, 3],
        optional_count: Some(200),
    });

    // Serialize and deserialize
    let json = serde_json::to_string(&original_node).expect("Failed to serialize");
    let deserialized_node: TestNode = serde_json::from_str(&json).expect("Failed to deserialize");

    // Verify round-trip for all fields
    assert_eq!(
        original_node.config.period.to_f32(),
        deserialized_node.config.period.to_f32()
    );
    assert_eq!(
        original_node.config.waveform,
        deserialized_node.config.waveform
    );
    assert_eq!(original_node.config.count, deserialized_node.config.count);
    assert_eq!(
        original_node.config.enabled,
        deserialized_node.config.enabled
    );
    assert_eq!(
        original_node.config.position.x.to_f32(),
        deserialized_node.config.position.x.to_f32()
    );
    assert_eq!(
        original_node.config.position.y.to_f32(),
        deserialized_node.config.position.y.to_f32()
    );
    assert_eq!(
        original_node.config.rotation.x.to_f32(),
        deserialized_node.config.rotation.x.to_f32()
    );
    assert_eq!(
        original_node.config.rotation.y.to_f32(),
        deserialized_node.config.rotation.y.to_f32()
    );
    assert_eq!(
        original_node.config.rotation.z.to_f32(),
        deserialized_node.config.rotation.z.to_f32()
    );
    assert_eq!(
        original_node.config.color.x.to_f32(),
        deserialized_node.config.color.x.to_f32()
    );
    assert_eq!(
        original_node.config.color.y.to_f32(),
        deserialized_node.config.color.y.to_f32()
    );
    assert_eq!(
        original_node.config.color.z.to_f32(),
        deserialized_node.config.color.z.to_f32()
    );
    assert_eq!(
        original_node.config.color.w.to_f32(),
        deserialized_node.config.color.w.to_f32()
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

    let original_node = TestNode::new(TestNodeConfig {
        period: 2.0f32.to_fixed(),
        waveform: LfoWaveform::Sine,
        count: 123,
        enabled: true,
        position: Vec2::new(5.0f32.to_fixed(), 10.0f32.to_fixed()),
        rotation: Vec3::new(15.0f32.to_fixed(), 30.0f32.to_fixed(), 45.0f32.to_fixed()),
        color: Vec4::new(
            0.8f32.to_fixed(),
            0.9f32.to_fixed(),
            1.0f32.to_fixed(),
            0.5f32.to_fixed(),
        ),
        transform: Mat3::identity(),
        steps: vec![StepConfig::Palette {
            size: 128,
            brightness: 1.5f32.to_fixed(), // 1.5
        }],
        values: vec![5, 10, 15, 20],
        optional_count: Some(300),
    });

    // Round-trip through JSON
    let json = serde_json::to_string(&original_node).expect("Failed to serialize");
    let round_tripped_node: TestNode = serde_json::from_str(&json).expect("Failed to deserialize");

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
        original_node.config.count, round_tripped_node.config.count,
        "Count should match after round-trip"
    );
    assert_eq!(
        original_node.config.enabled, round_tripped_node.config.enabled,
        "Enabled should match after round-trip"
    );
    assert_eq!(
        original_node.config.position.x.to_f32(),
        round_tripped_node.config.position.x.to_f32(),
        "Position.x should match after round-trip"
    );
    assert_eq!(
        original_node.config.position.y.to_f32(),
        round_tripped_node.config.position.y.to_f32(),
        "Position.y should match after round-trip"
    );
    assert_eq!(
        original_node.config.rotation.x.to_f32(),
        round_tripped_node.config.rotation.x.to_f32(),
        "Rotation.x should match after round-trip"
    );
    assert_eq!(
        original_node.config.rotation.y.to_f32(),
        round_tripped_node.config.rotation.y.to_f32(),
        "Rotation.y should match after round-trip"
    );
    assert_eq!(
        original_node.config.rotation.z.to_f32(),
        round_tripped_node.config.rotation.z.to_f32(),
        "Rotation.z should match after round-trip"
    );
    assert_eq!(
        original_node.config.color.x.to_f32(),
        round_tripped_node.config.color.x.to_f32(),
        "Color.x should match after round-trip"
    );
    assert_eq!(
        original_node.config.color.y.to_f32(),
        round_tripped_node.config.color.y.to_f32(),
        "Color.y should match after round-trip"
    );
    assert_eq!(
        original_node.config.color.z.to_f32(),
        round_tripped_node.config.color.z.to_f32(),
        "Color.z should match after round-trip"
    );
    assert_eq!(
        original_node.config.color.w.to_f32(),
        round_tripped_node.config.color.w.to_f32(),
        "Color.w should match after round-trip"
    );
    assert_eq!(
        original_node.output.to_f32(),
        round_tripped_node.output.to_f32(),
        "Output should match after round-trip"
    );
}

#[test]
fn test_print_all_primitive_types() {
    let _allow = enter_global_alloc_allowance();
    let pool = setup_pool();
    pool.run(|| {
        let test_node = TestNode::new(TestNodeConfig {
            period: core::f32::consts::PI.to_fixed(),
            waveform: LfoWaveform::Square,
            count: 999,
            enabled: true,
            position: Vec2::new(100.0f32.to_fixed(), 200.0f32.to_fixed()),
            rotation: Vec3::new(1.0f32.to_fixed(), 2.0f32.to_fixed(), 3.0f32.to_fixed()),
            color: Vec4::new(
                0.25f32.to_fixed(),
                0.5f32.to_fixed(),
                0.75f32.to_fixed(),
                1.0f32.to_fixed(),
            ),
            transform: Mat3::identity(),
            steps: vec![StepConfig::Blur {
                radius: 2.0f32.to_fixed(), // 2.0
            }],
            values: vec![100],
            optional_count: None,
        });

        let test_boxed: Box<dyn RecordValue> = Box::new(test_node);
        let test_value_box = LpValueBox::from(test_boxed);

        let output = print_lp_value_to_string(test_value_box, 0);

        // Verify all primitive types are printed correctly
        assert!(
            output.contains("Int32(999)"),
            "Output should contain Int32 value"
        );
        assert!(
            output.contains("Bool(true)"),
            "Output should contain Bool value"
        );
        assert!(
            output.contains("Vec2(100, 200)"),
            "Output should contain Vec2 value"
        );
        assert!(
            output.contains("Vec3(1, 2, 3)"),
            "Output should contain Vec3 value"
        );
        assert!(
            output.contains("Vec4(0.25, 0.5, 0.75, 1)"),
            "Output should contain Vec4 value"
        );
        // Fixed is printed with period field - check for "period: Fixed("
        assert!(
            output.contains("period: Fixed("),
            "Output should contain Fixed value for period field, got: {}",
            output
        );

        Ok::<(), AllocError>(())
    })
    .unwrap();
}
