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

#[cfg(any(feature = "serde", feature = "serde_json"))]
impl serde::Serialize for LpValueBox {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_lp_value_ref(
            match self {
                LpValueBox::Fixed(boxed) => LpValueRef::Fixed(boxed.as_ref()),
                LpValueBox::Record(boxed) => LpValueRef::Record(boxed.as_ref()),
            },
            serializer,
        )
    }
}

#[cfg(any(feature = "serde", feature = "serde_json"))]
fn serialize_lp_value_ref<S>(value_ref: LpValueRef, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value_ref {
        LpValueRef::Fixed(fixed_ref) => {
            use lp_math::fixed::Fixed;
            use serde::Serialize;
            // SAFETY: We know this is a Fixed because it's in the Fixed variant
            // The vtable pointer points to Fixed's implementation
            let fixed_value = unsafe { &*(fixed_ref as *const dyn LpValue as *const Fixed) };
            fixed_value.serialize(serializer)
        }
        LpValueRef::Record(record_ref) => {
            use crate::kind::record::record_value::RecordValue;
            use serde::ser::SerializeMap;
            let shape = RecordValue::shape(record_ref);
            let field_count = shape.field_count();
            let mut map = serializer.serialize_map(Some(field_count))?;
            for i in 0..field_count {
                if let Some(field_shape) = shape.get_field(i) {
                    if let Ok(field_value_ref) = record_ref.get_field_by_index(i) {
                        // Recursively serialize the field value
                        map.serialize_entry(
                            field_shape.name(),
                            &LpValueRefSerializer(field_value_ref),
                        )?;
                    }
                }
            }
            map.end()
        }
    }
}

#[cfg(any(feature = "serde", feature = "serde_json"))]
struct LpValueRefSerializer<'a>(LpValueRef<'a>);

#[cfg(any(feature = "serde", feature = "serde_json"))]
impl<'a> serde::Serialize for LpValueRefSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // LpValueRef doesn't implement Copy, but we can reconstruct it from the reference
        let value_ref = match self.0 {
            LpValueRef::Fixed(fixed_ref) => LpValueRef::Fixed(fixed_ref),
            LpValueRef::Record(record_ref) => LpValueRef::Record(record_ref),
        };
        serialize_lp_value_ref(value_ref, serializer)
    }
}
