use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error::Error;

///
/// An expression that can be evaluated to a value in the context of a scene.
///
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub enum Expr {
    ///
    /// Evaluates to the value of an entity output path.
    ///
    NodeOutput { node_id: String, path: String },
}

pub trait ExprEvaluator {
    fn eval_expr(&self, expr: &Expr) -> Result<serde_json::Value, Box<dyn Error>>;
}
