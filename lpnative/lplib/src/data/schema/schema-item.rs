use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "$type")]

pub enum SchemaItem {
    String {
        description: Option<String>,
        default: Option<String>,
    },
    Int32 {
        description: Option<String>,
        default: Option<i32>,
        min: Option<i32>,
        max: Option<i32>,
    },
    Float64 {
        description: Option<String>,
        default: Option<f64>,
        min: Option<f64>,
        max: Option<f64>,
    },
    Boolean {
        description: Option<String>,
        default: Option<bool>,
    },
    Record {
        description: Option<String>,
        fields: HashMap<String, SchemaItem>,
    },
    Tuple {
        description: Option<String>,
        items: Vec<SchemaItem>,
    },
    Array {
        description: Option<String>,
        item: Box<SchemaItem>,
    },
    Enum {
        description: Option<String>,
        variants: Vec<String>,
    },
    Option {
        description: Option<String>,
        value: Box<SchemaItem>,
    },
    Image {
        description: Option<String>,
    },
    Texture {
        description: Option<String>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_types_serialization() {
        let string_schema = SchemaItem::String {
            description: Some("A string field".to_string()),
            default: Some("default value".to_string()),
        };
        let json = serde_json::to_string_pretty(&string_schema).unwrap();
        assert_eq!(json, r#"{
  "$type": "String",
  "description": "A string field",
  "default": "default value"
}"#);

        let int_schema = SchemaItem::Int32 {
            description: Some("An integer field".to_string()),
            default: Some(42),
            min: Some(0),
            max: Some(100),
        };
        let json = serde_json::to_string_pretty(&int_schema).unwrap();
        assert_eq!(json, r#"{
  "$type": "Int32",
  "description": "An integer field",
  "default": 42,
  "min": 0,
  "max": 100
}"#);
    }

    #[test]
    fn test_record_serialization() {
        let mut fields = HashMap::new();
        fields.insert("name".to_string(), SchemaItem::String {
            description: Some("The name".to_string()),
            default: None,
        });
        fields.insert("age".to_string(), SchemaItem::Int32 {
            description: Some("The age".to_string()),
            default: None,
            min: Some(0),
            max: None,
        });

        let record = SchemaItem::Record {
            description: Some("A person record".to_string()),
            fields,
        };

        let json = serde_json::to_string_pretty(&record).unwrap();
        // Parse back to verify structure
        let parsed: SchemaItem = serde_json::from_str(&json).unwrap();
        
        if let SchemaItem::Record { fields, .. } = parsed {
            assert_eq!(fields.len(), 2);
            assert!(fields.contains_key("name"));
            assert!(fields.contains_key("age"));
        } else {
            panic!("Failed to parse record schema");
        }
    }

    #[test]
    fn test_tuple_serialization() {
        let tuple = SchemaItem::Tuple {
            description: Some("A coordinate".to_string()),
            items: vec![
                SchemaItem::Float64 {
                    description: Some("x".to_string()),
                    default: None,
                    min: None,
                    max: None,
                },
                SchemaItem::Float64 {
                    description: Some("y".to_string()),
                    default: None,
                    min: None,
                    max: None,
                },
            ],
        };

        let json = serde_json::to_string_pretty(&tuple).unwrap();
        let parsed: SchemaItem = serde_json::from_str(&json).unwrap();
        
        if let SchemaItem::Tuple { items, .. } = parsed {
            assert_eq!(items.len(), 2);
        } else {
            panic!("Failed to parse tuple schema");
        }
    }

    #[test]
    fn test_array_serialization() {
        let array = SchemaItem::Array {
            description: Some("A list of strings".to_string()),
            item: Box::new(SchemaItem::String {
                description: None,
                default: None,
            }),
        };

        let json = serde_json::to_string_pretty(&array).unwrap();
        let parsed: SchemaItem = serde_json::from_str(&json).unwrap();
        
        if let SchemaItem::Array { item, .. } = parsed {
            if let SchemaItem::String { .. } = *item {
                // Successfully parsed array of strings
            } else {
                panic!("Array item type mismatch");
            }
        } else {
            panic!("Failed to parse array schema");
        }
    }

    #[test]
    fn test_option_serialization() {
        let option = SchemaItem::Option {
            description: Some("An optional integer".to_string()),
            value: Box::new(SchemaItem::Int32 {
                description: None,
                default: None,
                min: None,
                max: None,
            }),
        };

        let json = serde_json::to_string_pretty(&option).unwrap();
        let parsed: SchemaItem = serde_json::from_str(&json).unwrap();
        
        if let SchemaItem::Option { value, .. } = parsed {
            if let SchemaItem::Int32 { .. } = *value {
                // Successfully parsed optional int
            } else {
                panic!("Option value type mismatch");
            }
        } else {
            panic!("Failed to parse option schema");
        }
    }
}
