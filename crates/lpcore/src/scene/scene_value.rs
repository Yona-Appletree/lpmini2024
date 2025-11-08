use crate::expr::Expr;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[allow(dead_code)]
struct SceneValue {
    pub base: JsonValue,
    pub current: JsonValue,
    pub bindings: HashMap<String, Expr>,
}

#[allow(dead_code)]
impl SceneValue {
    pub fn new() -> Self {
        Self {
            base: JsonValue::Null,
            current: JsonValue::Null,
            bindings: HashMap::new(),
        }
    }
}
