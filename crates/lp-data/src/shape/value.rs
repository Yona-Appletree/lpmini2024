//! Value trait hierarchy for runtime values.

use crate::value::LpValue;

/// Base trait for all runtime values.
pub trait LpValueTrait: core::fmt::Debug + Clone {
    // Base operations can be added here if needed
}

/// Value operations for record/struct types.
pub trait RecordValue: LpValueTrait {
    /// Get a field by name.
    fn get_field(&self, name: &str) -> Result<&LpValue, crate::value::RuntimeError>;

    /// Get a mutable field by name.
    fn get_field_mut(&mut self, name: &str) -> Result<&mut LpValue, crate::value::RuntimeError>;

    /// Set a field value.
    fn set_field(&mut self, name: &str, value: LpValue) -> Result<(), crate::value::RuntimeError>;
}

/// Value operations for array types.
pub trait ArrayValue: LpValueTrait {
    /// Get an element by index.
    fn get_element(&self, index: usize) -> Result<&LpValue, crate::value::RuntimeError>;

    /// Get a mutable element by index.
    fn get_element_mut(&mut self, index: usize)
        -> Result<&mut LpValue, crate::value::RuntimeError>;

    /// Set an element value.
    fn set_element(
        &mut self,
        index: usize,
        value: LpValue,
    ) -> Result<(), crate::value::RuntimeError>;

    /// Push a new element.
    fn push_element(&mut self, value: LpValue) -> Result<(), crate::value::RuntimeError>;

    /// Get the length of the array.
    fn len(&self) -> usize;
}

/// Value operations for option types.
pub trait OptionValue: LpValueTrait {
    /// Check if the option is Some.
    fn is_some(&self) -> bool;

    /// Check if the option is None.
    fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Unwrap the inner value.
    fn unwrap(&self) -> Result<&LpValue, crate::value::RuntimeError>;

    /// Unwrap the inner value mutably.
    fn unwrap_mut(&mut self) -> Result<&mut LpValue, crate::value::RuntimeError>;
}

/// Value operations for tuple types.
pub trait TupleValue: LpValueTrait {
    /// Get an element by index.
    fn get_element(&self, index: usize) -> Result<&LpValue, crate::value::RuntimeError>;

    /// Get a mutable element by index.
    fn get_element_mut(&mut self, index: usize)
        -> Result<&mut LpValue, crate::value::RuntimeError>;

    /// Set an element value.
    fn set_element(
        &mut self,
        index: usize,
        value: LpValue,
    ) -> Result<(), crate::value::RuntimeError>;

    /// Get the length of the tuple.
    fn len(&self) -> usize;
}

/// Value operations for map/dynamic record types.
pub trait MapValue: LpValueTrait {
    /// Get a field by name.
    fn get_field(&self, name: &str) -> Result<&LpValue, crate::value::RuntimeError>;

    /// Get a mutable field by name.
    fn get_field_mut(&mut self, name: &str) -> Result<&mut LpValue, crate::value::RuntimeError>;

    /// Set a field value.
    fn set_field(&mut self, name: &str, value: LpValue) -> Result<(), crate::value::RuntimeError>;

    /// Check if a field exists.
    fn has_field(&self, name: &str) -> bool;

    /// Get the number of fields.
    fn len(&self) -> usize;
}

/// Value operations for enum types.
pub trait EnumValue: LpValueTrait {
    /// Get the variant index.
    fn variant_index(&self) -> usize;

    /// Get the variant name.
    fn variant_name(&self) -> Option<&str>;

    /// Get the value associated with the variant (if any).
    fn value(&self) -> Option<&LpValue>;

    /// Get the value associated with the variant mutably (if any).
    fn value_mut(&mut self) -> Option<&mut LpValue>;
}
