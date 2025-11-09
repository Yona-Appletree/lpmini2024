//! Dynamic map value handling.

use lp_pool::collections::{LpBTreeMap, LpString};
use lp_pool::error::AllocError;

use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;
use crate::value::RuntimeError;

/// Dynamic map value storage (runtime-created records).
pub struct MapValue {
    pub shape: ShapeRef,
    pub fields: LpBTreeMap<LpString, crate::value::LpValue>,
}

impl MapValue {
    /// Create a new empty map.
    pub fn try_new() -> Result<Self, AllocError> {
        use crate::shape::map::StaticMapShape;
        use crate::shape::shape_ref::{MapShapeRef, ShapeRef};

        // Create a static map shape (maps don't have additional metadata)
        static MAP_SHAPE: StaticMapShape = StaticMapShape;
        let shape = ShapeRef::Map(MapShapeRef::Static(&MAP_SHAPE));

        Ok(Self {
            shape,
            fields: LpBTreeMap::new(),
        })
    }

    /// Get a field by name.
    pub fn get_field(&self, name: &str) -> Result<&crate::value::LpValue, RuntimeError> {
        let key = LpString::try_from_str(name)
            .map_err(|_| RuntimeError::AllocError(AllocError::PoolExhausted))?;
        self.fields
            .get(&key)
            .ok_or_else(|| RuntimeError::FieldNotFound {
                record_name: "Map",
                field_name: "", // Runtime field names can't be stored in static error
            })
    }

    /// Get a mutable field by name.
    pub fn get_field_mut(
        &mut self,
        name: &str,
    ) -> Result<&mut crate::value::LpValue, RuntimeError> {
        let key = LpString::try_from_str(name)
            .map_err(|_| RuntimeError::AllocError(AllocError::PoolExhausted))?;
        self.fields
            .get_mut(&key)
            .ok_or_else(|| RuntimeError::FieldNotFound {
                record_name: "Map",
                field_name: "", // Runtime field names can't be stored in static error
            })
    }

    /// Insert or update a field.
    pub fn try_insert_field(
        &mut self,
        name: &str,
        value: crate::value::LpValue,
    ) -> Result<Option<crate::value::LpValue>, RuntimeError> {
        let key = LpString::try_from_str(name)
            .map_err(|_| RuntimeError::AllocError(AllocError::PoolExhausted))?;
        self.fields
            .try_insert(key, value)
            .map_err(RuntimeError::AllocError)
    }

    /// Remove a field.
    pub fn try_remove_field(
        &mut self,
        name: &str,
    ) -> Result<Option<crate::value::LpValue>, RuntimeError> {
        let key = LpString::try_from_str(name)
            .map_err(|_| RuntimeError::AllocError(AllocError::PoolExhausted))?;
        self.fields
            .try_remove(&key)
            .map_err(RuntimeError::AllocError)
    }

    /// Set a field value (insert or update).
    pub fn try_set_field(
        &mut self,
        name: &str,
        value: crate::value::LpValue,
    ) -> Result<(), RuntimeError> {
        self.try_insert_field(name, value)?;
        Ok(())
    }

    /// Check if a field exists.
    pub fn has_field(&self, name: &str) -> bool {
        if let Ok(key) = LpString::try_from_str(name) {
            self.fields.contains_key(&key)
        } else {
            false
        }
    }

    /// Get the number of fields.
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Check if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

impl LpValueTrait for MapValue {
    fn shape(&self) -> &ShapeRef {
        &self.shape
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Map
    }
}

impl core::fmt::Debug for MapValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MapValue")
            .field("shape", &self.shape)
            .field("len", &self.fields.len())
            .finish()
    }
}

impl crate::shape::value::MapValue for MapValue {
    fn get_field(&self, name: &str) -> Result<&dyn LpValueTrait, RuntimeError> {
        let value = MapValue::get_field(self, name)?;
        Ok(value as &dyn LpValueTrait)
    }

    fn get_field_mut(&mut self, name: &str) -> Result<&mut dyn LpValueTrait, RuntimeError> {
        let value = MapValue::get_field_mut(self, name)?;
        Ok(value as &mut dyn LpValueTrait)
    }

    fn set_field(&mut self, name: &str, value: crate::value::LpValue) -> Result<(), RuntimeError> {
        self.try_set_field(name, value)
    }

    fn has_field(&self, name: &str) -> bool {
        MapValue::has_field(self, name)
    }

    fn len(&self) -> usize {
        MapValue::len(self)
    }
}
