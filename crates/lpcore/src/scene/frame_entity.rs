use serde_json::Value as JsonValue;

pub struct FrameEntity {
    pub input: JsonValue,
    pub output: JsonValue,
}

impl FrameEntity {
    pub fn new() -> Self {
        Self {
            input: JsonValue::Null,
            output: JsonValue::Null,
        }
    }
}
