//! Runtime registry for dynamic shapes with fallback to static registry.

use lp_pool::collections::{LpBTreeMap, LpBox, LpString};
use lp_pool::error::AllocError;

use crate::registry::static_registry::StaticRegistry;
use crate::shape::shape_ref::ShapeRef;

/// Registry for runtime shapes with fallback to static registry.
///
/// This registry lives in an LpPool context and first checks its dynamic
/// shapes, then falls back to the static registry.
pub struct RuntimeRegistry {
    static_registry: &'static StaticRegistry,
    dynamic_shapes: LpBTreeMap<LpString, LpBox<ShapeRef>>,
}

impl RuntimeRegistry {
    /// Create a new runtime registry with a reference to the static registry.
    pub fn new(static_registry: &'static StaticRegistry) -> Self {
        Self {
            static_registry,
            dynamic_shapes: LpBTreeMap::new(),
        }
    }

    /// Register a dynamic shape by name.
    pub fn register_dynamic(&mut self, name: &str, shape: ShapeRef) -> Result<(), AllocError> {
        let name_str = LpString::try_from_str(name)?;
        let shape_box = LpBox::try_new(shape)?;
        self.dynamic_shapes
            .try_insert(name_str, shape_box)
            .map_err(|_| AllocError::PoolExhausted)?;
        Ok(())
    }

    /// Get a shape by name, checking dynamic first, then static.
    /// Returns a reference to the shape (static reference for static shapes,
    /// reference to boxed shape for dynamic shapes).
    pub fn get(&self, name: &str) -> Option<&ShapeRef> {
        // First check dynamic shapes
        if let Ok(name_str) = LpString::try_from_str(name) {
            if let Some(shape_box) = self.dynamic_shapes.get(&name_str) {
                return Some(&**shape_box);
            }
        }

        // Fall back to static registry
        self.static_registry.get(name)
    }

    /// Check if a shape is registered (dynamic or static).
    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    /// Get the number of dynamic shapes.
    pub fn dynamic_count(&self) -> usize {
        self.dynamic_shapes.len()
    }
}
