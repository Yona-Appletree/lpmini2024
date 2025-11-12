//! Dynamic record value implementation.
//!
//! Dynamic record values are created at runtime and store their fields in a collection.
//! This is in contrast to static record values, which are Rust structs that implement
//! `RecordValue` directly via codegen.
//!
//! Uses `LpValueBox` for field storage, which allocates through the global allocator.

use alloc::string::String;
use alloc::vec::Vec;

use crate::kind::record::record_dyn::{RecordFieldDyn, RecordShapeDyn};
use crate::kind::record::record_meta::RecordFieldMetaDyn;
use crate::kind::record::record_value::RecordValue;
use crate::kind::record::RecordShape;
use crate::kind::shape::LpShape;
use crate::kind::value::{LpValue, LpValueBox, LpValueRef, LpValueRefMut};
use crate::value::RuntimeError;

/// Dynamic record value.
///
/// Stores fields as name-value pairs using allocator-aware wrappers.
/// All field values are stored as `LpValueBox`, which respects the lp_alloc limits.
pub struct RecordValueDyn {
    /// The shape of this record.
    shape: RecordShapeDyn,
    /// Fields stored as (name, value) pairs.
    fields: Vec<(String, LpValueBox)>,
}

impl RecordValueDyn {
    /// Create a new empty dynamic record value with the given shape.
    pub fn new(shape: RecordShapeDyn) -> Self {
        Self {
            shape,
            fields: Vec::new(),
        }
    }

    fn static_shape_of(value: &dyn LpValue) -> &'static dyn LpShape {
        // SAFETY: shapes are either static singletons or pool-allocated with 'static lifetime guarantees.
        unsafe { core::mem::transmute::<&dyn LpShape, &'static dyn LpShape>(value.shape()) }
    }

