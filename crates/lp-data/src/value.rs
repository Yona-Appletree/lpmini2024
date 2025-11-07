use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Fixed32(pub f32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
pub enum LpValue {
    Int32(i32),
    Fixed32(Fixed32),
    Bool(bool),
    Struct(BTreeMap<String, LpValue>),
    Array(Vec<LpValue>),
    Enum { name: String, variant: String },
}

impl LpValue {
    pub fn int32(value: i32) -> Self {
        LpValue::Int32(value)
    }

    pub fn fixed32(value: f32) -> Self {
        LpValue::Fixed32(Fixed32(value))
    }

    pub fn boolean(value: bool) -> Self {
        LpValue::Bool(value)
    }

    pub fn array(values: Vec<LpValue>) -> Self {
        LpValue::Array(values)
    }

    pub fn structure(fields: impl IntoIterator<Item = (impl Into<String>, LpValue)>) -> Self {
        let mut map = BTreeMap::new();
        for (key, value) in fields {
            map.insert(key.into(), value);
        }
        LpValue::Struct(map)
    }

    pub fn enumeration(name: impl Into<String>, variant: impl Into<String>) -> Self {
        LpValue::Enum {
            name: name.into(),
            variant: variant.into(),
        }
    }
}

