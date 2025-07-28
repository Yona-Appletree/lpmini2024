use crate::expr::Expr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct SceneConfig {
    pub name: String,
    pub nodes: HashMap<String, NodeConfig>,
    // future: add support for scene-defined entities
    //pub entity_defs: HashMap<String, EntityDef>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct NodeConfig {
    ///
    /// ID of the entity to be used for this node
    ///
    /// A URI-style identifier like:
    /// - builtin:lfo
    /// - scene:my_node
    ///  
    pub entity_id: String,

    /// Raw input values for the node
    //    pub input: JsonValue,

    /// Map of input path "x.y.z" to expression to evaluate
    pub bindings: HashMap<String, Expr>,
}
