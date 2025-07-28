use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

///
/// An expression that can be evaluated to a value in the context of a scene.
///
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum Expr {
    ///
    /// Evaluates to the value of an entity output path.
    ///
    NodeOutput { node_id: String, path: String },
}
