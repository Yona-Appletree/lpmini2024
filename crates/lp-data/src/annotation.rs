use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Annotations {
    entries: BTreeMap<&'static str, AnnotationValue>,
}

impl Annotations {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    pub fn with(mut self, key: &'static str, value: impl Into<AnnotationValue>) -> Self {
        self.entries.insert(key, value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&AnnotationValue> {
        self.entries.get(key)
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

pub type AnnotationMap = BTreeMap<&'static str, AnnotationValue>;

#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationValue {
    Number(f64),
    Text(String),
    Bool(bool),
    Array(Vec<AnnotationValue>),
    Object(AnnotationMap),
}

impl AnnotationValue {
    pub fn number(value: f64) -> Self {
        AnnotationValue::Number(value)
    }

    pub fn text(value: impl Into<String>) -> Self {
        AnnotationValue::Text(value.into())
    }

    pub fn boolean(value: bool) -> Self {
        AnnotationValue::Bool(value)
    }

    pub fn array(values: Vec<AnnotationValue>) -> Self {
        AnnotationValue::Array(values)
    }

    pub fn object() -> AnnotationObjectBuilder {
        AnnotationObjectBuilder {
            inner: BTreeMap::new(),
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            AnnotationValue::Number(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            AnnotationValue::Text(value) => Some(value.as_str()),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&AnnotationValue> {
        match self {
            AnnotationValue::Object(map) => map.get(key),
            _ => None,
        }
    }

    #[cfg(feature = "serde_json")]
    pub fn to_json(&self) -> serde_json::Value {
        use serde_json::{Map, Number, Value};

        match self {
            AnnotationValue::Number(value) => {
                Value::Number(Number::from_f64(*value).expect("invalid number"))
            }
            AnnotationValue::Text(value) => Value::String(value.clone()),
            AnnotationValue::Bool(value) => Value::Bool(*value),
            AnnotationValue::Array(values) => {
                Value::Array(values.iter().map(AnnotationValue::to_json).collect())
            }
            AnnotationValue::Object(values) => {
                let mut map = Map::new();
                for (key, value) in values {
                    map.insert((*key).to_string(), value.to_json());
                }
                Value::Object(map)
            }
        }
    }
}

impl From<&'static str> for AnnotationValue {
    fn from(value: &'static str) -> Self {
        AnnotationValue::text(value)
    }
}

impl From<String> for AnnotationValue {
    fn from(value: String) -> Self {
        AnnotationValue::Text(value)
    }
}

impl From<f64> for AnnotationValue {
    fn from(value: f64) -> Self {
        AnnotationValue::Number(value)
    }
}

impl From<bool> for AnnotationValue {
    fn from(value: bool) -> Self {
        AnnotationValue::Bool(value)
    }
}

impl From<Vec<AnnotationValue>> for AnnotationValue {
    fn from(value: Vec<AnnotationValue>) -> Self {
        AnnotationValue::Array(value)
    }
}

pub struct AnnotationObjectBuilder {
    inner: AnnotationMap,
}

impl AnnotationObjectBuilder {
    pub fn with(mut self, key: &'static str, value: impl Into<AnnotationValue>) -> Self {
        self.inner.insert(key, value.into());
        self
    }

    pub fn finish(self) -> AnnotationValue {
        AnnotationValue::Object(self.inner)
    }
}

impl From<AnnotationObjectBuilder> for AnnotationValue {
    fn from(builder: AnnotationObjectBuilder) -> Self {
        builder.finish()
    }
}

#[cfg(feature = "serde_json")]
impl Annotations {
    pub fn to_json(&self) -> serde_json::Value {
        use serde_json::Map;

        let mut map = Map::new();
        for (key, value) in &self.entries {
            map.insert((*key).to_string(), value.to_json());
        }
        serde_json::Value::Object(map)
    }
}
