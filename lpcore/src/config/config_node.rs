use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]

pub enum ConfigNode {
    EntityNode {
        kind: String,
        input: HashMap<String, serde_json::Value>,
    },

    ModuleNode {
        name: Option<String>,
        inputs: Option<HashMap<String, InputConfig>>,
        outputs: Option<HashMap<String, OutputConfig>>,
        entities: HashMap<String, ConfigNode>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputConfig {
    pub name: String,
    pub entity_id: String,
    pub input_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputConfig {
    pub name: String,
    pub entity_id: String,
    pub output_path: String,
}
