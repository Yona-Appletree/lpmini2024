//! Shape system for lp-data types.
//!
//! This module provides the foundation for the new type system architecture,
//! separating type kinds, shapes (metadata), and values (runtime data).

use lp_math::fixed::Fixed;
use lp_pool::{LpString, LpVec};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Basic types
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LpKind {
    // Primitive
    Fixed,

    // Composite
    Record,
}

/// Metadata for a shape.
pub trait ShapeMeta {
    fn label() -> LpString;
    fn desc_md() -> Option<LpString>;
}

/// Trait for LpShapeDyn and LpShapeStatic.
pub trait LpShapeHolder {
    fn shape(&self) -> &dyn LpShape;
}

pub enum LpShapeDyn {
    Fixed(),
    Record(RecordShapeDyn),
}

impl LpShapeHolder for LpShapeDyn {
    fn shape(&self) -> &dyn LpShape {
        match self {
            LpShapeDyn::Fixed() => &FixedShape,
            LpShapeDyn::Record(t) => t,
        }
    }
}

pub enum LpShapeStatic {
    Fixed(),
    Record(RecordShapeStatic),
}

/// Base trait for all shape types.
pub trait LpShape {
    fn kind(&self) -> LpKind;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fixed
//

pub struct FixedShape;
impl LpShape for FixedShape {
    fn kind(&self) -> LpKind {
        LpKind::Fixed
    }
}

struct FixedMeta {
    label: LpString,
    desc_md: Option<LpString>,
}
impl ShapeMeta for FixedMeta {

}


////////////////////////////////////////////////////////////////////////////////////////////////////
// Record
//

pub trait RecordShape: LpShape {
    fn elements(&self) -> &[dyn RecordFieldShape];
}

pub struct RecordFieldMeta {
    fn name(&self) -> LpString;
    fn meta(&self) ->LpString;
}
pub trait RecordFieldShape {
    fn name(&self) -> &str;
    fn shape(&self) -> &dyn LpShape;
    fn meta(&self) -> &RecordFieldMeta;
}

pub struct RecordShapeDyn {
    pub elements: LpVec<&dyn LpShape>,
}
impl RecordShape for RecordShapeDyn {
    fn elements(&self) -> &[ShapeRef] {
        &self.elements
    }
}

pub struct RecordShapeStatic {
    pub elements: &'static [LpShapeStatic],
}
impl RecordShape for RecordShapeStatic {
    fn elements(&self) -> &[ShapeRef] {
        &self.elements
    }
}



////////////////////////////////////////////////////////////////////////////////////////////////////
// Value
//


/// Base trait for all runtime values.
pub trait LpValue {
    /// Get the shape reference for this value.
    fn shape(&self) -> &dyn LpShape;
}

pub struct LpFixed(Fixed);
impl LpValue for LpFixed {
    fn shape(&self) -> &LpShape {

    }
}

/// Value operations for tuple types.
pub trait RecordValue: LpValue {
    /// Get an element by index.
    fn get_element(&self, index: usize) -> Result<&dyn LpValue, crate::value::RuntimeError>;

    /// Get a mutable element by index.
    fn get_element_mut(
        &mut self,
        index: usize,
    ) -> Result<&mut dyn LpValue, crate::value::RuntimeError>;

    /// Set an element value.
    fn set_element(
        &mut self,
        index: usize,
        value: LpValue,
    ) -> Result<(), crate::value::RuntimeError>;

    /// Get the length of the tuple.
    fn len(&self) -> usize;
}
