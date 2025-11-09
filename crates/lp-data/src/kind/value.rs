//! Value system for lp-data types.
//!
//! Values represent runtime data instances that reference their shape.

#[cfg(feature = "alloc")]
extern crate alloc;

use super::shape::LpShape;
use crate::value::RuntimeError;
use lp_math::fixed::Fixed;
use lp_pool::{lp_box_dyn, LpBoxDyn};

pub enum LpValueBox {
    Fixed(LpBoxDyn<dyn LpValue>),
    Record(LpBoxDyn<dyn RecordValue>),
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
}
