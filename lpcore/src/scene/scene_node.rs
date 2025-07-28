use crate::entity::entity_instance::EntityInstance;
use crate::expr::Expr;
use crate::scene::NodeConfig;
use std::collections::HashMap;

use serde_json::Value as JsonValue;

///
/// An entity instance in a scene
///
pub struct SceneNode {
    pub last_updated_frame: Option<u64>,

    pub config: NodeConfig,
    pub instance: Box<dyn EntityInstance>,

    pub current_input: JsonValue,
    pub current_output: JsonValue,

    pub input_bindings: HashMap<String, Expr>,
}
