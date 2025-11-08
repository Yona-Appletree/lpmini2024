use super::frame_entity::FrameEntity;
use indexmap::IndexMap;

pub struct FrameStore {
    pub frame_entities: IndexMap<String, FrameEntity>,
}

impl Default for FrameStore {
    fn default() -> Self {
        Self {
            frame_entities: IndexMap::new(),
        }
    }
}

impl FrameStore {
    pub fn new() -> Self {
        Self::default()
    }
}
