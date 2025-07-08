use crate::scene::{SceneConfig, SceneEntity};
use indexmap::IndexMap;

pub struct Scene {
    pub entities: IndexMap<String, Box<SceneEntity>>,
}

impl Scene {
    pub fn new() -> Scene {
        Self {
            entities: IndexMap::new(),
        }
    }

    pub fn apply_config(&mut self, config: &SceneConfig) {
        // remove old entities
        // add new entities
        // apply inputs
        // apply bindings
    }
}
