#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(feature = "std")]
use std::collections::BTreeMap;
#[cfg(feature = "std")]
use std::string::String;

use lp_data::kind::record::record_value::RecordValue;
use lp_data::kind::value::LpValueRef;
use lp_data::RuntimeError;
use lp_math::dec32::Dec32;

use crate::node::{LpNode, NodeContext};
use crate::nodes::LfoNode;
use crate::scene_config::LpSceneConfig;

/// Node instance enum (will be expanded as more node types are added).
pub enum NodeInstance {
    Lfo(LfoNode),
}

impl LpNode for NodeInstance {
    fn update(&mut self, context: &dyn NodeContext) -> Result<(), RuntimeError> {
        match self {
            NodeInstance::Lfo(node) => node.update(context),
        }
    }
}

impl lp_data::kind::value::LpValue for NodeInstance {
    fn shape(&self) -> &dyn lp_data::kind::shape::LpShape {
        match self {
            NodeInstance::Lfo(node) => lp_data::kind::value::LpValue::shape(node),
        }
    }
}

impl RecordValue for NodeInstance {
    fn shape(&self) -> &dyn lp_data::kind::record::record_shape::RecordShape {
        match self {
            NodeInstance::Lfo(node) => RecordValue::shape(node),
        }
    }

    fn get_field_by_index(&self, index: usize) -> Result<LpValueRef<'_>, RuntimeError> {
        match self {
            NodeInstance::Lfo(node) => node.get_field_by_index(index),
        }
    }

    fn get_field_by_index_mut(
        &mut self,
        index: usize,
    ) -> Result<lp_data::kind::value::LpValueRefMut<'_>, RuntimeError> {
        match self {
            NodeInstance::Lfo(node) => node.get_field_by_index_mut(index),
        }
    }
}

/// Runtime scene containing node instances.
pub struct LpScene {
    /// Map of node IDs to node instances.
    nodes: BTreeMap<String, NodeInstance>,

    /// Current frame counter.
    frame_counter: u64,

    /// Last frame time in milliseconds, used to calculate delta time.
    last_frame_time_ms: Option<i64>,
}

impl LpScene {
    /// Create a new scene from a configuration.
    pub fn from_config(config: &LpSceneConfig) -> Result<Self, RuntimeError> {
        let mut nodes = BTreeMap::new();

        for (node_id, input) in &config.nodes {
            let node = NodeInstance::Lfo(LfoNode::with_input(input.clone()));
            nodes.insert(node_id.clone(), node);
        }

        Ok(Self {
            nodes,
            frame_counter: 0,
            last_frame_time_ms: None,
        })
    }

    /// Update all nodes for the current frame.
    ///
    /// This updates nodes in bottom-up order (children before parents).
    /// For now, we update all nodes in the order they were added.
    pub fn update_frame(&mut self, frame_time_ms: i64) -> Result<(), RuntimeError> {
        self.frame_counter += 1;

        // Calculate delta time
        let delta_ms = if let Some(last_time) = self.last_frame_time_ms {
            Dec32::from_f32((frame_time_ms - last_time) as f32)
        } else {
            // First frame: use zero delta
            Dec32::ZERO
        };

        // Update last frame time
        self.last_frame_time_ms = Some(frame_time_ms);

        // Create context that doesn't hold a reference to self
        struct SimpleContext {
            frame_time_ms: i64,
            delta_ms: Dec32,
        }

        impl NodeContext for SimpleContext {
            fn frame_time_ms(&self) -> i64 {
                self.frame_time_ms
            }

            fn delta_ms(&self) -> Dec32 {
                self.delta_ms
            }
        }

        let context = SimpleContext {
            frame_time_ms,
            delta_ms,
        };

        // Update all nodes
        for node in self.nodes.values_mut() {
            LpNode::update(node, &context)?;
        }

        Ok(())
    }

    /// Get a mutable reference to a node by ID for updating.
    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut NodeInstance> {
        self.nodes.get_mut(node_id)
    }

    /// Get the output value of a node.
    ///
    /// This accesses the node's output field via RecordValue.
    pub fn get_node_output(
        &self,
        node_id: &str,
        output_name: &str,
    ) -> Result<LpValueRef, RuntimeError> {
        let node = self
            .nodes
            .get(node_id)
            .ok_or_else(|| RuntimeError::field_not_found("LpScene", node_id))?;

        node.get_field(output_name)
    }

    /// Get the current frame counter.
    pub fn frame_counter(&self) -> u64 {
        self.frame_counter
    }
}
