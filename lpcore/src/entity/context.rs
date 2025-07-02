use serde_json::Value as JsonValue;

pub struct FrameInfo {
    pub frame_counter: u64,
    pub now_ms: f64,
}

pub trait Context {
    fn frame_info(&self) -> FrameInfo;

    /// Get the output of an entity by its ID, if available.
    fn entity_output(&self, entity_id: &str) -> Option<JsonValue>;

    /// Compute input for this entity at the given path.
    fn input(&self) -> Option<JsonValue>;
    fn input_path(&self, path: &str) -> Option<JsonValue>;
}
