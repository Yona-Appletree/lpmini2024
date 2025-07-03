use crate::expr::expr_node::ExprNode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneDef {
    entities: HashMap<String, Box<EntityDef>>,
    modules: HashMap<String, Box<ModuleDef>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityDef {
    kind: String,
    input_base: serde_json::Value,
    input_bindings: HashMap<String, ExprNode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleDef {
    name: Option<String>,
    inputs: Option<HashMap<String, ModuleInput>>,
    outputs: Option<HashMap<String, ModuleOutput>>,
    entities: HashMap<String, Box<EntityDef>>,
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
