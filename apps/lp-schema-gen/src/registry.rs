use lp_data::{LpDescribe, TypeRegistry};

/// Registry wrapper for lp-data types
pub struct SchemaRegistry {
    registry: TypeRegistry,
}

impl SchemaRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            registry: TypeRegistry::new(),
        }
    }

    /// Register a type that implements LpDescribe
    pub fn register<T: LpDescribe>(&mut self) {
        self.registry.register::<T>();
    }

    /// Get the underlying TypeRegistry
    pub fn registry(&self) -> &TypeRegistry {
        &self.registry
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}
