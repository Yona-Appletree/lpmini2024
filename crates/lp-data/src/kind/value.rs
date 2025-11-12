//! Value system for lp-data types.
//!
//! Values represent runtime data instances that reference their shape.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use super::shape::LpShape;
use crate::kind::enum_struct::enum_struct_value::EnumStructValue;
use crate::kind::enum_unit::enum_value::EnumUnitValue;
use crate::kind::record::record_value::RecordValue;

pub enum LpValueBox {
    Fixed(Box<dyn LpValue>),
    Int32(Box<dyn LpValue>),
    Bool(Box<dyn LpValue>),
    Vec2(Box<dyn LpValue>),
    Vec3(Box<dyn LpValue>),
    Vec4(Box<dyn LpValue>),
    Record(Box<dyn RecordValue>),
    EnumUnit(Box<dyn EnumUnitValue>),
    EnumStruct(Box<dyn EnumStructValue>),
}

/// Type-aware reference to a value.
///
/// Preserves type information so that RecordValue methods can be called
/// without downcasting. This is the reference equivalent of LpValueBox.
#[derive(Copy, Clone)]
pub enum LpValueRef<'a> {
    Fixed(&'a dyn LpValue),
    Int32(&'a dyn LpValue),
    Bool(&'a dyn LpValue),
    Vec2(&'a dyn LpValue),
    Vec3(&'a dyn LpValue),
    Vec4(&'a dyn LpValue),
    Record(&'a dyn RecordValue),
    EnumUnit(&'a dyn EnumUnitValue),
    EnumStruct(&'a dyn EnumStructValue),
}

impl<'a> LpValueRef<'a> {
    /// Get a reference to the value as LpValue.
    pub fn as_lp_value(&self) -> &'a dyn LpValue {
        match self {
            LpValueRef::Fixed(v) => *v,
            LpValueRef::Int32(v) => *v,
            LpValueRef::Bool(v) => *v,
            LpValueRef::Vec2(v) => *v,
            LpValueRef::Vec3(v) => *v,
            LpValueRef::Vec4(v) => *v,
            LpValueRef::Record(v) => *v as &dyn LpValue,
            LpValueRef::EnumUnit(v) => *v as &dyn LpValue,
            LpValueRef::EnumStruct(v) => *v as &dyn LpValue,
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
    Int32(&'a mut dyn LpValue),
    Bool(&'a mut dyn LpValue),
    Vec2(&'a mut dyn LpValue),
    Vec3(&'a mut dyn LpValue),
    Vec4(&'a mut dyn LpValue),
    Record(&'a mut dyn RecordValue),
    EnumUnit(&'a mut dyn EnumUnitValue),
    EnumStruct(&'a mut dyn EnumStructValue),
}

impl<'a> LpValueRefMut<'a> {
    /// Get a mutable reference to the value as LpValue.
    pub fn as_lp_value_mut(&mut self) -> &mut dyn LpValue {
        match self {
            LpValueRefMut::Fixed(v) => *v,
            LpValueRefMut::Int32(v) => *v,
            LpValueRefMut::Bool(v) => *v,
            LpValueRefMut::Vec2(v) => *v,
            LpValueRefMut::Vec3(v) => *v,
            LpValueRefMut::Vec4(v) => *v,
            LpValueRefMut::Record(v) => *v as &mut dyn LpValue,
            LpValueRefMut::EnumUnit(v) => *v as &mut dyn LpValue,
            LpValueRefMut::EnumStruct(v) => *v as &mut dyn LpValue,
        }
    }
}

impl<'a> core::ops::Deref for LpValueRefMut<'a> {
    type Target = dyn LpValue + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            LpValueRefMut::Fixed(v) => *v,
            LpValueRefMut::Int32(v) => *v,
            LpValueRefMut::Bool(v) => *v,
            LpValueRefMut::Vec2(v) => *v,
            LpValueRefMut::Vec3(v) => *v,
            LpValueRefMut::Vec4(v) => *v,
            LpValueRefMut::Record(v) => *v as &dyn LpValue,
            LpValueRefMut::EnumUnit(v) => *v as &dyn LpValue,
            LpValueRefMut::EnumStruct(v) => *v as &dyn LpValue,
        }
    }
}

impl<'a> core::ops::DerefMut for LpValueRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            LpValueRefMut::Fixed(v) => *v,
            LpValueRefMut::Int32(v) => *v,
            LpValueRefMut::Bool(v) => *v,
            LpValueRefMut::Vec2(v) => *v,
            LpValueRefMut::Vec3(v) => *v,
            LpValueRefMut::Vec4(v) => *v,
            LpValueRefMut::Record(v) => *v as &mut dyn LpValue,
            LpValueRefMut::EnumUnit(v) => *v as &mut dyn LpValue,
            LpValueRefMut::EnumStruct(v) => *v as &mut dyn LpValue,
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

/// Helper functions to convert LpValue to LpValueRef based on shape kind.
/// These use unsafe to construct trait objects when we know the type implements
/// the trait based on runtime shape information.
#[cfg(any(feature = "serde", feature = "serde_json"))]
impl serde::Serialize for LpValueBox {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_lp_value_ref(
            match self {
                LpValueBox::Fixed(boxed) => LpValueRef::Fixed(boxed.as_ref()),
                LpValueBox::Int32(boxed) => LpValueRef::Int32(boxed.as_ref()),
                LpValueBox::Bool(boxed) => LpValueRef::Bool(boxed.as_ref()),
                LpValueBox::Vec2(boxed) => LpValueRef::Vec2(boxed.as_ref()),
                LpValueBox::Vec3(boxed) => LpValueRef::Vec3(boxed.as_ref()),
                LpValueBox::Vec4(boxed) => LpValueRef::Vec4(boxed.as_ref()),
                LpValueBox::Record(boxed) => LpValueRef::Record(boxed.as_ref()),
                LpValueBox::EnumUnit(boxed) => LpValueRef::EnumUnit(boxed.as_ref()),
                LpValueBox::EnumStruct(boxed) => LpValueRef::EnumStruct(boxed.as_ref()),
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
        LpValueRef::Int32(int32_ref) => {
            use serde::Serialize;
            // SAFETY: We know this is an i32 because it's in the Int32 variant
            let int32_value = unsafe { &*(int32_ref as *const dyn LpValue as *const i32) };
            int32_value.serialize(serializer)
        }
        LpValueRef::Bool(bool_ref) => {
            use serde::Serialize;
            // SAFETY: We know this is a bool because it's in the Bool variant
            let bool_value = unsafe { &*(bool_ref as *const dyn LpValue as *const bool) };
            bool_value.serialize(serializer)
        }
        LpValueRef::Vec2(vec2_ref) => {
            use lp_math::fixed::Vec2;
            use serde::Serialize;
            // SAFETY: We know this is a Vec2 because it's in the Vec2 variant
            let vec2_value = unsafe { &*(vec2_ref as *const dyn LpValue as *const Vec2) };
            vec2_value.serialize(serializer)
        }
        LpValueRef::Vec3(vec3_ref) => {
            use lp_math::fixed::Vec3;
            use serde::Serialize;
            // SAFETY: We know this is a Vec3 because it's in the Vec3 variant
            let vec3_value = unsafe { &*(vec3_ref as *const dyn LpValue as *const Vec3) };
            vec3_value.serialize(serializer)
        }
        LpValueRef::Vec4(vec4_ref) => {
            use lp_math::fixed::Vec4;
            use serde::Serialize;
            // SAFETY: We know this is a Vec4 because it's in the Vec4 variant
            let vec4_value = unsafe { &*(vec4_ref as *const dyn LpValue as *const Vec4) };
            vec4_value.serialize(serializer)
        }
        LpValueRef::Record(record_ref) => {
            use serde::ser::SerializeMap;

            use crate::kind::record::record_value::RecordValue;
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
        LpValueRef::EnumUnit(enum_ref) => {
            // Serialize enum as the variant name string
            let variant_name = enum_ref
                .variant_name()
                .map_err(|_| serde::ser::Error::custom("Failed to get enum variant name"))?;
            serializer.serialize_str(variant_name)
        }
        LpValueRef::EnumStruct(union_ref) => {
            use serde::ser::SerializeMap;

            let variant_name = union_ref
                .variant_name()
                .map_err(|_| serde::ser::Error::custom("Failed to get union variant name"))?;
            let variant_value = union_ref
                .variant_value()
                .map_err(|_| serde::ser::Error::custom("Failed to get union variant value"))?;
            let mut map = serializer.serialize_map(Some(1))?;
            map.serialize_entry(variant_name, &LpValueRefSerializer(variant_value))?;
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
        serialize_lp_value_ref(self.0, serializer)
    }
}
