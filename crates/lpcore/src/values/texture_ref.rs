use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct TextureRef {
    texture_id: u32,
}

impl TextureRef {
    pub fn new(texture_id: u32) -> Self {
        Self { texture_id }
    }
}
