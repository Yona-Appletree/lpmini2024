use std::error::Error;

use serde_json::Value as JsonValue;

use crate::expr::Expr;
use crate::scene::context::FrameInfo;

/// Trait for runtime instances of entities.
///
/// This trait defines how an entity interacts with its consumers, like a Scene.
pub trait EntityInstance {
    fn update(&mut self, context: &dyn UpdateContext) -> Result<JsonValue, Box<dyn Error>>;

    /// Save the state of this instance to a JSON value
    fn save_state(&self) -> Option<JsonValue> {
        None
    }

    /// Load the state of this instance from a JSON value
    fn load_state(&mut self, _state: JsonValue) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    /// Called before the instance is destroyed
    fn before_destroy(&mut self) {}
}

pub trait UpdateContext {
    /// Get current frame information
    fn frame_info(&self) -> FrameInfo;

    /// Get the output of a node by its ID, if available.
    fn get_node_output(&self, node_id: &str) -> Option<JsonValue>;

    /// Evaluate input at the given path.
    fn eval_input(&self, path: &str) -> Result<JsonValue, Box<dyn Error>>;

    /// Evaluate expression in the input context
    fn eval_expr(&self, expr: Expr) -> Result<JsonValue, Box<dyn Error>>;
}
