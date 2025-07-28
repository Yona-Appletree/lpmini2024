use std::error::Error;

use serde_json;

///
/// Evaluate a path against a JSON value.
///
/// The path is a list of path elements, which can be either a property name or an index.
///
/// The value is the JSON value to evaluate the path against.
///
pub fn eval_path(
    path: &[PathElem],
    obj: &serde_json::Value,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let mut current = obj;

    for elem in path {
        match elem {
            PathElem::Prop(prop) => {
                current = current
                    .get(prop)
                    .ok_or(format!("Property '{}' not found", prop))?;
            }
            PathElem::Index(index) => {
                current = current
                    .get(index)
                    .ok_or(format!("Index '{}' not found", index))?;
            }
        }
    }

    Ok(current.clone())
}

///
/// A path element is either a property name or an index.
///
#[derive(Debug)]
enum PathElem {
    Prop(String),
    Index(usize),
}

///
/// Parse a path into a list of path elements.
///
/// The path is a string of dot-separated property names and indices.
///
/// The path elements are either a property name or an index.
///
pub fn parse_path(path: &str) -> Result<Vec<PathElem>, Box<dyn Error>> {
    let parts: Vec<&str> = path.split('.').collect();
    Ok(parts
        .iter()
        .map(|s| {
            if let Ok(index) = s.parse::<usize>() {
                PathElem::Index(index)
            } else {
                PathElem::Prop(s.to_string())
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_path() {
        let path = "users.0.name";
        let elements = parse_path(path).unwrap();
        assert_eq!(elements.len(), 3);
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

        let path = parse_path("users.0.name").unwrap();
        let result = eval_path(&path, &data).unwrap();
        assert_eq!(result, json!("Alice"));

        let path = parse_path("users.0.age").unwrap();
        let result = eval_path(&path, &data).unwrap();
        assert_eq!(result, json!(30));
    }

    #[test]
    fn test_invalid_path() {
        let data = json!({ "x": 1 });

        let path = parse_path("y").unwrap();
        assert!(eval_path(&path, &data).is_err());

        let path = parse_path("x.0").unwrap();
        assert!(eval_path(&path, &data).is_err());
    }
}
