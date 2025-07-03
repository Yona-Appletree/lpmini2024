use crate::entity::entity::Entity;
use indexmap::IndexMap;

pub struct Scene {
    pub entities: IndexMap<String, Box<dyn Entity>>,
}

impl Scene {
    pub fn new() -> Scene {
        Self {
            entities: IndexMap::new(),
        }
    }
}
