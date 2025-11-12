//! Option value trait and helper implementations.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::kind::option::option_shape::OptionShape;
use crate::kind::value::{LpValueBox, LpValueRef, LpValueRefMut};
use crate::kind::LpValue;
use crate::RuntimeError;

/// Trait for Option values that can be Some(T) or None.
pub trait OptionValue: LpValue {
    /// Get the Option shape for this value.
    fn shape(&self) -> &dyn OptionShape;

    /// Check if this Option is Some (has a value).
    fn is_some(&self) -> bool;

    /// Check if this Option is None (no value).
    fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Get the value if Some, or return an error if None.
    fn get_value(&self) -> Result<LpValueRef<'_>, RuntimeError>;

    /// Get the value if Some (mutable), or return an error if None.
    fn get_value_mut(&mut self) -> Result<LpValueRefMut<'_>, RuntimeError>;
}

impl From<Box<dyn OptionValue>> for LpValueBox {
    fn from(value: Box<dyn OptionValue>) -> Self {
        LpValueBox::Option(value)
    }
}

impl<'a> LpValueRef<'a> {
    /// Try to get a reference to the value as OptionValue.
    pub fn as_option(&self) -> Option<&'a dyn OptionValue> {
        match self {
            LpValueRef::Option(value) => Some(*value),
            _ => None,
        }
    }
}

impl<'a> LpValueRefMut<'a> {
    /// Try to get a mutable reference to the value as OptionValue.
    pub fn as_option_mut(&mut self) -> Option<&mut dyn OptionValue> {
        match self {
            LpValueRefMut::Option(value) => Some(*value),
            _ => None,
        }
    }
}
