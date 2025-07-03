use crate::entities::EntityKind;
use crate::expr::Expr;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneConfig {
    /// Map of node id to node config
    pub nodes: HashMap<String, NodeConfig>,

    /// Map of module id to module config
    pub modules: HashMap<String, ModuleConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeConfig {
    pub kind: KindSpec,

    /// Raw input data for the node
    pub input: JsonValue,

    /// Map of input path "x.y.z" to expression to evaluate
    pub bindings: HashMap<String, Expr>,
}

/// Specification of the kind of node
#[derive(Serialize, Deserialize, Debug)]
pub enum KindSpec {
    Entity(EntityKind),
    Module(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleConfig {
    pub name: Option<String>,

    pub inputs: Option<HashMap<String, ModuleInput>>,
    pub outputs: Option<HashMap<String, ModuleOutput>>,

    pub nodes: HashMap<String, NodeConfig>,
    pub modules: HashMap<String, ModuleConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleInput {
    pub name: String,
    pub entity_id: String,
    pub input_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleOutput {
    pub name: String,
    pub entity_id: String,
    pub output_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityConnection {
    pub output_entity_id: String,
    pub output_path: String,
    pub input_entity_id: String,
    pub input_path: String,
}
