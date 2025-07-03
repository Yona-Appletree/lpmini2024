use crate::data::texture_ref::TextureRef;
use crate::entities::EntityKind;
use crate::entity::context::Context;
use crate::{data::size_int::SizeInt, entity::entity::Entity};
use schemars::{schema_for, JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::error::Error;

struct CircleEntity {}

impl CircleEntity {
    fn new() -> Self {
        Self {}
    }
}

impl Entity for CircleEntity {
    fn kind() -> EntityKind {
        EntityKind::Circle
    }

    fn update(&mut self, context: &dyn Context) -> Result<JsonValue, Box<dyn Error>> {
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
