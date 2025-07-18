use crate::entities::EntityKind;
use crate::expr::Expr;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct SceneConfig {
    pub name: String,
    pub nodes: HashMap<String, NodeConfig>,
    // future: add support for scene-defined nodes
    //pub node_defs: HashMap<String, NodeDef>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct NodeConfig {
    ///
    /// ID of the node definition to use for this node instance
    ///
    /// A URI-style identifier like:
    /// - builtin:lfo
    /// - scene:my_node
    ///  
    pub node_def_id: String,

    /// Raw input values for the node
    pub input: JsonValue,

    /// Map of input path "x.y.z" to expression to evaluate
    pub bindings: HashMap<String, Expr>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct EntityConnection {
    pub output_entity_id: String,
    pub output_path: String,
    pub input_entity_id: String,
    pub input_path: String,
}
