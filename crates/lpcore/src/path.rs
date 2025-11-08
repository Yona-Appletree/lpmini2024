use std::{error::Error, hash::Hash};

use serde_json::{self, Value};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsonPath {
    elems: Vec<PathElem>,
}

impl JsonPath {
    pub fn new(elems: Vec<PathElem>) -> Self {
        Self { elems }
    }

    ///
    /// Parse a path into a list of path elements.
    ///
    /// The path is a string of dot-separated property names and indices.
    ///
    /// The path elements are either a property name or an index.
    ///
    pub fn parse(path: &str) -> Result<Self, Box<dyn Error>> {
        let elems = path
            .split('.')
            .filter(|s| !s.is_empty())
            .map(|s| {
                if let Ok(index) = s.parse::<usize>() {
                    PathElem::Index(index)
                } else {
                    PathElem::Prop(s.to_string())
                }
            })
            .collect();
        Ok(Self { elems })
    }

    ///
    /// Get the value at the end of the path from the given object.
    ///
    /// Returns the value at the end of the path.
    ///
    pub fn get_from(&self, obj: &Value) -> Result<Value, Box<dyn Error>> {
        let mut current = obj;

        for elem in &self.elems {
            match elem {
                PathElem::Prop(ref prop) => {
                    current = current
                        .get(prop)
                        .ok_or(format!("Property '{}' not found", prop))?;
                }
                PathElem::Index(index) => {
                    current = current
                        .get(*index)
                        .ok_or(format!("Index '{}' not found", index))?;
                }
            }
        }

        Ok(current.clone())
    }

    ///
    /// Set the value at the end of the path in the given object.
    /// Creates intermediate objects and arrays as needed.
    ///
    pub fn set_in(&self, obj: &mut Value, value: Value) -> Result<(), Box<dyn Error>> {
        if self.elems.is_empty() {
            *obj = value;
            return Ok(());
        }

        let mut current = obj;
        let last_idx = self.elems.len() - 1;

        // Navigate to the parent of the target location, creating intermediate objects as needed
        for elem in self.elems[..last_idx].iter() {
            current = match elem {
                PathElem::Prop(ref prop) => {
                    if !current.is_object() {
                        *current = Value::Object(serde_json::Map::new());
                    }
                    let obj = current.as_object_mut().unwrap();
                    if !obj.contains_key(prop) {
                        obj.insert(prop.clone(), Value::Null);
                    }
                    obj.get_mut(prop).unwrap()
                }
                PathElem::Index(idx) => {
                    if !current.is_array() {
                        *current = Value::Array(vec![Value::Null; idx + 1]);
                    } else {
                        let arr = current.as_array_mut().unwrap();
                        if *idx >= arr.len() {
                            arr.resize(*idx + 1, Value::Null);
                        }
                    }
                    current.as_array_mut().unwrap().get_mut(*idx).unwrap()
                }
            };
        }

        // Set the value at the final location
        match self.elems.last().unwrap() {
            PathElem::Prop(prop) => {
                if !current.is_object() {
                    *current = Value::Object(serde_json::Map::new());
                }
                current.as_object_mut().unwrap().insert(prop.clone(), value);
            }
            PathElem::Index(idx) => {
                if !current.is_array() {
                    *current = Value::Array(vec![Value::Null; idx + 1]);
                } else {
                    let arr = current.as_array_mut().unwrap();
                    if *idx >= arr.len() {
                        arr.resize(*idx + 1, Value::Null);
                    }
                }
                current.as_array_mut().unwrap()[*idx] = value;
            }
        }

        Ok(())
    }

    ///
    /// Removes a path prefix from this path.
    ///
    /// Returns `None` if the prefix is not a prefix of this path.
    ///
    pub fn without_prefix(&self, prefix: &JsonPath) -> Option<JsonPath> {
        // Check if prefix is longer than the path
        if prefix.elems.len() > self.elems.len() {
            return None;
        }

        // Check if all prefix elements match
        if !self
            .elems
            .iter()
            .zip(prefix.elems.iter())
            .all(|(a, b)| a == b)
        {
            return None;
        }

        let without_prefix_elems = self.elems.iter().skip(prefix.elems.len());
        Some(JsonPath {
            elems: without_prefix_elems.cloned().collect(),
        })
    }

