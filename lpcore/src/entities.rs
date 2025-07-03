use serde::{Deserialize, Serialize};

pub mod circle;
pub mod lfo;

#[derive(Serialize, Deserialize, Debug)]
pub enum EntityKind {
    Circle,
    Lfo,
}

pub fn create_entity(kind: EntityKind) -> Box<dyn Entity> {
    match kind {
        EntityKind::Circle => Box::new(CircleEntity::new()),
        EntityKind::Lfo => Box::new(LfoEntity::new()),
    }
}
