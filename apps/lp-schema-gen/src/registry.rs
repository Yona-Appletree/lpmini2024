use std::collections::BTreeMap;

use lp_data::kind::shape::LpShape;

#[allow(dead_code)]
/// Trait for types that can provide their shape and name
pub trait LpDescribe: lp_data::kind::value::LpValue + Default {
    /// Get the type name
    fn type_name() -> &'static str;
}

#[allow(dead_code)]
/// Registry wrapper for lp-data types
///
/// Stores instances of types so we can get their shapes when needed.
/// The shapes are 'static, so this is safe.
pub struct SchemaRegistry {
    types: Vec<Box<dyn TypeEntry>>,
}

#[allow(dead_code)]
trait TypeEntry: 'static {
    fn name(&self) -> &'static str;
    fn shape(&self) -> &'static dyn LpShape;
}

#[allow(dead_code)]
struct TypeEntryImpl<T: LpDescribe + 'static> {
    instance: T,
}

impl<T: LpDescribe + 'static> TypeEntry for TypeEntryImpl<T> {
    fn name(&self) -> &'static str {
        T::type_name()
    }

    fn shape(&self) -> &'static dyn LpShape {
        // SAFETY: Shapes returned by LpValue::shape() are 'static
        // (they're either static constants or pool-allocated with 'static lifetime)
        unsafe { core::mem::transmute(self.instance.shape()) }
    }
}

impl SchemaRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }

    /// Register a type that implements LpDescribe
    #[allow(dead_code)]
    pub fn register<T: LpDescribe + 'static>(&mut self) {
        let entry = TypeEntryImpl::<T> {
            instance: T::default(),
        };
        self.types.push(Box::new(entry));
    }

    /// Get all registered types as a map of name -> shape
    pub fn all_types(&self) -> BTreeMap<&'static str, &dyn LpShape> {
        let mut map = BTreeMap::new();
        for entry in &self.types {
            map.insert(entry.name(), entry.shape());
        }
        map
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}
