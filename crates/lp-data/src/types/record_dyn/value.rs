//! Dynamic map value handling.

#[cfg(feature = "alloc")]
use alloc::string::String;

use lp_pool::collections::{LpBTreeMap, LpBox, LpString};
use lp_pool::error::AllocError;

use crate::metadata::LpTypeMeta;
use crate::value::RuntimeError;

/// Dynamic map value storage (runtime-created records).
pub struct MapValue {
    pub map_type: LpBox<LpTypeMeta>,
    pub fields: LpBTreeMap<LpString, crate::value::LpValue>,
}

impl MapValue {
    /// Create a new empty map.
    pub fn try_new() -> Result<Self, AllocError> {
        use crate::metadata::LpType;
        use crate::types::MapType;

        let map_type_meta = LpTypeMeta::new(LpType::Map(MapType::new()));
        let map_type = LpBox::try_new(map_type_meta)?;

        Ok(Self {
            map_type,
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
