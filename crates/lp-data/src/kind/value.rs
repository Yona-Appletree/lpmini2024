//! Value system for lp-data types.
//!
//! Values represent runtime data instances that reference their shape.

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
pub use alloc::vec::IntoIter as VecIntoIter;

use super::shape::LpShape;
use crate::value::RuntimeError;
use lp_math::fixed::Fixed;

pub enum LpValueBox {
    Fixed(Box<dyn LpValue>),
    Record(Box<dyn RecordValue>),
}

impl From<Fixed> for LpValueBox {
    fn from(value: Fixed) -> Self {
        LpValueBox::Fixed(Box::new(value))
    }
}

impl From<Box<dyn RecordValue>> for LpValueBox {
    fn from(value: Box<dyn RecordValue>) -> Self {
        LpValueBox::Record(value)
    }
}

impl Clone for LpValueBox {
    fn clone(&self) -> Self {
        match self {
            LpValueBox::Fixed(boxed) => {
                // For Fixed values, we need to downcast to clone
                // This is safe because we know it's a Fixed
                let fixed_value =
                    unsafe { &*(boxed.as_ref() as *const dyn LpValue as *const Fixed) };
                LpValueBox::Fixed(Box::new(*fixed_value))
            }
            LpValueBox::Record(boxed) => LpValueBox::Record(boxed.clone_box()),
        }
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
    /// Get a field by name (immutable).
    fn get_field(&self, name: &str) -> Result<&dyn LpValue, RuntimeError>;

    /// Get a field by name (mutable).
    fn get_field_mut(&mut self, name: &str) -> Result<&mut dyn LpValue, RuntimeError>;

    /// Set a field value.
    ///
    /// The exact parameter type is still being determined (see DESIGN.md).
    fn set_field(&mut self, name: &str, value: &dyn LpValue) -> Result<(), RuntimeError>;

    /// Get the number of fields in this record.
    fn field_count(&self) -> usize;

    /// Iterate over all fields as (name, value) pairs.
    ///
    /// Returns an iterator that yields owned `LpValueBox` values.
    /// Note: This requires collecting fields into a temporary collection,
    /// so it's more efficient to use `get_field()` when you know the field name.
    fn iter_fields(&self) -> VecIntoIter<(String, LpValueBox)>;

    /// Clone this record value into a new boxed trait object.
    ///
    /// This is used by `LpValueBox::clone()` to clone record values.
    fn clone_box(&self) -> Box<dyn RecordValue>;
}
