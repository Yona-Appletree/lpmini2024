//! Tests for scene graph traversal.
use crate::kind::{
    record::record_dyn::RecordShapeDyn,
    value::{LpValue, LpValueBox, RecordValue},
};
use crate::tests::scene::{
    lfo::{LfoConfig, LfoNode},
    print_lp_value::print_lp_value,
};
use core::ptr::NonNull;

extern crate alloc;
use crate::kind::record::RecordValueDyn;
use lp_math::fixed::ToFixed;
use lp_pool::{enter_global_alloc_allowance, LpBoxDyn, LpMemoryPool, LpString};

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
        let lfo_record: &dyn RecordValue = &lfo_node;
        #[allow(deprecated)]
        let lfo_boxed = LpBoxDyn::try_new_unsized(lfo_record)?;
        let lfo_value_box = LpValueBox::from(lfo_boxed);
        let name = LpString::try_from_str("test")?;
        nodes
            .add_field(name, lfo_value_box)
            .map_err(|_| lp_pool::AllocError::PoolExhausted)?;

        // Traverse and print the scene
        LpMemoryPool::with_global_alloc(|| {
            println!("Scene graph:");
            // Convert RecordValueDyn to LpValueBox for printing
            let nodes_record: &dyn RecordValue = &nodes;
            #[allow(deprecated)]
            let nodes_boxed = LpBoxDyn::try_new_unsized(nodes_record).expect("Failed to box nodes");
            let nodes_value_box = LpValueBox::from(nodes_boxed);
            print_lp_value(nodes_value_box, 0);
        });

        Ok::<(), lp_pool::AllocError>(())
    })
    .unwrap();
}
