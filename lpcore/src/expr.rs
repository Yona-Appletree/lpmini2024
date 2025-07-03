use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Output { entity_id: String, path: String },
}
