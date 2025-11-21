#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(feature = "std")]
use std::collections::BTreeMap;
#[cfg(feature = "std")]
use std::string::String;

use crate::nodes::lfo::lfo_input::LfoInput;

/// Scene configuration containing node definitions.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LpSceneConfig {
    /// Map of node IDs to their configurations.
    ///
    /// For now, we support LFO nodes only. The value is the LfoInput.
    /// Using BTreeMap directly since RecordValue doesn't support it yet.
    pub nodes: BTreeMap<String, LfoInput>,
}

impl Default for LpSceneConfig {
    fn default() -> Self {
        Self {
            nodes: BTreeMap::new(),
        }
    }
}

impl LpSceneConfig {
    /// Create a new empty scene config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an LFO node to the scene.
    pub fn add_lfo_node(&mut self, node_id: impl Into<String>, input: LfoInput) {
        self.nodes.insert(node_id.into(), input);
    }
}
