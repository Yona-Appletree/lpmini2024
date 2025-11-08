//! Static registry for compile-time known shapes.

#[cfg(feature = "alloc")]
use alloc::collections::BTreeMap;

use crate::shape::shape_ref::ShapeRef;

/// Registry for static shapes (compile-time known types).
///
/// This registry uses no allocations and stores `&'static` references.
/// It is populated at compile time via the derive macro.
pub struct StaticRegistry {
    #[cfg(feature = "alloc")]
    shapes: BTreeMap<&'static str, &'static ShapeRef>,
    #[cfg(not(feature = "alloc"))]
    shapes: &'static [(&'static str, &'static ShapeRef)],
}

impl StaticRegistry {
    /// Create a new empty registry.
    #[cfg(feature = "alloc")]
    pub fn new() -> Self {
        Self {
            shapes: BTreeMap::new(),
        }
    }

    /// Create a new empty registry.
    #[cfg(not(feature = "alloc"))]
    pub const fn new() -> Self {
        Self { shapes: &[] }
    }

    /// Register a shape by name.
    #[cfg(feature = "alloc")]
    pub fn register(&mut self, name: &'static str, shape: &'static ShapeRef) {
        self.shapes.insert(name, shape);
    }

    /// Register a shape by name.
    #[cfg(not(feature = "alloc"))]
    pub const fn register(_name: &'static str, _shape: &'static ShapeRef) -> Self {
        // In no_std mode, we can't mutate, so this is a no-op
        // Shapes should be registered via const initialization
        Self::new()
    }

    /// Get a shape by name.
    pub fn get(&self, name: &str) -> Option<&'static ShapeRef> {
        #[cfg(feature = "alloc")]
        {
            self.shapes.get(name).copied()
        }
        #[cfg(not(feature = "alloc"))]
        {
            self.shapes
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, shape)| *shape)
        }
    }

    /// Check if a shape is registered.
    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }
}

#[cfg(feature = "alloc")]
impl Default for StaticRegistry {
    fn default() -> Self {
        Self::new()
    }
}
