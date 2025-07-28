use std::error::Error;

use crate::{
    expr::Expr,
    path::JsonPath,
    scene::{scene_config::NodeConfig, scene_node::SceneNode},
};

///
/// Evaluate the input of a node at the given path.
///
/// - Starts by getting the base value from the node's config.
/// - Then applies any input bindings relevant to the path.
/// - Then returns the final value.
///
pub fn eval_input(
    node: SceneNode,
    context: &dyn EvalContext,
    path: &str,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let requested_path = JsonPath::parse(path)?;
    let requested_value = requested_path.get_from(&node.config.input)?;

    let result = node.config.bindings.iter().try_fold(
        requested_value,
        |mut acc, binding| -> Result<serde_json::Value, Box<dyn Error>> {
            let binding_full_path = JsonPath::parse(binding.0)?;

            let binding_sub_path = binding_full_path
                .without_prefix(&requested_path)
                .ok_or_else(|| -> Box<dyn Error> { "Invalid binding path".to_string().into() })?;

            let computed_value = context.eval_expr(binding.1.clone())?;
            binding_sub_path
                .set_in(&mut acc, computed_value)
                .map_err(|e| -> Box<dyn Error> { e })?;
            Ok(acc)
        },
    )?;

    Ok(result)
}

pub trait EvalContext {
    fn eval_expr(&self, expr: Expr) -> Result<serde_json::Value, Box<dyn Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::entity_instance::EntityInstance;
    use std::collections::HashMap;

    struct MockEntityInstance;

    impl EntityInstance for MockEntityInstance {
        fn update(
            &mut self,
            _context: &dyn crate::entity::entity_instance::UpdateContext,
        ) -> Result<serde_json::Value, Box<dyn Error>> {
            Ok(serde_json::json!(null))
        }
    }

    struct MockEvalContext {
        values: HashMap<String, serde_json::Value>,
    }

    impl EvalContext for MockEvalContext {
        fn eval_expr(&self, expr: Expr) -> Result<serde_json::Value, Box<dyn Error>> {
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
        let mut bindings = HashMap::new();
        bindings.insert(
            "position.x".to_string(),
            Expr::NodeOutput {
                node_id: "lfo1".to_string(),
                path: "value".to_string(),
            },
        );

        let node = SceneNode {
            last_updated_frame: None,
            config: NodeConfig {
                entity_id: "builtin:circle".to_string(),
                input: serde_json::json!({
                    "position": {
                        "x": 0.0,
                        "y": 0.0
                    }
                }),
                bindings,
            },
            instance: Box::new(MockEntityInstance),
            current_input: serde_json::json!(null),
            current_output: serde_json::json!(null),
            input_bindings: HashMap::new(),
        };

        let mut context_values = HashMap::new();
        context_values.insert("lfo1/value".to_string(), serde_json::json!(10.0));
        let context = MockEvalContext {
            values: context_values,
        };

        let result = eval_input(node, &context, "position.x")?;
        assert_eq!(result, serde_json::json!(10.0));
        Ok(())
    }

    #[test]
    fn test_eval_input_no_binding() -> Result<(), Box<dyn Error>> {
        let node = SceneNode {
            last_updated_frame: None,
            config: NodeConfig {
                entity_id: "builtin:circle".to_string(),
                input: serde_json::json!({
                    "position": {
                        "x": 5.0,
                        "y": 0.0
                    }
                }),
                bindings: HashMap::new(),
            },
            instance: Box::new(MockEntityInstance),
            current_input: serde_json::json!(null),
            current_output: serde_json::json!(null),
            input_bindings: HashMap::new(),
        };

        let context = MockEvalContext {
            values: HashMap::new(),
        };

        let result = eval_input(node, &context, "position.x")?;
        assert_eq!(result, serde_json::json!(5.0));
        Ok(())
    }

    #[test]
    fn test_eval_input_invalid_path() -> Result<(), Box<dyn Error>> {
        let node = SceneNode {
            last_updated_frame: None,
            config: NodeConfig {
                entity_id: "builtin:circle".to_string(),
                input: serde_json::json!({
                    "position": {
                        "x": 5.0,
                        "y": 0.0
                    }
                }),
                bindings: HashMap::new(),
            },
            instance: Box::new(MockEntityInstance),
            current_input: serde_json::json!(null),
            current_output: serde_json::json!(null),
            input_bindings: HashMap::new(),
        };

        let context = MockEvalContext {
            values: HashMap::new(),
        };

        let result = eval_input(node, &context, "position.z");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_eval_input_invalid_binding() -> Result<(), Box<dyn Error>> {
        let mut bindings = HashMap::new();
        bindings.insert(
            "position.x.invalid".to_string(),
            Expr::NodeOutput {
                node_id: "lfo1".to_string(),
                path: "value".to_string(),
            },
        );

        let node = SceneNode {
            last_updated_frame: None,
            config: NodeConfig {
                entity_id: "builtin:circle".to_string(),
                input: serde_json::json!({
                    "position": {
                        "x": 0.0,
                        "y": 0.0
                    }
                }),
                bindings,
            },
            instance: Box::new(MockEntityInstance),
            current_input: serde_json::json!(null),
            current_output: serde_json::json!(null),
            input_bindings: HashMap::new(),
        };

        let context = MockEvalContext {
            values: HashMap::new(),
        };

        let result = eval_input(node, &context, "position.x");
        assert!(result.is_err());
        Ok(())
    }
}
