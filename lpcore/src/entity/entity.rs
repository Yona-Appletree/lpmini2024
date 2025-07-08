use crate::{entities::EntityKind, entity::context::Context};
use serde_json::Value as JsonValue;
use std::error::Error;

pub trait Entity {
    fn kind(&self) -> EntityKind;

    fn update(&mut self, context: &dyn Context) -> Result<JsonValue, Box<dyn Error>>;

    fn get_state(&self) -> Option<JsonValue> {
        None
    }
    fn set_state(&mut self, _state: JsonValue) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
