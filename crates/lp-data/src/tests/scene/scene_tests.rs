//! Tests for scene graph traversal.
use crate::kind::{record::record_dyn::RecordShapeDyn, value::LpValueBox};
use crate::tests::scene::{
    lfo::{LfoConfig, LfoNode},
    print_lp_value::print_lp_value,
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

#[test]
fn test_scene_traversal() {
    let _allow = enter_global_alloc_allowance();
    let pool = setup_pool();
    pool.run(|| {
        // Create a simple scene with a module containing nodes
        let mut nodes = RecordValueDyn::new(RecordShapeDyn::new());

        // Create an LFO node
        let lfo_node = LfoNode::new(LfoConfig {
            period: 2.0f32.to_fixed(),
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

        // Traverse and print the scene
        LpMemoryPool::with_global_alloc(|| {
            println!("Scene graph:");
            // Convert RecordValueDyn to LpValueBox for printing
            // The value is moved into pool memory
            let nodes_boxed = lp_box_dyn!(nodes, dyn RecordValue).expect("Failed to box nodes");
            let nodes_value_box = LpValueBox::from(nodes_boxed);
            print_lp_value(nodes_value_box, 0);
        });

        Ok::<(), lp_pool::AllocError>(())
    })
    .unwrap();
}
