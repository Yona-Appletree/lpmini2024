//! Value system for lp-data types.
//!
//! Values represent runtime data instances that reference their shape.

#[cfg(feature = "alloc")]
extern crate alloc;

use super::shape::LpShape;
use crate::kind::record::record_value::RecordValue;
use lp_pool::LpBoxDyn;

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

/// Base trait for all runtime values.
///
/// Values are concrete instances of data. Rust types like `Fixed` or `LfoConfig`
/// implement this trait directly - they ARE the values, not wrappers.
pub trait LpValue {
    /// Get the shape reference for this value.
    fn shape(&self) -> &dyn LpShape;
}
