//! Dynamic record value implementation.
//!
//! Dynamic record values are created at runtime and store their fields in a collection.
//! This is in contrast to static record values, which are Rust structs that implement
//! `RecordValue` directly via codegen.
//!
//! Uses `LpBoxDyn<dyn LpValue>` for field storage, which allocates from lp-pool.

use lp_pool::{LpBoxDyn, LpString, LpVec};

use crate::kind::{
    record::record_dyn::RecordShapeDyn,
    shape::LpShape,
    value::{LpValue, RecordValue},
};
use crate::value::RuntimeError;

/// Dynamic record value.
///
/// Stores fields as name-value pairs in lp-pool allocated collections.
/// All field values are boxed as `LpBoxDyn<dyn LpValue>`, which allocates from lp-pool.
pub struct RecordValueDyn {
    /// The shape of this record.
    shape: RecordShapeDyn,
    /// Fields stored as (name, value) pairs.
    fields: LpVec<(LpString, LpBoxDyn<dyn LpValue>)>,
}

impl RecordValueDyn {
    /// Create a new empty dynamic record value with the given shape.
    pub fn new(shape: RecordShapeDyn) -> Self {
        Self {
            shape,
            fields: LpVec::new(),
        }
    }

    /// Add a field to this record.
    ///
    /// If a field with the same name already exists, it will be replaced.
    pub fn add_field(
        &mut self,
        name: LpString,
        value: LpBoxDyn<dyn LpValue>,
    ) -> Result<(), RuntimeError> {
        // Check if field already exists and replace it
        for (existing_name, existing_value) in self.fields.iter_mut() {
            if existing_name.as_str() == name.as_str() {
                *existing_value = value;
                return Ok(());
            }
        }
        // Add new field
        self.fields
            .try_push((name, value))
            .map_err(|_| RuntimeError::IndexOutOfBounds { index: 0, len: 0 })
    }

    /// Get the name of this record type.
    pub fn record_name(&self) -> &str {
        self.shape.name.as_str()
    }

    /// Iterate over all fields as (name, value) pairs.
    ///
    /// This allows traversing the record's fields without needing to know field names in advance.
    pub fn iter_fields(&self) -> impl Iterator<Item = (&str, &dyn LpValue)> {
        self.fields
            .iter()
            .map(|(name, value)| (name.as_str(), value.as_ref()))
    }

    /// Remove a field by name.
    pub fn remove_field(&mut self, name: &str) -> Result<(), RuntimeError> {
        // Find and remove the field using swap-remove
        for i in 0..self.fields.len() {
            if let Some((field_name, _)) = self.fields.get(i) {
                if field_name.as_str() == name {
                    let last_idx = self.fields.len() - 1;
                    if i != last_idx {
                        unsafe {
                            let ptr = self.fields.as_mut_slice().as_mut_ptr();
                            core::ptr::swap(ptr.add(i), ptr.add(last_idx));
                        }
                    }
                    self.fields
                        .pop()
                        .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                            index: i,
                            len: self.fields.len(),
                        })?;
                    return Ok(());
                }
            }
        }
        Err(RuntimeError::FieldNotFound {
            #[cfg(feature = "alloc")]
            record_name: alloc::string::String::from("RecordValueDyn"),
            #[cfg(not(feature = "alloc"))]
            record_name: "RecordValueDyn",
            #[cfg(feature = "alloc")]
            field_name: alloc::string::String::from(name),
            #[cfg(not(feature = "alloc"))]
            field_name: name,
        })
    }
}

impl LpValue for RecordValueDyn {
    fn shape(&self) -> &dyn LpShape {
        &self.shape
    }
}

impl RecordValue for RecordValueDyn {
    fn get_field(&self, name: &str) -> Result<&dyn LpValue, RuntimeError> {
        for (field_name, field_value) in self.fields.iter() {
            if field_name.as_str() == name {
                return Ok(field_value.as_ref());
            }
        }
        Err(RuntimeError::FieldNotFound {
            #[cfg(feature = "alloc")]
            record_name: alloc::string::String::from("RecordValueDyn"),
            #[cfg(not(feature = "alloc"))]
            record_name: "RecordValueDyn",
            #[cfg(feature = "alloc")]
            field_name: alloc::string::String::from(name),
            #[cfg(not(feature = "alloc"))]
            field_name: name,
        })
    }

