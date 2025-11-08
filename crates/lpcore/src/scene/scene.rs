use core::error::Error;

use indexmap::IndexMap;

use crate::entity::entity_id::{EntityId, EntitySource};
use crate::entity::entity_instance::EntityInstance;
use crate::scene::scene_node::SceneNode;
use crate::scene::SceneConfig;

pub struct Scene {
    pub frame_counter: u64,
    pub nodes: IndexMap<String, SceneNode>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            frame_counter: 0,
            nodes: IndexMap::new(),
        }
    }
}

impl Scene {
    pub fn new() -> Scene {
        Self::default()
    }

    #[allow(dead_code)]
    fn create_entity_instance(
        &self,
        id_str: &str,
    ) -> Result<Box<dyn EntityInstance>, Box<dyn Error>> {
        let parsed = EntityId::parse_str(id_str)?;
        match parsed.source {
            EntitySource::BuiltIn => {
                // let instance = EntityInstance::new(entity_id);
                // Ok(Box::new(instance))
                todo!("entity creation")
            }
            EntitySource::Scene => todo!(),
        }
    }

    pub fn apply_config(&mut self, config: &SceneConfig) {
        // remove old nodes
        self.nodes.retain(|id, node| {
            let should_keep = config.nodes.contains_key(id);
            if !should_keep {
                node.instance.before_destroy();
            }
            should_keep
        });

        // add new nodes
        for (id, _node_config) in config.nodes.iter() {
            if !self.nodes.contains_key(id) {
                todo!("entity creation");

                // let instance = EntityInstance::new(node_config);
                // let mut node = SceneNode {
                //     last_updated_frame: None,
                //     config: node_config.clone(),
                //     instance: Box::new(instance),
                //     current_input: serde_json::Value::Null,
                //     current_output: serde_json::Value::Null,
                //     input_bindings: HashMap::new(),
                // };
                // self.nodes.insert(id.clone(), node);
            }
        }

        // apply inputs
        // apply bindings
    }
}
