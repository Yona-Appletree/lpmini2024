use crate::entities::EntityKind;
use crate::expr::Expr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct SceneConfig {
    pub name: String,
    pub module: ModuleConfig,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct NodeConfig {
    pub kind: NodeKind,

    /// Raw input values for the node
    pub input: JsonValue,

    /// Map of input path "x.y.z" to expression to evaluate
    pub bindings: HashMap<String, Expr>,
}

/// Specification of the kind of node
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum NodeKind {
    Entity(EntityKind),
    Module(String),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct ModuleConfig {
    pub name: Option<String>,

    pub inputs: Option<HashMap<String, ModuleInput>>,
    pub outputs: Option<HashMap<String, ModuleOutput>>,

    pub nodes: HashMap<String, NodeConfig>,
    pub modules: HashMap<String, ModuleConfig>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct ModuleInput {
    pub name: String,
    pub entity_id: String,
    pub input_path: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct ModuleOutput {
    pub name: String,
    pub entity_id: String,
    pub output_path: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct EntityConnection {
    pub output_entity_id: String,
    pub output_path: String,
    pub input_entity_id: String,
    pub input_path: String,
}
