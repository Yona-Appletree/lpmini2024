//! Test demonstrating runtime value system with a simple scene graph.

use core::ptr::NonNull;

use lp_math::fixed::{Fixed, Vec3};
use lp_pool::error::AllocError;
use lp_pool::memory_pool::LpMemoryPool;

use crate::metadata::{LpTypeMeta, RecordField, RecordType};
use crate::value::LpValue;
use crate::{LpDescribe, TypeRef};

use super::nodes::{lfo::LfoConfig, perlin3::Perlin3Input};

// Static type metadata for runtime scene graph
const FIXED_META: LpTypeMeta = LpTypeMeta::new(crate::LpType::fixed());
const VEC3_META: LpTypeMeta = LpTypeMeta::new(crate::LpType::vec3());

// LFO node type: { config: LfoConfig, output: Fixed }
// We'll construct this at runtime since lp_schema() is not const

// Perlin3 input type: { pos: Vec3 }
const PERLIN3_INPUT_FIELDS: &[RecordField<TypeRef>] = &[RecordField::new("pos", &VEC3_META)];
const PERLIN3_INPUT_RECORD_TYPE: RecordType<TypeRef> =
    RecordType::new("Perlin3Input", PERLIN3_INPUT_FIELDS);
const PERLIN3_INPUT_META: LpTypeMeta =
    LpTypeMeta::new(crate::LpType::Record(PERLIN3_INPUT_RECORD_TYPE));

// Perlin3 node type: { input: Perlin3Input }
const PERLIN3_FIELDS: &[RecordField<TypeRef>] = &[RecordField::new("input", &PERLIN3_INPUT_META)];
const PERLIN3_RECORD_TYPE: RecordType<TypeRef> = RecordType::new("Perlin3Node", PERLIN3_FIELDS);
const PERLIN3_NODE_META: LpTypeMeta = LpTypeMeta::new(crate::LpType::Record(PERLIN3_RECORD_TYPE));

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
        const SIMPLE_NODES_FIELDS: &[RecordField<TypeRef>] =
            &[RecordField::new("perlin3", &PERLIN3_NODE_META)];
        const SIMPLE_NODES_RECORD_TYPE: RecordType<TypeRef> =
            RecordType::new("Nodes", SIMPLE_NODES_FIELDS);
        const SIMPLE_NODES_META: LpTypeMeta =
            LpTypeMeta::new(crate::LpType::Record(SIMPLE_NODES_RECORD_TYPE));

        // Create the runtime value structure
        let mut nodes = LpValue::try_record(&SIMPLE_NODES_META)?;

        // Initialize Perlin3 node
        let mut perlin3_node = nodes
            .get_field_mut("perlin3")
            .map_err(|_| AllocError::InvalidLayout)?;
        let mut perlin3_input = perlin3_node
            .get_field_mut("input")
            .map_err(|_| AllocError::InvalidLayout)?;
        let perlin3_pos = perlin3_input
            .get_field_mut("pos")
            .map_err(|_| AllocError::InvalidLayout)?;
        *perlin3_pos = LpValue::vec3(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO);

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
    assert!(nodes.get_field("perlin3").is_ok());
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
    let mut perlin3_node = nodes.get_field_mut("perlin3").unwrap();
    let mut perlin3_input = perlin3_node.get_field_mut("input").unwrap();
    let perlin3_pos = perlin3_input.get_field_mut("pos").unwrap();
    *perlin3_pos = LpValue::vec3(pos_value.x, pos_value.y, pos_value.z);

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
    let perlin3_node = nodes
        .get_field("perlin3")
        .expect("perlin3 node should exist");
    let perlin3_input = perlin3_node
        .get_field("input")
        .expect("perlin3.input should exist");
    let perlin3_pos = perlin3_input
        .get_field("pos")
        .expect("perlin3.input.pos should exist");
    let (x, y, z) = perlin3_pos.as_vec3().expect("should be Vec3");
    assert_eq!(x, Fixed::ZERO);
    assert_eq!(y, Fixed::ZERO);
    assert_eq!(z, Fixed::ZERO);
}
