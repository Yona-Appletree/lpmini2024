use super::scene_entity::SceneEntity;
use indexmap::IndexMap;

pub struct SceneStore {
    pub frame_counter: u64,
    pub entities: IndexMap<String, SceneEntity>,
}

impl SceneStore {
    pub fn new() -> Self {
        Self {
            frame_counter: 0,
            entities: IndexMap::new(),
        }
    }
}
