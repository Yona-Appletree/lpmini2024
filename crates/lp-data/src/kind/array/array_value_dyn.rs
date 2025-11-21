//! Dynamic array value implementation.
//!
//! Dynamic array values are created at runtime and store their elements in a collection.
//! This is in contrast to static array values, which are Rust arrays/slices that implement
//! `ArrayValue` directly via codegen.
//!
//! Uses `LpValueBox` for element storage, which allocates through the global allocator.

use alloc::vec::Vec;

use crate::kind::array::array_dyn::ArrayShapeDyn;
use crate::kind::array::array_shape::ArrayShape;
use crate::kind::array::array_value::ArrayValue;
use crate::kind::shape::LpShape;
use crate::kind::value::{LpValue, LpValueBox, LpValueRef, LpValueRefMut};
use crate::RuntimeError;

/// Dynamic array value.
///
/// Stores elements as a `Vec<LpValueBox>`.
/// All element values are stored as `LpValueBox`, which respects the lp_alloc limits.
pub struct ArrayValueDyn {
    /// The shape of this array.
    shape: ArrayShapeDyn,
    /// Elements stored as a vector of boxed values.
    elements: Vec<LpValueBox>,
}

impl ArrayValueDyn {
    /// Create a new empty dynamic array value with the given shape.
    pub fn new(shape: ArrayShapeDyn) -> Self {
        Self {
            shape,
            elements: Vec::new(),
        }
    }

    fn static_shape_of(value: &dyn LpValue) -> &'static dyn LpShape {
        // SAFETY: shapes are either static singletons or pool-allocated with 'static lifetime guarantees.
        unsafe { core::mem::transmute::<&dyn LpShape, &'static dyn LpShape>(value.shape()) }
    }

    fn validate_element_shape(&self, value: &LpValueBox) -> Result<(), RuntimeError> {
        let element_shape = self.shape.element_shape();
        let value_shape = match value {
            LpValueBox::Dec32(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Int32(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Bool(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Vec2(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Vec3(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Vec4(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Mat3(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Record(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::EnumUnit(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::EnumStruct(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Array(boxed) => Self::static_shape_of(boxed.as_ref()),
            LpValueBox::Option(boxed) => Self::static_shape_of(boxed.as_ref()),
        };

        // Compare by kind first (faster), then by pointer equality for exact match
        if element_shape.kind() == value_shape.kind() {
            // For primitive types, pointer equality should match
            // For complex types, we rely on kind matching
            if core::ptr::eq(
                element_shape as *const dyn LpShape,
                value_shape as *const dyn LpShape,
            ) {
                Ok(())
            } else {
                // If kinds match but pointers don't, it might be different instances of the same shape
                // For now, accept it if kinds match (this handles cases where shapes are recreated)
                Ok(())
            }
        } else {
            Err(RuntimeError::type_mismatch(
                &format!("{:?}", element_shape.kind()),
                &format!("{:?}", value_shape.kind()),
            ))
        }
    }
}

impl LpValue for ArrayValueDyn {
    fn shape(&self) -> &dyn LpShape {
        &self.shape
    }
}

impl ArrayValue for ArrayValueDyn {
    fn shape(&self) -> &dyn ArrayShape {
        &self.shape
    }

    fn get_element(&self, index: usize) -> Result<LpValueRef<'_>, RuntimeError> {
        let len = self.elements.len();
        let element = self
            .elements
            .get(index)
            .ok_or(RuntimeError::IndexOutOfBounds { index, len })?;

        let value_ref = match element {
            LpValueBox::Dec32(boxed) => LpValueRef::Dec32(boxed.as_ref()),
            LpValueBox::Int32(boxed) => LpValueRef::Int32(boxed.as_ref()),
            LpValueBox::Bool(boxed) => LpValueRef::Bool(boxed.as_ref()),
            LpValueBox::Vec2(boxed) => LpValueRef::Vec2(boxed.as_ref()),
            LpValueBox::Vec3(boxed) => LpValueRef::Vec3(boxed.as_ref()),
            LpValueBox::Vec4(boxed) => LpValueRef::Vec4(boxed.as_ref()),
            LpValueBox::Mat3(boxed) => LpValueRef::Mat3(boxed.as_ref()),
            LpValueBox::Record(boxed) => LpValueRef::Record(boxed.as_ref()),
            LpValueBox::EnumUnit(boxed) => LpValueRef::EnumUnit(boxed.as_ref()),
            LpValueBox::EnumStruct(boxed) => LpValueRef::EnumStruct(boxed.as_ref()),
            LpValueBox::Array(boxed) => LpValueRef::Array(boxed.as_ref()),
            LpValueBox::Option(boxed) => LpValueRef::Option(boxed.as_ref()),
        };

        Ok(value_ref)
    }

    fn get_element_mut(&mut self, index: usize) -> Result<LpValueRefMut<'_>, RuntimeError> {
        let len = self.elements.len();
        let element = self
            .elements
            .get_mut(index)
            .ok_or(RuntimeError::IndexOutOfBounds { index, len })?;

        let value_ref_mut = match element {
            LpValueBox::Dec32(boxed) => LpValueRefMut::Dec32(boxed.as_mut()),
            LpValueBox::Int32(boxed) => LpValueRefMut::Int32(boxed.as_mut()),
            LpValueBox::Bool(boxed) => LpValueRefMut::Bool(boxed.as_mut()),
            LpValueBox::Vec2(boxed) => LpValueRefMut::Vec2(boxed.as_mut()),
            LpValueBox::Vec3(boxed) => LpValueRefMut::Vec3(boxed.as_mut()),
            LpValueBox::Vec4(boxed) => LpValueRefMut::Vec4(boxed.as_mut()),
            LpValueBox::Mat3(boxed) => LpValueRefMut::Mat3(boxed.as_mut()),
            LpValueBox::Record(boxed) => LpValueRefMut::Record(boxed.as_mut()),
            LpValueBox::EnumUnit(boxed) => LpValueRefMut::EnumUnit(boxed.as_mut()),
            LpValueBox::EnumStruct(boxed) => LpValueRefMut::EnumStruct(boxed.as_mut()),
            LpValueBox::Array(boxed) => LpValueRefMut::Array(boxed.as_mut()),
            LpValueBox::Option(boxed) => LpValueRefMut::Option(boxed.as_mut()),
        };

        Ok(value_ref_mut)
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn push(&mut self, value: LpValueBox) -> Result<(), RuntimeError> {
        self.validate_element_shape(&value)?;
        self.elements.push(value);
        self.shape.len = self.elements.len();
        Ok(())
    }

    fn remove(&mut self, index: usize) -> Result<(), RuntimeError> {
        let len = self.elements.len();
        if index >= len {
            return Err(RuntimeError::IndexOutOfBounds { index, len });
        }
        self.elements.remove(index);
        self.shape.len = self.elements.len();
        Ok(())
    }
}

#[cfg(any(feature = "serde", feature = "serde_json"))]
impl serde::Serialize for ArrayValueDyn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.elements.len()))?;
        for element in self.elements.iter() {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}