    /// Add a field to this record.
    ///
    /// If a field with the same name already exists, it will be replaced.
    pub fn add_field(&mut self, name: String, value: LpValueBox) -> Result<(), RuntimeError> {
        // Extract the shape reference first (shapes are 'static, so this is safe)
        let shape_ref: &'static dyn LpShape = match &value {
            LpValueBox::Fixed(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Int32(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Bool(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Vec2(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Vec3(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Vec4(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Record(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::EnumUnit(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::EnumStruct(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Array(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Option(boxed) => Self::static_shape_of(boxed.as_ref()),
        };

        // Check if field already exists and replace it
        for (i, (existing_name, existing_value)) in self.fields.iter_mut().enumerate() {
            if existing_name.as_str() == name.as_str() {
                *existing_value = value;
                // Update the shape field as well
                if let Some(shape_field) = self.shape.fields.get_mut(i) {
                    shape_field.shape = shape_ref;
                }
                return Ok(());
            }
        }

        // Add new field to both value and shape
        let field_shape = RecordFieldDyn {
            name: name.clone(),
            shape: shape_ref,
            meta: RecordFieldMetaDyn { docs: None },
        };

        self.shape.fields.push(field_shape);

        self.fields.push((name, value));
        Ok(())
    }

    /// Get the name of this record type.
    pub fn record_name(&self) -> &str {
        self.shape.meta().name()
    }

    /// Get the number of fields in this record value.
    ///
    /// This returns the actual number of fields stored in the value,
    /// which should always match `shape().field_count()` for dynamic records.
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
                            // Swap-remove from fields
                            let ptr = self.fields.as_mut_slice().as_mut_ptr();
                            core::ptr::swap(ptr.add(i), ptr.add(last_idx));

                            // Swap-remove from shape fields
                            let shape_ptr = self.shape.fields.as_mut_slice().as_mut_ptr();
                            core::ptr::swap(shape_ptr.add(i), shape_ptr.add(last_idx));
                        }
                    }
                    let field_len = self.fields.len();
                    self.fields.pop().ok_or(RuntimeError::IndexOutOfBounds {
                        index: i,
                        len: field_len,
                    })?;
                    let shape_len = self.shape.fields.len();
                    self.shape
                        .fields
                        .pop()
                        .ok_or(RuntimeError::IndexOutOfBounds {
                            index: i,
                            len: shape_len,
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
        let len = self.fields.len();
        let (_, field_value) = self
            .fields
            .get(index)
            .ok_or(RuntimeError::IndexOutOfBounds { index, len })?;

        let value_ref = match field_value {
            LpValueBox::Fixed(boxed) => LpValueRef::Fixed(boxed.as_ref()),
            LpValueBox::Int32(boxed) => LpValueRef::Int32(boxed.as_ref()),
            LpValueBox::Bool(boxed) => LpValueRef::Bool(boxed.as_ref()),
            LpValueBox::Vec2(boxed) => LpValueRef::Vec2(boxed.as_ref()),
            LpValueBox::Vec3(boxed) => LpValueRef::Vec3(boxed.as_ref()),
            LpValueBox::Vec4(boxed) => LpValueRef::Vec4(boxed.as_ref()),
            LpValueBox::Record(boxed) => LpValueRef::Record(boxed.as_ref()),
            LpValueBox::EnumUnit(boxed) => LpValueRef::EnumUnit(boxed.as_ref()),
            LpValueBox::EnumStruct(boxed) => LpValueRef::EnumStruct(boxed.as_ref()),
            LpValueBox::Array(boxed) => LpValueRef::Array(boxed.as_ref()),
            LpValueBox::Option(boxed) => LpValueRef::Option(boxed.as_ref()),
        };

        Ok(value_ref)
    }

    fn get_field_by_index_mut(&mut self, index: usize) -> Result<LpValueRefMut<'_>, RuntimeError> {
        let len = self.fields.len();
        let (_, field_value) = self
            .fields
            .get_mut(index)
            .ok_or(RuntimeError::IndexOutOfBounds { index, len })?;

        let value_ref_mut = match field_value {
            LpValueBox::Fixed(boxed) => LpValueRefMut::Fixed(boxed.as_mut()),
            LpValueBox::Int32(boxed) => LpValueRefMut::Int32(boxed.as_mut()),
            LpValueBox::Bool(boxed) => LpValueRefMut::Bool(boxed.as_mut()),
            LpValueBox::Vec2(boxed) => LpValueRefMut::Vec2(boxed.as_mut()),
            LpValueBox::Vec3(boxed) => LpValueRefMut::Vec3(boxed.as_mut()),
            LpValueBox::Vec4(boxed) => LpValueRefMut::Vec4(boxed.as_mut()),
            LpValueBox::Record(boxed) => LpValueRefMut::Record(boxed.as_mut()),
            LpValueBox::EnumUnit(boxed) => LpValueRefMut::EnumUnit(boxed.as_mut()),
            LpValueBox::EnumStruct(boxed) => LpValueRefMut::EnumStruct(boxed.as_mut()),
            LpValueBox::Array(boxed) => LpValueRefMut::Array(boxed.as_mut()),
            LpValueBox::Option(boxed) => LpValueRefMut::Option(boxed.as_mut()),
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
    use lp_alloc::{
        enter_global_alloc_allowance, init_test_allocator, AllocLimitError as AllocError,
    };
    use lp_math::fixed::{Fixed, Vec2, Vec3, Vec4};

    use super::*;
    use crate::kind::record::record_dyn::RecordShapeDyn;
    use crate::kind::record::record_meta::RecordMetaDyn;

    struct TestPool;

    impl TestPool {
        fn run<F, R>(&self, f: F) -> R
        where
            F: FnOnce() -> R,
        {
            let _guard = enter_global_alloc_allowance();
            f()
        }
    }

    fn setup_pool() -> TestPool {
        init_test_allocator();
        TestPool
    }

    #[test]
    fn test_record_value_dyn_new() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("TestRecord".to_string())?;
            let shape = RecordShapeDyn {
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: Vec::new(),
            };
            let record = RecordValueDyn::new(shape);
            assert_eq!(record.field_count(), 0);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_add_field() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("TestRecord".to_string())?;
            let shape = RecordShapeDyn {
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: Vec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            // Create a Fixed value and convert it to LpValueBox
            let fixed_value = Fixed::ZERO;
            let field_name = Ok::<_, AllocError>("value".to_string())?;

            let value_box = LpValueBox::from(fixed_value);
            record
                .add_field(field_name, value_box)
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            assert_eq!(record.field_count(), 1);
            // Shape should match the actual field count
            assert_eq!(
                RecordValue::shape(&record).field_count(),
                1,
                "Shape field count should match actual field count"
            );
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_shape_matches_fields() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("TestRecord".to_string())?;
            let shape = RecordShapeDyn {
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: Vec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            // Add multiple fields
            let field1_name = Ok::<_, AllocError>("field1".to_string())?;
            let field2_name = Ok::<_, AllocError>("field2".to_string())?;
            let value1 = LpValueBox::from(Fixed::ZERO);
            let value2 = LpValueBox::from(Fixed::ZERO);

            record
                .add_field(field1_name, value1)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            record
                .add_field(field2_name, value2)
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            // Shape should match actual fields
            assert_eq!(record.field_count(), 2);
            assert_eq!(
                RecordValue::shape(&record).field_count(),
                2,
                "Shape field count should match actual field count"
            );

            // Shape should have the correct field names
            let shape_ref = RecordValue::shape(&record);
            assert_eq!(shape_ref.get_field(0).unwrap().name(), "field1");
            assert_eq!(shape_ref.get_field(1).unwrap().name(), "field2");

            // Can get fields by name through shape
            assert!(shape_ref.find_field("field1").is_some());
            assert!(shape_ref.find_field("field2").is_some());
            assert!(shape_ref.find_field("nonexistent").is_none());

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_get_field() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("TestRecord".to_string())?;
            let shape = RecordShapeDyn {
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: Vec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let fixed_value = Fixed::ZERO; // Use ZERO for now
            let field_name = Ok::<_, AllocError>("value".to_string())?;

            let value_box = LpValueBox::from(fixed_value);
            record
                .add_field(field_name, value_box)
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            let retrieved = record
                .get_field("value")
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            assert_eq!(
                retrieved.as_lp_value().shape().kind(),
                crate::kind::kind::LpKind::Fixed
            );

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_remove_field() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("TestRecord".to_string())?;
            let shape = RecordShapeDyn {
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: Vec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let fixed_value = Fixed::ZERO;
            let field_name = Ok::<_, AllocError>("value".to_string())?;

            let value_box = LpValueBox::from(fixed_value);
            record
                .add_field(field_name, value_box)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            assert_eq!(record.field_count(), 1);

            record
                .remove_field("value")
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            assert_eq!(record.field_count(), 0);
            // Shape should also be empty
            assert_eq!(
                RecordValue::shape(&record).field_count(),
                0,
                "Shape field count should match actual field count after removal"
            );

            // Try to get removed field - should fail
            assert!(record.get_field("value").is_err());

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_replace_field() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("TestRecord".to_string())?;
            let shape = RecordShapeDyn {
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: Vec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let value1 = Fixed::ZERO;
            let value2 = Fixed::ZERO;
            let field_name = Ok::<_, AllocError>("value".to_string())?;

            let value_box1 = LpValueBox::from(value1);
            record
                .add_field(field_name, value_box1)
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            // Adding again should replace - create new String
            let field_name2 = Ok::<_, AllocError>("value".to_string())?;
            let value_box2 = LpValueBox::from(value2);
            record
                .add_field(field_name2, value_box2)
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            assert_eq!(record.field_count(), 1);
            // Shape should still match after replacement
            assert_eq!(
                RecordValue::shape(&record).field_count(),
                1,
                "Shape field count should match actual field count after replacement"
            );

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_field_not_found() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("TestRecord".to_string())?;
            let shape = RecordShapeDyn {
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: Vec::new(),
            };
            let record = RecordValueDyn::new(shape);

            let result = record.get_field("nonexistent");
            assert!(result.is_err());

            if let Err(RuntimeError::FieldNotFound { field_name, .. }) = result {
                assert_eq!(field_name.as_str(), "nonexistent");
            } else {
                panic!("Expected FieldNotFound error");
            }

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_multiple_fields() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("TestRecord".to_string())?;
            let shape = RecordShapeDyn {
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: Vec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let value1 = Fixed::ZERO;
            let value2 = Fixed::ZERO;
            let value3 = Fixed::ZERO;

            let value_box1 = LpValueBox::from(value1);
            let value_box2 = LpValueBox::from(value2);
            let value_box3 = LpValueBox::from(value3);

            record
                .add_field(Ok::<_, AllocError>("a".to_string())?, value_box1)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            record
                .add_field(Ok::<_, AllocError>("b".to_string())?, value_box2)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            record
                .add_field(Ok::<_, AllocError>("c".to_string())?, value_box3)
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            assert_eq!(record.field_count(), 3);
            // Shape should match
            assert_eq!(
                RecordValue::shape(&record).field_count(),
                3,
                "Shape field count should match actual field count"
            );

            assert!(record.get_field("a").is_ok());
            assert!(record.get_field("b").is_ok());
            assert!(record.get_field("c").is_ok());

            // Verify shape has the correct field names
            let shape_ref = RecordValue::shape(&record);
            assert_eq!(shape_ref.get_field(0).unwrap().name(), "a");
            assert_eq!(shape_ref.get_field(1).unwrap().name(), "b");
            assert_eq!(shape_ref.get_field(2).unwrap().name(), "c");

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_get_field_mut() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("TestRecord".to_string())?;
            let shape = RecordShapeDyn {
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: Vec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            let fixed_value = Fixed::ZERO;
            let field_name = Ok::<_, AllocError>("value".to_string())?;

            let value_box = LpValueBox::from(fixed_value);
            record
                .add_field(field_name, value_box)
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            // Can get mutable reference
            let mut mut_field = record
                .get_field_mut("value")
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            assert_eq!(
                mut_field.as_lp_value_mut().shape().kind(),
                crate::kind::kind::LpKind::Fixed
            );

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_shape() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("MyRecord".to_string())?;
            let shape = RecordShapeDyn {
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: Vec::new(),
            };
            let record = RecordValueDyn::new(shape);

            use crate::kind::record::record_value::RecordValue;
            let record_shape = RecordValue::shape(&record);
            assert_eq!(record_shape.kind(), crate::kind::kind::LpKind::Record);
            assert_eq!(record_shape.meta().name(), "MyRecord");
            assert_eq!(
                record_shape.field_count(),
                0,
                "Empty record should have 0 fields"
            );

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_record_value_dyn_with_all_primitive_types() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("TestRecord".to_string())?;
            let shape = RecordShapeDyn {
                meta: RecordMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                fields: Vec::new(),
            };
            let mut record = RecordValueDyn::new(shape);

            // Add all primitive types
            record
                .add_field(
                    Ok::<_, AllocError>("count".to_string())?,
                    LpValueBox::from(42i32),
                )
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            record
                .add_field(
                    Ok::<_, AllocError>("enabled".to_string())?,
                    LpValueBox::from(true),
                )
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            record
                .add_field(
                    Ok::<_, AllocError>("position".to_string())?,
                    LpValueBox::from(Vec2::new(Fixed::ZERO, Fixed::ZERO)),
                )
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            record
                .add_field(
                    Ok::<_, AllocError>("rotation".to_string())?,
                    LpValueBox::from(Vec3::new(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)),
                )
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            record
                .add_field(
                    Ok::<_, AllocError>("color".to_string())?,
                    LpValueBox::from(Vec4::new(
                        Fixed::ZERO,
                        Fixed::ZERO,
                        Fixed::ZERO,
                        Fixed::ZERO,
                    )),
                )
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            record
                .add_field(
                    Ok::<_, AllocError>("frequency".to_string())?,
                    LpValueBox::from(Fixed::from_i32(42)),
                )
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            // Verify all fields can be retrieved and have correct types
            assert_eq!(
                record
                    .get_field("count")
                    .map_err(|_| AllocError::SoftLimitExceeded)?
                    .as_lp_value()
                    .shape()
                    .kind(),
                crate::kind::kind::LpKind::Int32
            );
            assert_eq!(
                record
                    .get_field("enabled")
                    .map_err(|_| AllocError::SoftLimitExceeded)?
                    .as_lp_value()
                    .shape()
                    .kind(),
                crate::kind::kind::LpKind::Bool
            );
            assert_eq!(
                record
                    .get_field("position")
                    .map_err(|_| AllocError::SoftLimitExceeded)?
                    .as_lp_value()
                    .shape()
                    .kind(),
                crate::kind::kind::LpKind::Vec2
            );
            assert_eq!(
                record
                    .get_field("rotation")
                    .map_err(|_| AllocError::SoftLimitExceeded)?
                    .as_lp_value()
                    .shape()
                    .kind(),
                crate::kind::kind::LpKind::Vec3
            );
            assert_eq!(
                record
                    .get_field("color")
                    .map_err(|_| AllocError::SoftLimitExceeded)?
                    .as_lp_value()
                    .shape()
                    .kind(),
                crate::kind::kind::LpKind::Vec4
            );
            assert_eq!(
                record
                    .get_field("frequency")
                    .map_err(|_| AllocError::SoftLimitExceeded)?
                    .as_lp_value()
                    .shape()
                    .kind(),
                crate::kind::kind::LpKind::Fixed
            );

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}
