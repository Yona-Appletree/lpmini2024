use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct EntityId {
    pub source: EntitySource,
    pub specifier: String,
}

/// Holds a parsed entity id, including the source and specifier
impl EntityId {
    pub fn new(source: EntitySource, specifier: String) -> Self {
        Self { source, specifier }
    }

    pub fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        let id_type = match parts[0] {
            "builtin" => EntitySource::BuiltIn,
            "scene" => EntitySource::Scene,
            _ => panic!("Invalid entity id type: {}", parts[0]),
        };

        Ok(Self {
            source: id_type,
            specifier: parts[1].to_string(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum EntitySource {
    BuiltIn,
    Scene,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_entity_id() {
        let entity_id = EntityId::new(EntitySource::BuiltIn, "test_entity".to_string());
        assert!(matches!(entity_id.source, EntitySource::BuiltIn));
        assert_eq!(entity_id.specifier, "test_entity");
    }

    #[test]
    fn test_from_str_builtin() {
        let entity_id = EntityId::from_str("builtin:test_entity").unwrap();
        assert!(matches!(entity_id.source, EntitySource::BuiltIn));
        assert_eq!(entity_id.specifier, "test_entity");
    }

    #[test]
    fn test_from_str_scene() {
        let entity_id = EntityId::from_str("scene:my_scene").unwrap();
        assert!(matches!(entity_id.source, EntitySource::Scene));
        assert_eq!(entity_id.specifier, "my_scene");
    }

    #[test]
    #[should_panic(expected = "Invalid entity id type: invalid")]
    fn test_from_str_invalid_type() {
        EntityId::from_str("invalid:test_entity").unwrap();
    }
}
