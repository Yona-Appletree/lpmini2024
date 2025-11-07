use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::ty::LpType;

/// Trait for types that can provide their LP schema definition
pub trait LpDataType {
    /// Returns the name of this type
    fn type_name() -> &'static str;

    /// Returns the LP type schema for this type
    fn lp_type() -> LpType;
}

/// Registry for all registered LP data types
pub struct TypeRegistry {
    types: BTreeMap<&'static str, LpType>,
}

impl TypeRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            types: BTreeMap::new(),
        }
    }

    /// Register a type that implements LpDataType
    pub fn register<T: LpDataType>(&mut self) {
        self.types.insert(T::type_name(), T::lp_type());
    }

    /// Register a type with a custom name
    pub fn register_with_name(&mut self, name: &'static str, ty: LpType) {
        self.types.insert(name, ty);
    }

    /// Get a type by name
    pub fn get(&self, name: &str) -> Option<&LpType> {
        self.types.get(name)
    }

    /// Get all registered types
    pub fn all_types(&self) -> &BTreeMap<&'static str, LpType> {
        &self.types
    }

    /// Get all type names
    pub fn type_names(&self) -> Vec<&'static str> {
        self.types.keys().copied().collect()
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