    fn get_field_mut(&mut self, name: &str) -> Result<&mut dyn LpValue, RuntimeError> {
        for (field_name, field_value) in self.fields.iter_mut() {
            if field_name.as_str() == name {
                return Ok(field_value.as_mut());
            }
        }
        Err(RuntimeError::FieldNotFound {
            #[cfg(feature = "alloc")]
            record_name: alloc::string::String::from("RecordValueDyn"),
            #[cfg(not(feature = "alloc"))]
            record_name: "RecordValueDyn",
            #[cfg(feature = "alloc")]
            field_name: alloc::string::String::from(name),
            #[cfg(not(feature = "alloc"))]
            field_name: name,
        })
    }

    fn set_field(&mut self, _name: &str, _value: &dyn LpValue) -> Result<(), RuntimeError> {
        // For now, we can't easily clone values, so we'll need to handle this differently
        // This is a limitation - we'd need a way to clone or take ownership
        // For now, return an error indicating this isn't fully implemented
        Err(RuntimeError::TypeMismatch {
            #[cfg(feature = "alloc")]
            expected: alloc::string::String::from(
                "set_field not fully implemented for RecordValueDyn",
            ),
            #[cfg(not(feature = "alloc"))]
            expected: "set_field not fully implemented",
            #[cfg(feature = "alloc")]
            actual: alloc::string::String::from("use add_field instead"),
            #[cfg(not(feature = "alloc"))]
            actual: "use add_field instead",
        })
    }

    fn field_count(&self) -> usize {
        self.fields.len()
    }
}

