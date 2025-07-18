use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    EntityOutput { entity_id: String, path: String },
}
