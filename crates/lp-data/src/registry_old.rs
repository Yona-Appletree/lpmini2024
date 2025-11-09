use alloc::collections::BTreeMap;
use alloc::vec::Vec;

// TODO: This old registry system will be removed
// For now, using a placeholder type
pub struct LpTypeMeta; // Placeholder - old system removed

/// Trait implemented by types that can describe their schema.
pub trait LpDescribe {
    /// Canonical name for the described type.
    const TYPE_NAME: &'static str;

    /// Returns the schema shape reference for this type.
    fn lp_schema() -> &'static crate::shape::shape_ref::ShapeRef;
}

/// Entry produced by explicit registration helpers.
pub struct SchemaRegistration {
    pub name: &'static str,
    pub schema: &'static LpTypeMeta,
}

/// Registry for all registered LP data types
pub struct TypeRegistry {
    types: BTreeMap<&'static str, &'static LpTypeMeta>,
}

impl TypeRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            types: BTreeMap::new(),
        }
    }

    /// Register a type that implements LpDescribe
    pub fn register<T: LpDescribe>(&mut self) {
        // Note: This is a placeholder - the old registry system is deprecated
        // The new system uses StaticRegistry and RuntimeRegistry
        let _ = T::TYPE_NAME;
        let _ = T::lp_schema();
    }

    /// Register all descriptors in one call.
    pub fn register_many(&mut self, entries: &[SchemaRegistration]) {
        for entry in entries {
            self.register_with_meta(entry.name, entry.schema);
        }
    }

    /// Register a type with a custom name
    pub fn register_with_meta(&mut self, name: &'static str, schema: &'static LpTypeMeta) {
        self.types.insert(name, schema);
    }

    /// Get a type by name
    pub fn get(&self, name: &str) -> Option<&'static LpTypeMeta> {
        self.types.get(name).copied()
    }

    /// Get all registered types
    pub fn all_types(&self) -> &BTreeMap<&'static str, &'static LpTypeMeta> {
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

/// Explicit helper to register a list of types implementing [`LpDescribe`].
///
/// Keeping this explicit makes call sites obvious today. Once we're confident in
/// the derive story we could move to an automatic registry (e.g. `inventory`).
#[macro_export]
macro_rules! register_lp_schemas {
    ($registry:expr, $( $ty:ty ),+ $(,)?) => {
        $(
            $registry.register::<$ty>();
        )+
    };
}
