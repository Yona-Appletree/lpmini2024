//! Tests for scene graph traversal.
use crate::kind::{
    record::record_dyn::RecordShapeDyn,
    value::{LpValue, RecordValue},
};
use crate::tests::scene::print_lp_value::print_record_value;
use crate::tests::scene::{
    lfo::{LfoConfig, LfoNode},
    print_lp_value::print_lp_value,
};
use core::ptr::NonNull;

extern crate alloc;
use crate::kind::record::RecordValueDyn;
use lp_math::fixed::ToFixed;
use lp_pool::{LpBoxDyn, LpMemoryPool, LpString};

fn setup_pool() -> LpMemoryPool {
    let mut memory = [0u8; 16384];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() }
}

#[test]
fn test_scene_traversal() {
    let pool = setup_pool();
    pool.run(|| {
        // Create a simple scene with a module containing nodes
        let mut nodes = RecordValueDyn::new(RecordShapeDyn::new());

        // Create an LFO node
        let lfo_node = LfoNode::new(LfoConfig {
            period: 2.0f32.to_fixed(),
        });

        // Convert LfoNode to LpBoxDyn<dyn LpValue>
        // LfoNode is sized, so we can use try_new, but we need dyn LpValue
        // Use the deprecated try_new_unsized for now (LfoNode implements LpValue)
        let lfo_value: &dyn LpValue = &lfo_node;
        #[allow(deprecated)]
        let lfo_boxed = LpBoxDyn::try_new_unsized(lfo_value)?;
        let name = LpString::try_from_str("test")?;
        nodes
            .add_field(name, lfo_boxed)
            .map_err(|_| lp_pool::AllocError::PoolExhausted)?;

        // Traverse and print the scene
        LpMemoryPool::with_global_alloc(|| {
            println!("Scene graph:");
            // Since nodes is RecordValueDyn, we can pass it as &dyn RecordValue
            print_record_value(&nodes as &dyn RecordValue, 0);
        });

        Ok::<(), lp_pool::AllocError>(())
    })
    .unwrap();
}
