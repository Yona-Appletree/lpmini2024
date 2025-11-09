//! Test demonstrating runtime value system with a simple scene graph.

use core::ptr::NonNull;

use lp_math::fixed::{Fixed, Vec3};
use lp_pool::error::AllocError;
use lp_pool::memory_pool::LpMemoryPool;

use crate::shape::record::{RecordField, StaticRecordShape};
use crate::shape::shape_ref::Vec3ShapeRef;
use crate::shape::shape_ref::{RecordShapeRef, ShapeRef};
use crate::shape::vec3::StaticVec3Shape;
use crate::value::LpValue;

// Static shapes for runtime scene graph
static VEC3_SHAPE_STATIC: StaticVec3Shape = StaticVec3Shape::default();
static VEC3_SHAPE: ShapeRef = ShapeRef::Vec3(Vec3ShapeRef::Static(&VEC3_SHAPE_STATIC));

// Perlin3 input type: { pos: Vec3 }
static PERLIN3_INPUT_FIELDS: &[RecordField] = &[RecordField::new("pos", VEC3_SHAPE)];
static PERLIN3_INPUT_SHAPE: StaticRecordShape = StaticRecordShape {
    name: "Perlin3Input",
    fields: PERLIN3_INPUT_FIELDS,
    ui: crate::shape::record::RecordUi { collapsible: false },
};
static PERLIN3_INPUT_SHAPE_REF: ShapeRef =
    ShapeRef::Record(RecordShapeRef::Static(&PERLIN3_INPUT_SHAPE));

// Perlin3 node type: { input: Perlin3Input }
static PERLIN3_FIELDS: &[RecordField] = &[RecordField::new("input", PERLIN3_INPUT_SHAPE_REF)];
static PERLIN3_NODE_SHAPE: StaticRecordShape = StaticRecordShape {
    name: "Perlin3Node",
    fields: PERLIN3_FIELDS,
    ui: crate::shape::record::RecordUi { collapsible: false },
};
static PERLIN3_NODE_SHAPE_REF: ShapeRef =
    ShapeRef::Record(RecordShapeRef::Static(&PERLIN3_NODE_SHAPE));

// Top-level nodes type: { lfo: LfoNode, perlin3: Perlin3Node }
// We'll construct this at runtime

/// Create a runtime scene graph structure.
///
/// Structure: {
///   nodes: {
///     lfo: { config: LfoConfig, output: Fixed },
///     perlin3: { input: { pos: Vec3 } }
///   }
/// }
fn create_scene_graph(pool: &LpMemoryPool) -> Result<LpValue, AllocError> {
    pool.run(|| {
        // For this test, we'll create a simpler structure using existing static types
        // We'll create: { perlin3: { input: { pos: Vec3 } } }
        // This demonstrates the runtime value system without needing to create new record types

        // Create a record with just perlin3 node for now
        // In a real implementation, we'd need to support dynamic record type creation
        static SIMPLE_NODES_FIELDS: &[RecordField] =
            &[RecordField::new("perlin3", PERLIN3_NODE_SHAPE_REF)];
        static SIMPLE_NODES_SHAPE: StaticRecordShape = StaticRecordShape {
            name: "Nodes",
            fields: SIMPLE_NODES_FIELDS,
            ui: crate::shape::record::RecordUi { collapsible: false },
        };
        let nodes_shape_ref = ShapeRef::Record(RecordShapeRef::Static(&SIMPLE_NODES_SHAPE));

        // Create the runtime value structure
        let mut nodes = LpValue::try_record(nodes_shape_ref)?;

        // Initialize Perlin3 node
        match &mut nodes {
            LpValue::Struct(s) => {
                let perlin3_node = s
                    .get_field_mut("perlin3")
                    .map_err(|_| AllocError::InvalidLayout)?;
                match perlin3_node {
                    LpValue::Struct(s2) => {
                        let perlin3_input = s2
                            .get_field_mut("input")
                            .map_err(|_| AllocError::InvalidLayout)?;
                        match perlin3_input {
                            LpValue::Struct(s3) => {
                                *s3.get_field_mut("pos")
                                    .map_err(|_| AllocError::InvalidLayout)? =
                                    LpValue::vec3(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO);
                            }
                            _ => return Err(AllocError::InvalidLayout),
                        }
                    }
                    _ => return Err(AllocError::InvalidLayout),
                }
            }
            _ => return Err(AllocError::InvalidLayout),
        }

        Ok(nodes)
    })
}

