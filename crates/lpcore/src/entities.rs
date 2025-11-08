use core::error::Error;

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

impl EntityKind {
    pub fn parse_str(s: &str) -> Result<Self, Box<dyn Error>> {
        match s {
            "circle" => Ok(EntityKind::Circle),
            "lfo" => Ok(EntityKind::Lfo),
            _ => Err(format!("Unknown entity kind: {}", s).into()),
        }
    }
}

pub fn create_entity(kind: EntityKind) -> Box<dyn EntityInstance> {
    match kind {
        EntityKind::Circle => Box::new(CircleEntity::new()),
        EntityKind::Lfo => Box::new(LfoEntity::new()),
    }
}
