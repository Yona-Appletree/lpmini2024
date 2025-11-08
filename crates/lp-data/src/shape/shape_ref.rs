//! Shape references for static and dynamic shapes.

use lp_pool::collections::LpBox;

use crate::shape::array::{DynamicArrayShape, StaticArrayShape};
use crate::shape::map::{DynamicMapShape, StaticMapShape};
use crate::shape::option::{DynamicOptionShape, StaticOptionShape};
use crate::shape::r#enum::{DynamicEnumShape, StaticEnumShape};
use crate::shape::record::{DynamicRecordShape, StaticRecordShape};
use crate::shape::tuple::{DynamicTupleShape, StaticTupleShape};

/// Reference to a shape, either static or dynamic.
#[derive(Debug)]
pub enum ShapeRef {
    Record(RecordShapeRef),
    Array(ArrayShapeRef),
    Option(OptionShapeRef),
    Tuple(TupleShapeRef),
    Map(MapShapeRef),
    Enum(EnumShapeRef),
    // Primitives (no metadata needed)
    Int32,
    Fixed,
    Bool,
    String,
    Vec2,
    Vec3,
    Vec4,
}

/// Reference to a record shape.
#[derive(Debug)]
pub enum RecordShapeRef {
    Static(&'static StaticRecordShape),
    Dynamic(LpBox<DynamicRecordShape>),
}

/// Reference to an array shape.
#[derive(Debug)]
pub enum ArrayShapeRef {
    Static(&'static StaticArrayShape),
    Dynamic(LpBox<DynamicArrayShape>),
}

/// Reference to an option shape.
#[derive(Debug)]
pub enum OptionShapeRef {
    Static(&'static StaticOptionShape),
    Dynamic(LpBox<DynamicOptionShape>),
}

/// Reference to a tuple shape.
#[derive(Debug)]
pub enum TupleShapeRef {
    Static(&'static StaticTupleShape),
    Dynamic(LpBox<DynamicTupleShape>),
}

/// Reference to a map shape.
#[derive(Debug)]
pub enum MapShapeRef {
    Static(&'static StaticMapShape),
    Dynamic(LpBox<DynamicMapShape>),
}

/// Reference to an enum shape.
#[derive(Debug)]
pub enum EnumShapeRef {
    Static(&'static StaticEnumShape),
    Dynamic(LpBox<DynamicEnumShape>),
}
