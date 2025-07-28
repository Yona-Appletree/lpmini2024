use crate::entity::node_instance::{EntityInstance, UpdateContext};
use crate::values::size_int::SizeInt;
use crate::values::texture_ref::TextureRef;
use schemars::{schema_for, JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::error::Error;

pub struct CircleEntity {}

impl CircleEntity {
    pub fn new() -> Self {
        Self {}
    }
}

impl EntityInstance for CircleEntity {
    fn update(&mut self, context: &dyn UpdateContext) -> Result<JsonValue, Box<dyn Error>> {
        Ok(json!({
            "texture": TextureRef::new(0)
        }))
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
struct Input {
    image_size: SizeInt,
    radius: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    texture: TextureRef,
}
impl Output {
    fn new(texture: TextureRef) -> Self {
        Self { texture }
    }
}

pub fn schema() -> Schema {
    schema_for!(Input)
}
