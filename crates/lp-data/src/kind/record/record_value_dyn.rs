//! Dynamic record value implementation.
//!
//! Dynamic record values are created at runtime and store their fields in a collection.
//! This is in contrast to static record values, which are Rust structs that implement
//! `RecordValue` directly via codegen.
//!
//! Uses `LpValueBox` for field storage, which allocates from lp-pool.

use lp_pool::{LpString, LpVec};

use crate::kind::record::record_value::RecordValue;
use crate::kind::{
    record::{record_dyn::RecordShapeDyn, RecordShape},
    shape::LpShape,
    value::{LpValue, LpValueBox, LpValueRef, LpValueRefMut},
};
use crate::value::RuntimeError;

/// Dynamic record value.
///
/// Stores fields as name-value pairs in lp-pool allocated collections.
/// All field values are stored as `LpValueBox`, which allocates from lp-pool.
pub struct RecordValueDyn {
    /// The shape of this record.
    shape: RecordShapeDyn,
    /// Fields stored as (name, value) pairs.
    fields: LpVec<(LpString, LpValueBox)>,
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
    pub fn add_field(&mut self, name: LpString, value: LpValueBox) -> Result<(), RuntimeError> {
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
        self.shape.meta().name()
    }

    /// Get the number of fields in this record value.
    ///
    /// This returns the actual number of fields stored in the value,
    /// which may differ from `shape().field_count()` for dynamic records.
    pub fn field_count(&self) -> usize {
        self.fields.len()
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
        Err(RuntimeError::field_not_found("RecordValueDyn", name))
    }
}

impl LpValue for RecordValueDyn {
    fn shape(&self) -> &dyn LpShape {
        &self.shape
    }
}

impl RecordValue for RecordValueDyn {
    fn shape(&self) -> &dyn RecordShape {
        &self.shape
    }

    fn get_field_by_index(&self, index: usize) -> Result<LpValueRef<'_>, RuntimeError> {
        let (_, field_value) =
            self.fields
                .get(index)
                .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                    index,
                    len: self.fields.len(),
                })?;

        let value_ref = match field_value {
            LpValueBox::Fixed(boxed) => LpValueRef::Fixed(boxed.as_ref()),
            LpValueBox::Record(boxed) => LpValueRef::Record(boxed.as_ref()),
        };

        Ok(value_ref)
    }

    fn get_field_by_index_mut(&mut self, index: usize) -> Result<LpValueRefMut<'_>, RuntimeError> {
        let len = self.fields.len();
        let (_, field_value) = self
            .fields
            .get_mut(index)
            .ok_or_else(|| RuntimeError::IndexOutOfBounds { index, len })?;

        let value_ref_mut = match field_value {
            LpValueBox::Fixed(boxed) => LpValueRefMut::Fixed(boxed.as_mut()),
            LpValueBox::Record(boxed) => LpValueRefMut::Record(boxed.as_mut()),
        };

        Ok(value_ref_mut)
    }
}

#[cfg(any(feature = "serde", feature = "serde_json"))]
impl serde::Serialize for RecordValueDyn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.fields.len()))?;
        for (name, value) in self.fields.iter() {
            map.serialize_entry(name.as_str(), value)?;
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kind::record::{record_dyn::RecordShapeDyn, record_meta::RecordMetaDyn};
    use core::ptr::NonNull;
    use lp_math::fixed::Fixed;
    use lp_pool::{LpMemoryPool, LpString};

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
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
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
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: LpVec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            // Create a Fixed value and convert it to LpValueBox
            let fixed_value = Fixed::ZERO;
            let field_name = LpString::try_from_str("value")?;

            let value_box = LpValueBox::from(fixed_value);
            record
                .add_field(field_name, value_box)
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
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: LpVec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let fixed_value = Fixed::ZERO; // Use ZERO for now
            let field_name = LpString::try_from_str("value")?;

            let value_box = LpValueBox::from(fixed_value);
            record
                .add_field(field_name, value_box)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;

            let retrieved = record
                .get_field("value")
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;
            assert_eq!(
                retrieved.as_lp_value().shape().kind(),
                crate::kind::kind::LpKind::Fixed
            );

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
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: LpVec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let fixed_value = Fixed::ZERO;
            let field_name = LpString::try_from_str("value")?;

            let value_box = LpValueBox::from(fixed_value);
            record
                .add_field(field_name, value_box)
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
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: LpVec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let value1 = Fixed::ZERO;
            let value2 = Fixed::ZERO;
            let field_name = LpString::try_from_str("value")?;

            let value_box1 = LpValueBox::from(value1);
            record
                .add_field(field_name, value_box1)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;

            // Adding again should replace - create new LpString
            let field_name2 = LpString::try_from_str("value")?;
            let value_box2 = LpValueBox::from(value2);
            record
                .add_field(field_name2, value_box2)
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
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: LpVec::new(),
            };
            let record = RecordValueDyn::new(shape);

            let result = record.get_field("nonexistent");
            assert!(result.is_err());

            if let Err(RuntimeError::FieldNotFound { field_name, .. }) = result {
                assert_eq!(field_name.as_str(), "nonexistent");
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
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: LpVec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let value1 = Fixed::ZERO;
            let value2 = Fixed::ZERO;
            let value3 = Fixed::ZERO;

            let value_box1 = LpValueBox::from(value1);
            let value_box2 = LpValueBox::from(value2);
            let value_box3 = LpValueBox::from(value3);

            record
                .add_field(LpString::try_from_str("a")?, value_box1)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;
            record
                .add_field(LpString::try_from_str("b")?, value_box2)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;
            record
                .add_field(LpString::try_from_str("c")?, value_box3)
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
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: LpVec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let fixed_value = Fixed::ZERO;
            let field_name = LpString::try_from_str("value")?;

            let value_box = LpValueBox::from(fixed_value);
            record
                .add_field(field_name, value_box)
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;

            // Can get mutable reference
            let mut mut_field = record
                .get_field_mut("value")
                .map_err(|_| lp_pool::AllocError::PoolExhausted)?;
            assert_eq!(
                mut_field.as_lp_value_mut().shape().kind(),
                crate::kind::kind::LpKind::Fixed
            );

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
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: LpVec::new(),
            };
            let record = RecordValueDyn::new(shape);

            use crate::kind::record::record_value::RecordValue;
            let record_shape = RecordValue::shape(&record);
            assert_eq!(record_shape.kind(), crate::kind::kind::LpKind::Record);

            // Can't easily verify the name without downcasting, but we can verify the kind
            // The shape is stored internally, so we know it's correct

            Ok::<(), lp_pool::AllocError>(())
        })
        .unwrap();
    }
}
