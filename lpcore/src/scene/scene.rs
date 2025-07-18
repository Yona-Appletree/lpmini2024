use crate::scene::scene_node::SceneNode;
use crate::scene::SceneConfig;
use indexmap::IndexMap;

pub struct Scene {
    pub frame_counter: u64,
    pub nodes: IndexMap<String, SceneNode>,
}

impl Scene {
    pub fn new() -> Scene {
        Self {
            frame_counter: 0,
            nodes: IndexMap::new(),
        }
    }

    pub fn apply_config(&mut self, config: &SceneConfig) {
        // remove old entities
        // add new entities
        // apply inputs
        // apply bindings
    }
}