impl Clone for RecordValueDyn {
    fn clone(&self) -> Self {
        // Clone the shape
        let cloned_shape = RecordShapeDyn {
            name: self.shape.name.clone(),
            fields: self.shape.fields.clone(),
        };

        // Create new RecordValueDyn with cloned shape
        // NOTE: We can't clone LpBoxDyn<dyn LpValue> generically, so fields are not cloned.
        // This means cloned RecordValueDyn will have empty fields.
        // This is a limitation - in practice, avoid cloning RecordValueDyn if you need the fields.
        RecordValueDyn {
            shape: cloned_shape,
            fields: LpVec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kind::fixed::fixed_static::FIXED_SHAPE;
    use crate::kind::record::record_dyn::RecordShapeDyn;
    use core::ptr::NonNull;
    use lp_math::fixed::Fixed;
    use lp_pool::{allow_global_alloc, LpMemoryPool};

    fn setup_pool() -> LpMemoryPool {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() }
    }

    #[test]
    fn test_record_value_dyn_new() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = LpString::try_from_str("TestRecord")?;
            let shape = RecordShapeDyn {
                name: shape_name,
                fields: LpVec::new(),
            };
            let record = RecordValueDyn::new(shape);
            assert_eq!(record.field_count(), 0);
            Ok::<(), lp_pool::AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_add_field() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = LpString::try_from_str("TestRecord")?;
            let shape = RecordShapeDyn {
                name: shape_name,
                fields: LpVec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            // Create a Fixed value and box it using lp-pool
            let fixed_value = Fixed::ZERO;
            let field_name = LpString::try_from_str("value")?;

            // Box the value using lp-pool
            let trait_ref: &dyn LpValue = &fixed_value;
            let boxed_value = LpBoxDyn::try_new_unsized(trait_ref)?;
            record
                .add_field(field_name, boxed_value)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;

            assert_eq!(record.field_count(), 1);
            Ok::<(), lp_pool::AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_get_field() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = LpString::try_from_str("TestRecord")?;
            let shape = RecordShapeDyn {
                name: shape_name,
                fields: LpVec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let fixed_value = Fixed::ZERO; // Use ZERO for now
            let field_name = LpString::try_from_str("value")?;

            let trait_ref: &dyn LpValue = &fixed_value;
            let boxed_value = LpBoxDyn::try_new_unsized(trait_ref)?;
            record
                .add_field(field_name, boxed_value)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;

            let retrieved = record
                .get_field("value")
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;
            assert_eq!(retrieved.shape().kind(), crate::kind::kind::LpKind::Fixed);

            Ok::<(), lp_pool::AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_remove_field() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = LpString::try_from_str("TestRecord")?;
            let shape = RecordShapeDyn {
                name: shape_name,
                fields: LpVec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let fixed_value = Fixed::ZERO;
            let field_name = LpString::try_from_str("value")?;

            let trait_ref: &dyn LpValue = &fixed_value;
            let boxed_value = LpBoxDyn::try_new_unsized(trait_ref)?;
            record
                .add_field(field_name, boxed_value)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;
            assert_eq!(record.field_count(), 1);

            record
                .remove_field("value")
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;
            assert_eq!(record.field_count(), 0);

            // Try to get removed field - should fail
            assert!(record.get_field("value").is_err());

            Ok::<(), lp_pool::AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_replace_field() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = LpString::try_from_str("TestRecord")?;
            let shape = RecordShapeDyn {
                name: shape_name,
                fields: LpVec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let value1 = Fixed::ZERO;
            let value2 = Fixed::ZERO;
            let field_name = LpString::try_from_str("value")?;

            let trait_ref1: &dyn LpValue = &value1;
            let boxed1 = LpBoxDyn::try_new_unsized(trait_ref1)?;
            // Create a copy of the field name string
            let field_name_copy = LpString::try_from_str("value")?;
            record
                .add_field(field_name_copy, boxed1)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;

            // Adding again should replace - create new LpString
            let field_name2 = LpString::try_from_str("value")?;
            let trait_ref2: &dyn LpValue = &value2;
            let boxed2 = LpBoxDyn::try_new_unsized(trait_ref2)?;
            record
                .add_field(field_name2, boxed2)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;

            assert_eq!(record.field_count(), 1);

            Ok::<(), lp_pool::AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_field_not_found() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = LpString::try_from_str("TestRecord")?;
            let shape = RecordShapeDyn {
                name: shape_name,
                fields: LpVec::new(),
            };
            let record = RecordValueDyn::new(shape);

            let result = record.get_field("nonexistent");
            assert!(result.is_err());

            if let Err(RuntimeError::FieldNotFound { field_name, .. }) = result {
                assert_eq!(field_name, "nonexistent");
            } else {
                panic!("Expected FieldNotFound error");
            }

            Ok::<(), lp_pool::AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_multiple_fields() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = LpString::try_from_str("TestRecord")?;
            let shape = RecordShapeDyn {
                name: shape_name,
                fields: LpVec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let value1 = Fixed::ZERO;
            let value2 = Fixed::ZERO;
            let value3 = Fixed::ZERO;

            let trait_ref1: &dyn LpValue = &value1;
            let boxed1 = LpBoxDyn::try_new_unsized(trait_ref1)?;
            let trait_ref2: &dyn LpValue = &value2;
            let boxed2 = LpBoxDyn::try_new_unsized(trait_ref2)?;
            let trait_ref3: &dyn LpValue = &value3;
            let boxed3 = LpBoxDyn::try_new_unsized(trait_ref3)?;

            record
                .add_field(LpString::try_from_str("a")?, boxed1)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;
            record
                .add_field(LpString::try_from_str("b")?, boxed2)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;
            record
                .add_field(LpString::try_from_str("c")?, boxed3)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;

            assert_eq!(record.field_count(), 3);

            assert!(record.get_field("a").is_ok());
            assert!(record.get_field("b").is_ok());
            assert!(record.get_field("c").is_ok());

            Ok::<(), lp_pool::AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_get_field_mut() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = LpString::try_from_str("TestRecord")?;
            let shape = RecordShapeDyn {
                name: shape_name,
                fields: LpVec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let fixed_value = Fixed::ZERO;
            let field_name = LpString::try_from_str("value")?;

            let trait_ref: &dyn LpValue = &fixed_value;
            let boxed_value = LpBoxDyn::try_new_unsized(trait_ref)?;
            record
                .add_field(field_name, boxed_value)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;

            // Can get mutable reference
            let mut_field = record
                .get_field_mut("value")
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;
            assert_eq!(mut_field.shape().kind(), crate::kind::kind::LpKind::Fixed);

            Ok::<(), lp_pool::AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_shape() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = LpString::try_from_str("MyRecord")?;
            let shape = RecordShapeDyn {
                name: shape_name,
                fields: LpVec::new(),
            };
            let record = RecordValueDyn::new(shape);

            let record_shape = record.shape();
            assert_eq!(record_shape.kind(), crate::kind::kind::LpKind::Record);

            // Can't easily verify the name without downcasting, but we can verify the kind
            // The shape is stored internally, so we know it's correct

            Ok::<(), lp_pool::AllocError>(())
        })
        .unwrap();
    }
}