    pub fn without_prefix_str(&self, prefix: &str) -> Option<JsonPath> {
        let prefix_path = Self::parse(prefix).ok()?;
        self.without_prefix(&prefix_path)
    }
}

///
/// A path element is either a property name or an index.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathElem {
    Prop(String),
    Index(usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_path() {
        let path = "users.0.name";
        let parsed_path = JsonPath::parse(path).unwrap();
        assert_eq!(parsed_path.elems.len(), 3);
    }

    #[test]
    fn test_eval_path() {
        let data = json!({
            "users": [
                {
                    "name": "Alice",
                    "age": 30
                }
            ]
        });

        let path = JsonPath::parse("users.0.name").unwrap();
        let result = path.get_from(&data).unwrap();
        assert_eq!(result, json!("Alice"));

        let path = JsonPath::parse("users.0.age").unwrap();
        let result = path.get_from(&data).unwrap();
        assert_eq!(result, json!(30));
    }

    #[test]
    fn test_invalid_path() {
        let data = json!({ "x": 1 });

        let path = JsonPath::parse("y").unwrap();
        assert!(path.get_from(&data).is_err());

        let path = JsonPath::parse("x.0").unwrap();
        assert!(path.get_from(&data).is_err());
    }

    #[test]
    fn test_set_in() {
        // Test setting in object
        let mut data = json!({
            "users": [
                {
                    "name": "Alice",
                    "age": 30
                }
            ]
        });

        let path = JsonPath::parse("users.0.age").unwrap();
        path.set_in(&mut data, json!(31)).unwrap();
        assert_eq!(path.get_from(&data).unwrap(), json!(31));

        // Test setting in array
        let mut data = json!([1, 2, 3]);
        let path = JsonPath::parse("1").unwrap();
        path.set_in(&mut data, json!(5)).unwrap();
        assert_eq!(path.get_from(&data).unwrap(), json!(5));

        // Test creating new properties
        let mut data = json!({});
        let path = JsonPath::parse("users.0.name").unwrap();
        path.set_in(&mut data, json!("Alice")).unwrap();
        assert_eq!(
            data,
            json!({
                "users": [
                    {
                        "name": "Alice"
                    }
                ]
            })
        );

        // Test creating array with gaps
        let mut data = json!({});
        let path = JsonPath::parse("values.2").unwrap();
        path.set_in(&mut data, json!(42)).unwrap();
        assert_eq!(
            data,
            json!({
                "values": [null, null, 42]
            })
        );

        // Test creating deep nested structure
        let mut data = json!({});
        let path = JsonPath::parse("a.b.c.0.d.1.e").unwrap();
        path.set_in(&mut data, json!("value")).unwrap();
        assert_eq!(
            data,
            json!({
                "a": {
                    "b": {
                        "c": [
                            {
                                "d": [
                                    null,
                                    {
                                        "e": "value"
                                    }
                                ]
                            }
                        ]
                    }
                }
            })
        );

        // Test empty path
        let mut data = json!({"x": 1});
        let path = JsonPath::parse("").unwrap();
        path.set_in(&mut data, json!({"y": 2})).unwrap();
        assert_eq!(data, json!({"y": 2}));
    }

    #[test]
    fn test_without_prefix() {
        let path = JsonPath::parse("users.0.name.first").unwrap();

        // Test successful prefix removal
        let without_prefix = path.without_prefix_str("users.0").unwrap();
        assert_eq!(without_prefix.elems.len(), 2);

        // Test prefix that doesn't match
        assert!(path.without_prefix_str("users.1").is_none());
        assert!(path.without_prefix_str("admins.0").is_none());

        // Test prefix longer than path
        assert!(path
            .without_prefix_str("users.0.name.first.middle")
            .is_none());

        // Test empty path after prefix
        let path = JsonPath::parse("users.0").unwrap();
        let without_prefix = path.without_prefix_str("users.0").unwrap();
        assert_eq!(without_prefix.elems.len(), 0);

        // Test invalid prefix
        assert!(path.without_prefix_str("users.invalid").is_none());
    }
}
