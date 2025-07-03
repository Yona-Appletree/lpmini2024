use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ExprNode {
    EntityOutput { entity_id: String, path: String },
}
