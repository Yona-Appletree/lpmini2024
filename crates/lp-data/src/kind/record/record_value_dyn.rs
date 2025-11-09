//! Dynamic record value implementation.
//!
//! Dynamic record values are created at runtime and store their fields in a collection.
//! This is in contrast to static record values, which are Rust structs that implement
//! `RecordValue` directly via codegen.

extern crate alloc;

use alloc::{string::String, vec::Vec};

use crate::kind::{
    record::record_dyn::RecordShapeDyn,
    shape::LpShape,
    value::{LpValue, LpValueBox, RecordValue, VecIntoIter},
};
use crate::value::RuntimeError;

/// Dynamic record value.
///
/// Stores fields as name-value pairs.
/// All field values are stored as `LpValueBox`.
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

    /// Add a field to this record.
    ///
    /// If a field with the same name already exists, it will be replaced.
    pub fn add_field(&mut self, name: String, value: LpValueBox) -> Result<(), RuntimeError> {
        // Check if field already exists and replace it
        for (existing_name, existing_value) in self.fields.iter_mut() {
            if existing_name == &name {
                *existing_value = value;
                return Ok(());
            }
        }
        // Add new field
        self.fields.push((name, value));
        Ok(())
    }

    /// Get the name of this record type.
    pub fn record_name(&self) -> &str {
        &self.shape.name
    }

    /// Remove a field by name.
    pub fn remove_field(&mut self, name: &str) -> Result<(), RuntimeError> {
        // Find and remove the field
        if let Some(pos) = self
            .fields
            .iter()
            .position(|(field_name, _)| field_name == name)
        {
            self.fields.remove(pos);
            Ok(())
        } else {
            Err(RuntimeError::FieldNotFound {
                record_name: String::from("RecordValueDyn"),
                field_name: String::from(name),
            })
        }
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
            if field_name == name {
                match field_value {
                    LpValueBox::Fixed(boxed) => return Ok(boxed.as_ref()),
                    LpValueBox::Record(boxed) => return Ok(boxed.as_ref()),
                }
            }
        }
        Err(RuntimeError::FieldNotFound {
            record_name: String::from("RecordValueDyn"),
            field_name: String::from(name),
        })
    }

    fn get_field_mut(&mut self, name: &str) -> Result<&mut dyn LpValue, RuntimeError> {
        for (field_name, field_value) in self.fields.iter_mut() {
            if field_name == name {
                match field_value {
                    LpValueBox::Fixed(boxed) => return Ok(boxed.as_mut()),
                    LpValueBox::Record(boxed) => return Ok(boxed.as_mut()),
                }
            }
        }
        Err(RuntimeError::FieldNotFound {
            record_name: String::from("RecordValueDyn"),
            field_name: String::from(name),
        })
    }

    fn set_field(&mut self, _name: &str, _value: &dyn LpValue) -> Result<(), RuntimeError> {
        // For now, we can't easily clone values, so we'll need to handle this differently
        // This is a limitation - we'd need a way to clone or take ownership
        // For now, return an error indicating this isn't fully implemented
        Err(RuntimeError::TypeMismatch {
            expected: String::from("set_field not fully implemented for RecordValueDyn"),
            actual: String::from("use add_field instead"),
        })
    }

    fn field_count(&self) -> usize {
        self.fields.len()
    }

    fn iter_fields(&self) -> VecIntoIter<(String, LpValueBox)> {
        let mut result = Vec::new();
        for (name, value) in self.fields.iter() {
            result.push((name.clone(), value.clone()));
        }
        result.into_iter()
    }

    fn clone_box(&self) -> Box<dyn RecordValue> {
        Box::new(self.clone())
    }
}

