use lpcore::entities::{lfo, EntityKind};
use lpcore::scene::scene_config::{NodeConfig, NodeKind, SceneConfig};
use serde_json;
use std::collections::HashMap;

fn main() {
    let schema = lfo::schema();
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}

fn example_scene() {
    let scene_config = SceneConfig {
        nodes: HashMap::from([
            (
                "lfo".to_string(),
                NodeConfig {
                    kind: NodeKind::Entity(EntityKind::Lfo),
                    input: serde_json::Value::Null,
                    bindings: HashMap::new(),
                },
            ),
            (
                "circle".to_string(),
                NodeConfig {
                    kind: NodeKind::Entity(EntityKind::Circle),
                    input: serde_json::Value::Null,
                    bindings: HashMap::from([(
                        "radius".to_string(),
                        Expr::Output {
                            entity_id: "lfo".to_string(),
                            path: "values".to_string(),
                        },
                    )]),
                },
            ),
        ]),
        modules: HashMap::new(),
    };
}
