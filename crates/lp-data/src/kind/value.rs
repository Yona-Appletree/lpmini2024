//! Value system for lp-data types.
//!
//! Values represent runtime data instances that reference their shape.

#[cfg(feature = "alloc")]
extern crate alloc;

use super::shape::LpShape;
use crate::value::RuntimeError;
use lp_math::fixed::Fixed;
use lp_pool::{LpBox, LpBoxDyn};

pub enum LpValueBox {
    Fixed(LpBoxDyn<dyn LpValue>),
    Record(LpBoxDyn<dyn RecordValue>),
}

impl From<Fixed> for LpValueBox {
    fn from(value: Fixed) -> Self {
        // Box the Fixed value as a trait object
        let trait_ref: &dyn LpValue = &value;
        // Fixed is Copy, so bitwise copying the trait object is safe
        #[allow(deprecated)]
        let boxed =
            LpBoxDyn::try_new_unsized(trait_ref).expect("Failed to allocate Fixed value in pool");
        LpValueBox::Fixed(boxed)
    }
}

impl From<LpBoxDyn<dyn RecordValue>> for LpValueBox {
    fn from(value: LpBoxDyn<dyn RecordValue>) -> Self {
        LpValueBox::Record(value)
    }
}

impl Clone for LpValueBox {
    fn clone(&self) -> Self {
        match self {
            LpValueBox::Fixed(boxed) => {
                // Clone the underlying value by creating a new box
                let trait_ref: &dyn LpValue = boxed.as_ref();
                // We're cloning the value, so this is safe
                #[allow(deprecated)]
                let cloned_box = LpBoxDyn::try_new_unsized(trait_ref)
                    .expect("Failed to allocate cloned Fixed value in pool");
                LpValueBox::Fixed(cloned_box)
            }
            LpValueBox::Record(boxed) => {
                // Clone the underlying record value by creating a new box
                let trait_ref: &dyn RecordValue = boxed.as_ref();
                // We're cloning the value, so this is safe
                #[allow(deprecated)]
                let cloned_box = LpBoxDyn::try_new_unsized(trait_ref)
                    .expect("Failed to allocate cloned Record value in pool");
                LpValueBox::Record(cloned_box)
            }
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
    #[cfg(feature = "alloc")]
    fn iter_fields(&self) -> alloc::vec::IntoIter<(alloc::string::String, LpValueBox)>;
}
