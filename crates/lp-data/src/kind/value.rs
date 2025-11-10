//! Value system for lp-data types.
//!
//! Values represent runtime data instances that reference their shape.

#[cfg(feature = "alloc")]
extern crate alloc;

use super::shape::LpShape;
use crate::kind::RecordShape;
use crate::value::RuntimeError;
use lp_math::fixed::Fixed;
use lp_pool::{lp_box_dyn, LpBoxDyn};

pub enum LpValueBox {
    Fixed(LpBoxDyn<dyn LpValue>),
    Record(LpBoxDyn<dyn RecordValue>),
}

/// Type-aware reference to a value.
///
/// Preserves type information so that RecordValue methods can be called
/// without downcasting. This is the reference equivalent of LpValueBox.
pub enum LpValueRef<'a> {
    Fixed(&'a dyn LpValue),
    Record(&'a dyn RecordValue),
}

impl<'a> LpValueRef<'a> {
    /// Get a reference to the value as LpValue.
    pub fn as_lp_value(&self) -> &'a dyn LpValue {
        match self {
            LpValueRef::Fixed(v) => *v,
            LpValueRef::Record(v) => *v as &dyn LpValue,
        }
    }

    /// Try to get a reference to the value as RecordValue.
    pub fn as_record(&self) -> Option<&'a dyn RecordValue> {
        match self {
            LpValueRef::Fixed(_) => None,
            LpValueRef::Record(v) => Some(*v),
        }
    }
}

impl<'a> core::ops::Deref for LpValueRef<'a> {
    type Target = dyn LpValue + 'a;

    fn deref(&self) -> &Self::Target {
        self.as_lp_value()
    }
}

/// Type-aware mutable reference to a value.
///
/// Preserves type information so that RecordValue methods can be called
/// without downcasting. This is the mutable reference equivalent of LpValueBox.
pub enum LpValueRefMut<'a> {
    Fixed(&'a mut dyn LpValue),
    Record(&'a mut dyn RecordValue),
}

impl<'a> LpValueRefMut<'a> {
    /// Get a mutable reference to the value as LpValue.
    pub fn as_lp_value_mut(&mut self) -> &mut dyn LpValue {
        match self {
            LpValueRefMut::Fixed(v) => *v,
            LpValueRefMut::Record(v) => *v as &mut dyn LpValue,
        }
    }

    /// Try to get a mutable reference to the value as RecordValue.
    pub fn as_record_mut(&mut self) -> Option<&mut dyn RecordValue> {
        match self {
            LpValueRefMut::Fixed(_) => None,
            LpValueRefMut::Record(v) => Some(*v),
        }
    }
}

impl<'a> core::ops::Deref for LpValueRefMut<'a> {
    type Target = dyn LpValue + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            LpValueRefMut::Fixed(v) => *v,
            LpValueRefMut::Record(v) => *v as &dyn LpValue,
        }
    }
}

impl<'a> core::ops::DerefMut for LpValueRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            LpValueRefMut::Fixed(v) => *v,
            LpValueRefMut::Record(v) => *v as &mut dyn LpValue,
        }
    }
}

impl From<Fixed> for LpValueBox {
    fn from(value: Fixed) -> Self {
        // Box the Fixed value as a trait object
        // Fixed is Copy, so we can move it into pool memory
        let boxed =
            lp_box_dyn!(value, dyn LpValue).expect("Failed to allocate Fixed value in pool");
        LpValueBox::Fixed(boxed)
    }
}

impl From<LpBoxDyn<dyn RecordValue>> for LpValueBox {
    fn from(value: LpBoxDyn<dyn RecordValue>) -> Self {
        LpValueBox::Record(value)
    }
}

/// Base trait for all runtime values.
///
/// Values are concrete instances of data. Rust types like `Fixed` or `LfoConfig`
/// implement this trait directly - they ARE the values, not wrappers.
pub trait LpValue {
    /// Get the shape reference for this value.
    fn shape(&self) -> &dyn LpShape;
}

/// Trait for record values that have fields.
pub trait RecordValue: LpValue {
    fn shape(&self) -> &dyn RecordShape;

    /// Get a field by name (immutable).
    fn get_field(&self, name: &str) -> Result<LpValueRef, RuntimeError>;

    /// Get a field by name (mutable).
    fn get_field_mut(&mut self, name: &str) -> Result<LpValueRefMut, RuntimeError>;

    /// Set a field value.
    ///
    /// The exact parameter type is still being determined (see DESIGN.md).
    fn set_field(&mut self, name: &str, value: &dyn LpValue) -> Result<(), RuntimeError>;

    /// Get the number of fields in this record.
    fn field_count(&self) -> usize;

    /// Get a field by index, returning both the field name and value.
    ///
    /// Returns `(field_name, field_value)` for the field at the given index.
    /// This allows iteration over fields without cloning.
    fn get_field_by_index(&self, index: usize) -> Result<(&str, LpValueRef), RuntimeError>;
}
