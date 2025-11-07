use lpcore::entities::lfo;
use lpcore::expr::Expr;
use lpcore::scene::scene_config::{NodeConfig, SceneConfig, SceneMeta};
use lpcore::scene::Scene;
use serde_json;
use std::collections::HashMap;

fn main() {
    let schema = lfo::schema();
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}

fn example_scene() {
    let scene_config = SceneConfig {
        meta: SceneMeta { name: None },
        nodes: HashMap::from([
            (
                "lfo".to_string(),
                NodeConfig {
                    entity_id: "builtin:lfo".to_string(),
                    input: serde_json::to_value(lfo::Input {
                        min: 0f64,
                        max: 1f64,
                        period_ms: 1_000,
                        shape: lfo::Shape::Sine,
                    })
                    .unwrap(),
                    bindings: HashMap::new(),
                },
            ),
            (
                "circle".to_string(),
                NodeConfig {
                    entity_id: "builtin:lfo".to_string(),
                    input: serde_json::Value::Null,
                    bindings: HashMap::from([(
                        "radius".to_string(),
                        Expr::NodeOutput {
                            node_id: "lfo".to_string(),
                            path: "".to_string(),
                        },
                    )]),
                },
            ),
        ]),
    };

    let mut scene = Scene::new();
    scene.apply_config(&scene_config);
}
