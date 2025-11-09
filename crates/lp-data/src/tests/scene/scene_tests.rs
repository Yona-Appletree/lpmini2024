//! Tests for scene graph traversal.

extern crate alloc;

use crate::kind::{
    record::record_dyn::RecordShapeDyn,
    record::RecordValueDyn,
    value::{LpValueBox, RecordValue},
};
use crate::tests::scene::{
    lfo::{LfoConfig, LfoNode},
    print_lp_value::print_lp_value,
};
use alloc::string::String;
use lp_math::fixed::ToFixed;

#[test]
fn test_scene_traversal() {
    // Create a simple scene with a module containing nodes
    let mut nodes = RecordValueDyn::new(RecordShapeDyn::new());

    // Create an LFO node
    let lfo_node = LfoNode::new(LfoConfig {
        period: 2.0f32.to_fixed(),
    });

    // Convert LfoNode to LpValueBox
    let lfo_value_box = LpValueBox::from(Box::new(lfo_node) as Box<dyn RecordValue>);
    nodes
        .add_field(String::from("test"), lfo_value_box)
        .unwrap();

    // Traverse and print the scene
    println!("Scene graph:");
    let nodes_value_box = LpValueBox::from(Box::new(nodes) as Box<dyn RecordValue>);
    print_lp_value(nodes_value_box, 0);
}
