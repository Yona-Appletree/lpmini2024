use std::collections::BTreeMap;

use schemars::{JsonSchema, Schema};

/// Registry for all registered LP data types using schemars
pub struct SchemaRegistry {
    schemas: BTreeMap<String, Schema>,
}

impl SchemaRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            schemas: BTreeMap::new(),
        }
    }

    /// Register a type that implements JsonSchema
    pub fn register<T: JsonSchema>(&mut self) {
        let schema = schemars::schema_for!(T);
        let name = T::schema_name().to_string();
        self.schemas.insert(name, schema);
    }

    /// Register a type with a custom name
    pub fn register_with_name(&mut self, name: String, schema: Schema) {
        self.schemas.insert(name, schema);
    }

    /// Get a schema by type name
    pub fn get(&self, name: &str) -> Option<&Schema> {
        self.schemas.get(name)
    }

    /// Get all registered type names
    pub fn type_names(&self) -> Vec<String> {
        self.schemas.keys().cloned().collect()
    }

    /// Get all schemas
    pub fn all_schemas(&self) -> &BTreeMap<String, Schema> {
        &self.schemas
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}