#[test]
fn test_runtime_scene_graph_creation() {
    let mut memory = [0u8; 16384];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    let pool = unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() };

    let nodes = create_scene_graph(&pool).expect("failed to create scene graph");

    // Verify structure exists
    match &nodes {
        LpValue::Struct(s) => assert!(s.get_field("perlin3").is_ok()),
        _ => panic!("expected Struct"),
    }
}

#[test]
fn test_runtime_scene_graph_path_access() {
    let mut memory = [0u8; 16384];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    let pool = unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() };

    let nodes = create_scene_graph(&pool).expect("failed to create scene graph");

    // Test path-based access
    let perlin3_pos = nodes
        .get_path("perlin3.input.pos")
        .expect("failed to get perlin3.input.pos");
    let (x, y, z) = perlin3_pos.as_vec3().expect("pos should be Vec3");
    assert_eq!(x, Fixed::ZERO);
    assert_eq!(y, Fixed::ZERO);
    assert_eq!(z, Fixed::ZERO);
}

#[test]
fn test_runtime_scene_graph_dynamic_update() {
    let mut memory = [0u8; 16384];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    let pool = unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() };

    let mut nodes = create_scene_graph(&pool).expect("failed to create scene graph");

    // Simulate setting Perlin3's pos input
    // Create a Vec3 value
    let pos_value = Vec3::new(
        Fixed::from_f32(0.5),
        Fixed::from_f32(0.5),
        Fixed::from_f32(0.5),
    );
    match &mut nodes {
        LpValue::Struct(s) => {
            let perlin3_node = s.get_field_mut("perlin3").unwrap();
            match perlin3_node {
                LpValue::Struct(s2) => {
                    let perlin3_input = s2.get_field_mut("input").unwrap();
                    match perlin3_input {
                        LpValue::Struct(s3) => {
                            *s3.get_field_mut("pos").unwrap() =
                                LpValue::vec3(pos_value.x, pos_value.y, pos_value.z);
                        }
                        _ => panic!("expected Struct"),
                    }
                }
                _ => panic!("expected Struct"),
            }
        }
        _ => panic!("expected Struct"),
    }

    // Verify Perlin3's pos was updated
    let updated_pos = nodes
        .get_path("perlin3.input.pos")
        .expect("failed to get perlin3.input.pos");
    let (x, y, z) = updated_pos.as_vec3().expect("pos should be Vec3");
    assert_eq!(x, Fixed::from_f32(0.5));
    assert_eq!(y, Fixed::from_f32(0.5));
    assert_eq!(z, Fixed::from_f32(0.5));
}

#[test]
fn test_runtime_scene_graph_field_access() {
    let mut memory = [0u8; 16384];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    let pool = unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() };

    let nodes = create_scene_graph(&pool).expect("failed to create scene graph");

    // Test field-by-field access (as the VM would do)
    match &nodes {
        LpValue::Struct(s) => {
            let perlin3_node = s.get_field("perlin3").expect("perlin3 node should exist");
            match perlin3_node {
                LpValue::Struct(s2) => {
                    let perlin3_input = s2.get_field("input").expect("perlin3.input should exist");
                    match perlin3_input {
                        LpValue::Struct(s3) => {
                            let perlin3_pos =
                                s3.get_field("pos").expect("perlin3.input.pos should exist");
                            let (x, y, z) = perlin3_pos.as_vec3().expect("should be Vec3");
                            assert_eq!(x, Fixed::ZERO);
                            assert_eq!(y, Fixed::ZERO);
                            assert_eq!(z, Fixed::ZERO);
                        }
                        _ => panic!("expected Struct"),
                    }
                }
                _ => panic!("expected Struct"),
            }
        }
        _ => panic!("expected Struct"),
    }
}
