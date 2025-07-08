use crate::entity::entity::Entity;
use crate::expr::Expr;
use serde_json::value::Value as JsonValue;
use std::collections::HashMap;

pub struct SceneEntity {
    pub last_updated_frame: Option<u64>,
    pub entity: Box<dyn Entity>,
    pub base_input: JsonValue,
    pub bindings: HashMap<String, Expr>,
}
