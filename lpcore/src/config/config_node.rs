use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum ConfigNode {
    EntityNode {
        _kind: String,

        #[serde(flatten)]
        input: HashMap<String, serde_json::Value>,
    },

    ModuleNode {
        _name: Option<String>,
        _inputs: Option<HashMap<String, InputConfig>>,
        _outputs: Option<HashMap<String, OutputConfig>>,

        #[serde(flatten)]
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
