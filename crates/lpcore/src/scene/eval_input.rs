use std::collections::HashMap;
use std::error::Error;

use crate::expr::{Expr, ExprEvaluator};
use crate::path::JsonPath;

/// Evaluate the input of a node at the given path.
///
/// - Starts by getting the base value from the initial value.
/// - Then applies any input bindings relevant to the path.
/// - Then returns the final value.
#[allow(dead_code)]
pub fn eval_input(
    initial_value: serde_json::Value,
    bindings: &HashMap<JsonPath, Expr>,
    context: &dyn ExprEvaluator,
    path: &JsonPath,
) -> Result<serde_json::Value, Box<dyn Error>> {
    // Try to get the requested value, or use an empty object if the path doesn't exist
    // but there are bindings that would populate it
    let requested_value = match path.get_from(&initial_value) {
        Ok(value) => value,
        Err(_) => {
            // Only create a default value if there are bindings for this path
            let has_relevant_bindings = bindings.keys().any(|bp| bp.without_prefix(path).is_some());
            if has_relevant_bindings {
                serde_json::json!({})
            } else {
                // No bindings, so the path is truly invalid
                return Err("Path not found and no bindings to create it".into());
            }
        }
    };

    let result = bindings.iter().try_fold(
        requested_value,
        |mut acc, (binding_path, expr)| -> Result<serde_json::Value, Box<dyn Error>> {
            if let Some(binding_sub_path) = binding_path.without_prefix(path) {
                // Apply the binding
                let computed_value = context.eval_expr(expr)?;

                binding_sub_path
                    .set_in(&mut acc, computed_value)
                    .map_err(|e| -> Box<dyn Error> { e })?;
            }

            Ok(acc)
        },
    )?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    /// Helper function to create a binding from input path to a node's output
    fn bind(input_path: &str, node_id: &str, output_path: &str) -> (JsonPath, Expr) {
        (
            JsonPath::parse(input_path).unwrap(),
            Expr::NodeOutput {
                node_id: node_id.to_string(),
                path: output_path.to_string(),
            },
        )
    }

    struct MockEvalContext {
        values: HashMap<String, serde_json::Value>,
        eval_count: std::cell::RefCell<usize>,
    }

    impl MockEvalContext {
        fn new() -> Self {
            Self {
                values: HashMap::new(),
                eval_count: std::cell::RefCell::new(0),
            }
        }

        fn with_value(node_id: &str, path: &str, value: serde_json::Value) -> Self {
            let mut values = HashMap::new();
            values.insert(format!("{}/{}", node_id, path), value);
            Self {
                values,
                eval_count: std::cell::RefCell::new(0),
            }
        }

        fn get_eval_count(&self) -> usize {
            *self.eval_count.borrow()
        }
    }

    impl ExprEvaluator for MockEvalContext {
        fn eval_expr(&self, expr: &Expr) -> Result<serde_json::Value, Box<dyn Error>> {
            *self.eval_count.borrow_mut() += 1;
            match expr {
                Expr::NodeOutput { node_id, path } => {
                    let key = format!("{}/{}", node_id, path);
                    self.values
                        .get(&key)
                        .cloned()
                        .ok_or_else(|| -> Box<dyn Error> { format!("No value for {}", key).into() })
                }
            }
        }
    }

    #[test]
    fn test_eval_input_basic() -> Result<(), Box<dyn Error>> {
        let path = JsonPath::parse("position.x")?;
        let bindings = HashMap::from([bind("position.x", "lfo1", "value")]);

        let initial_value = serde_json::json!({
            "position": {
                "x": 0.0,
                "y": 0.0
            }
        });

        let context = MockEvalContext::with_value("lfo1", "value", serde_json::json!(10.0));

        let result = eval_input(initial_value, &bindings, &context, &path)?;
        assert_eq!(result, serde_json::json!(10.0));
        assert_eq!(context.get_eval_count(), 1);
        Ok(())
    }

    #[test]
    fn test_eval_input_no_binding() -> Result<(), Box<dyn Error>> {
        let bindings = HashMap::new();
        let path = JsonPath::parse("position.x")?;

        let initial_value = serde_json::json!({
            "position": {
                "x": 5.0,
                "y": 0.0
            }
        });

        let context = MockEvalContext::new();

        let result = eval_input(initial_value, &bindings, &context, &path)?;
        assert_eq!(result, serde_json::json!(5.0));
        assert_eq!(context.get_eval_count(), 0);
        Ok(())
    }

    #[test]
    fn test_eval_input_invalid_path() -> Result<(), Box<dyn Error>> {
        let bindings = HashMap::new();
        let path = JsonPath::parse("position.z")?;

        let initial_value = serde_json::json!({
            "position": {
                "x": 5.0,
                "y": 0.0
            }
        });

        let context = MockEvalContext::new();

        let result = eval_input(initial_value, &bindings, &context, &path);
        assert!(result.is_err());
        assert_eq!(context.get_eval_count(), 0);
        Ok(())
    }

    #[test]
    fn test_eval_input_creates_path() -> Result<(), Box<dyn Error>> {
        let path = JsonPath::parse("position")?;
        let bindings = HashMap::from([bind("position.x", "lfo1", "value")]);

        let initial_value = serde_json::json!({});

        let context = MockEvalContext::with_value("lfo1", "value", serde_json::json!(10.0));

        let result = eval_input(initial_value, &bindings, &context, &path)?;
        assert_eq!(result, serde_json::json!({ "x": 10.0 }));
        Ok(())
    }

    #[test]
    fn test_eval_input_ignores_unrelated_bindings() -> Result<(), Box<dyn Error>> {
        let path = JsonPath::parse("position.x")?;
        let bindings = HashMap::from([
            bind("position.x", "lfo1", "value"),
            bind("position.y", "lfo2", "value"),
            bind("rotation", "lfo3", "value"),
        ]);

        let initial_value = serde_json::json!({
            "position": {
                "x": 0.0,
                "y": 0.0
            },
            "rotation": 0.0
        });

        let context = MockEvalContext::with_value("lfo1", "value", serde_json::json!(10.0));

        let result = eval_input(initial_value, &bindings, &context, &path)?;

        // Verify that:
        // 1. The correct value was computed
        assert_eq!(result, serde_json::json!(10.0));
        // 2. Only one expression was evaluated (the one for position.x)
        assert_eq!(context.get_eval_count(), 1);

        Ok(())
    }
}
