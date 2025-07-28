use crate::entities::circle::CircleEntity;
use crate::entities::lfo::LfoEntity;
use crate::entity::entity_instance::EntityInstance;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod circle;
pub mod lfo;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum EntityKind {
    Circle,
    Lfo,
}

pub fn create_entity(kind: EntityKind) -> Box<dyn EntityInstance> {
    match kind {
        EntityKind::Circle => Box::new(CircleEntity::new()),
        EntityKind::Lfo => Box::new(LfoEntity::new()),
    }
}