impl Clone for RecordValueDyn {
    fn clone(&self) -> Self {
        // Clone the shape
        let cloned_shape = RecordShapeDyn {
            name: self.shape.name.clone(),
            fields: self.shape.fields.clone(),
        };

        // Clone the fields
        let cloned_fields = self
            .fields
            .iter()
            .map(|(n, v)| (n.clone(), v.clone()))
            .collect();

        RecordValueDyn {
            shape: cloned_shape,
            fields: cloned_fields,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kind::record::record_dyn::RecordShapeDyn;
    use lp_math::fixed::Fixed;

    #[test]
    fn test_record_value_dyn_new() {
        let shape = RecordShapeDyn {
            name: String::from("TestRecord"),
            fields: Vec::new(),
        };
        let record = RecordValueDyn::new(shape);
        assert_eq!(record.field_count(), 0);
    }

    #[test]
    fn test_record_value_dyn_add_field() {
        let shape = RecordShapeDyn {
            name: String::from("TestRecord"),
            fields: Vec::new(),
        };
        let mut record = RecordValueDyn::new(shape);

        let fixed_value = Fixed::ZERO;
        let value_box = LpValueBox::from(fixed_value);
        record.add_field(String::from("value"), value_box).unwrap();

        assert_eq!(record.field_count(), 1);
    }

    #[test]
    fn test_record_value_dyn_get_field() {
        let shape = RecordShapeDyn {
            name: String::from("TestRecord"),
            fields: Vec::new(),
        };
        let mut record = RecordValueDyn::new(shape);

        let fixed_value = Fixed::ZERO;
        let value_box = LpValueBox::from(fixed_value);
        record.add_field(String::from("value"), value_box).unwrap();

        let retrieved = record.get_field("value").unwrap();
        assert_eq!(retrieved.shape().kind(), crate::kind::kind::LpKind::Fixed);
    }

    #[test]
    fn test_record_value_dyn_remove_field() {
        let shape = RecordShapeDyn {
            name: String::from("TestRecord"),
            fields: Vec::new(),
        };
        let mut record = RecordValueDyn::new(shape);

        let fixed_value = Fixed::ZERO;
        let value_box = LpValueBox::from(fixed_value);
        record.add_field(String::from("value"), value_box).unwrap();
        assert_eq!(record.field_count(), 1);

        record.remove_field("value").unwrap();
        assert_eq!(record.field_count(), 0);

        assert!(record.get_field("value").is_err());
    }

    #[test]
    fn test_record_value_dyn_replace_field() {
        let shape = RecordShapeDyn {
            name: String::from("TestRecord"),
            fields: Vec::new(),
        };
        let mut record = RecordValueDyn::new(shape);

        let value1 = Fixed::ZERO;
        let value2 = Fixed::ZERO;

        let value_box1 = LpValueBox::from(value1);
        record.add_field(String::from("value"), value_box1).unwrap();

        let value_box2 = LpValueBox::from(value2);
        record.add_field(String::from("value"), value_box2).unwrap();

        assert_eq!(record.field_count(), 1);
    }

    #[test]
    fn test_record_value_dyn_field_not_found() {
        let shape = RecordShapeDyn {
            name: String::from("TestRecord"),
            fields: Vec::new(),
        };
        let record = RecordValueDyn::new(shape);

        let result = record.get_field("nonexistent");
        assert!(result.is_err());

        if let Err(RuntimeError::FieldNotFound { field_name, .. }) = result {
            assert_eq!(field_name, "nonexistent");
        } else {
            panic!("Expected FieldNotFound error");
        }
    }

    #[test]
    fn test_record_value_dyn_multiple_fields() {
        let shape = RecordShapeDyn {
            name: String::from("TestRecord"),
            fields: Vec::new(),
        };
        let mut record = RecordValueDyn::new(shape);

        let value_box1 = LpValueBox::from(Fixed::ZERO);
        let value_box2 = LpValueBox::from(Fixed::ZERO);
        let value_box3 = LpValueBox::from(Fixed::ZERO);

        record.add_field(String::from("a"), value_box1).unwrap();
        record.add_field(String::from("b"), value_box2).unwrap();
        record.add_field(String::from("c"), value_box3).unwrap();

        assert_eq!(record.field_count(), 3);

        assert!(record.get_field("a").is_ok());
        assert!(record.get_field("b").is_ok());
        assert!(record.get_field("c").is_ok());
    }

    #[test]
    fn test_record_value_dyn_get_field_mut() {
        let shape = RecordShapeDyn {
            name: String::from("TestRecord"),
            fields: Vec::new(),
        };
        let mut record = RecordValueDyn::new(shape);

        let fixed_value = Fixed::ZERO;
        let value_box = LpValueBox::from(fixed_value);
        record.add_field(String::from("value"), value_box).unwrap();

        let mut_field = record.get_field_mut("value").unwrap();
        assert_eq!(mut_field.shape().kind(), crate::kind::kind::LpKind::Fixed);
    }

    #[test]
    fn test_record_value_dyn_shape() {
        let shape = RecordShapeDyn {
            name: String::from("MyRecord"),
            fields: Vec::new(),
        };
        let record = RecordValueDyn::new(shape);

        let record_shape = record.shape();
        assert_eq!(record_shape.kind(), crate::kind::kind::LpKind::Record);
    }
}
