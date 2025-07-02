use schemars::{schema_for, JsonSchema, Schema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct SizeInt {
    width: i32,
    height: i32,
}
