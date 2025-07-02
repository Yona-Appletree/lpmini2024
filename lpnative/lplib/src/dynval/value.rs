use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type")]
pub enum ValueNode {
    String {
        value: String,
    },
    Int32 {
        value: i32,
    },
    Float64 {
        value: f64,
    },
    Boolean {
        value: bool,
    },
    Record {
        fields: HashMap<String, Box<ValueNode>>,
    },
    Tuple {
        items: Vec<Box<ValueNode>>,
    },
    Array {
        items: Vec<Box<ValueNode>>,
    },
    Enum {
        option: String,
        value: Option<Box<ValueNode>>,
    },
    Option {
        value: Option<Box<ValueNode>>,
    },
    Binary {
        value: Box<[u8]>,
    },
    Texture {
        value: String,
    },
}
