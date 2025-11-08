use std::collections::HashMap;

use serde_json::Value as JsonValue;

use crate::expr::Expr;

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
