use crate::expr::Expr;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

struct SceneValue {
    pub base: JsonValue,
    pub current: JsonValue,
    pub bindings: HashMap<String, Expr>,
}

impl SceneValue {
    pub fn new() -> Self {
        Self {
            base: JsonValue::Null,
            current: JsonValue::Null,
            bindings: HashMap::new(),
        }
    }
}
