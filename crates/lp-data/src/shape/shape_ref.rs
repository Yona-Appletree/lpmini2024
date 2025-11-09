//! Shape references for static and dynamic shapes.

use lp_pool::collections::LpBox;

use crate::shape::array::{DynamicArrayShape, StaticArrayShape};
use crate::shape::bool::StaticBoolShape;
use crate::shape::fixed::StaticFixedShape;
use crate::shape::int32::StaticInt32Shape;
use crate::shape::map::{DynamicMapShape, StaticMapShape};
use crate::shape::option::{DynamicOptionShape, StaticOptionShape};
use crate::shape::r#enum::{DynamicEnumShape, StaticEnumShape};
use crate::shape::record::{DynamicRecordShape, StaticRecordShape};
use crate::shape::string::StaticStringShape;
use crate::shape::tuple::{DynamicTupleShape, StaticTupleShape};
use crate::shape::vec2::StaticVec2Shape;
use crate::shape::vec3::StaticVec3Shape;
use crate::shape::vec4::StaticVec4Shape;

/// Reference to a shape, either static or dynamic.
#[derive(Debug)]
pub enum ShapeRef {
    Record(RecordShapeRef),
    Array(ArrayShapeRef),
    Option(OptionShapeRef),
    Tuple(TupleShapeRef),
    Map(MapShapeRef),
    Enum(EnumShapeRef),
    // Primitives
    Int32(Int32ShapeRef),
    Fixed(FixedShapeRef),
    Bool(BoolShapeRef),
    String(StringShapeRef),
    Vec2(Vec2ShapeRef),
    Vec3(Vec3ShapeRef),
    Vec4(Vec4ShapeRef),
}

/// Reference to a record shape.
#[derive(Debug, Clone)]
pub enum RecordShapeRef {
    Static(&'static StaticRecordShape),
    Dynamic(LpBox<DynamicRecordShape>),
}

/// Reference to an array shape.
#[derive(Debug, Clone)]
pub enum ArrayShapeRef {
    Static(&'static StaticArrayShape),
    Dynamic(LpBox<DynamicArrayShape>),
}

/// Reference to an option shape.
#[derive(Debug, Clone)]
pub enum OptionShapeRef {
    Static(&'static StaticOptionShape),
    Dynamic(LpBox<DynamicOptionShape>),
}

/// Reference to a tuple shape.
#[derive(Debug, Clone)]
pub enum TupleShapeRef {
    Static(&'static StaticTupleShape),
    Dynamic(LpBox<DynamicTupleShape>),
}

/// Reference to a map shape.
#[derive(Debug, Clone)]
pub enum MapShapeRef {
    Static(&'static StaticMapShape),
    Dynamic(LpBox<DynamicMapShape>),
}

/// Reference to an enum shape.
#[derive(Debug, Clone)]
pub enum EnumShapeRef {
    Static(&'static StaticEnumShape),
    Dynamic(LpBox<DynamicEnumShape>),
}

// Safety: Dynamic shapes are only created at runtime within a memory pool context.
// Static ShapeRef instances only use Static variants, never Dynamic variants.
// The Dynamic variants are only used at runtime, not in static contexts.
unsafe impl Sync for RecordShapeRef {}
unsafe impl Sync for ArrayShapeRef {}
unsafe impl Sync for OptionShapeRef {}
unsafe impl Sync for TupleShapeRef {}
unsafe impl Sync for MapShapeRef {}
unsafe impl Sync for EnumShapeRef {}
unsafe impl Sync for ShapeRef {}

/// Reference to a fixed-point shape.
#[derive(Debug)]
pub enum FixedShapeRef {
    Static(&'static StaticFixedShape),
}

/// Reference to an int32 shape.
#[derive(Debug)]
pub enum Int32ShapeRef {
    Static(&'static StaticInt32Shape),
}

/// Reference to a bool shape.
#[derive(Debug)]
pub enum BoolShapeRef {
    Static(&'static StaticBoolShape),
}

/// Reference to a string shape.
#[derive(Debug)]
pub enum StringShapeRef {
    Static(&'static StaticStringShape),
}

/// Reference to a vec2 shape.
#[derive(Debug)]
pub enum Vec2ShapeRef {
    Static(&'static StaticVec2Shape),
}

/// Reference to a vec3 shape.
#[derive(Debug)]
pub enum Vec3ShapeRef {
    Static(&'static StaticVec3Shape),
}

/// Reference to a vec4 shape.
#[derive(Debug)]
pub enum Vec4ShapeRef {
    Static(&'static StaticVec4Shape),
}

impl ShapeRef {
    /// Get the kind of this shape.
    pub fn kind(&self) -> crate::shape::kind::LpKind {
        match self {
            ShapeRef::Record(_) => crate::shape::kind::LpKind::Record,
            ShapeRef::Array(_) => crate::shape::kind::LpKind::Array,
            ShapeRef::Option(_) => crate::shape::kind::LpKind::Option,
            ShapeRef::Tuple(_) => crate::shape::kind::LpKind::Tuple,
            ShapeRef::Map(_) => crate::shape::kind::LpKind::Map,
            ShapeRef::Enum(_) => crate::shape::kind::LpKind::Enum,
            ShapeRef::Int32(_) => crate::shape::kind::LpKind::Int32,
            ShapeRef::Fixed(_) => crate::shape::kind::LpKind::Fixed,
            ShapeRef::Bool(_) => crate::shape::kind::LpKind::Bool,
            ShapeRef::String(_) => crate::shape::kind::LpKind::String,
            ShapeRef::Vec2(_) => crate::shape::kind::LpKind::Vec2,
            ShapeRef::Vec3(_) => crate::shape::kind::LpKind::Vec3,
            ShapeRef::Vec4(_) => crate::shape::kind::LpKind::Vec4,
        }
    }

    /// Create a default Fixed shape reference.
    pub fn fixed_default() -> Self {
        static DEFAULT: StaticFixedShape = StaticFixedShape::default();
        Self::Fixed(FixedShapeRef::Static(&DEFAULT))
    }

    /// Create a default Int32 shape reference.
    pub fn int32_default() -> Self {
        static DEFAULT: StaticInt32Shape = StaticInt32Shape::default();
        Self::Int32(Int32ShapeRef::Static(&DEFAULT))
    }

    /// Create a default Bool shape reference.
    pub fn bool_default() -> Self {
        static DEFAULT: StaticBoolShape = StaticBoolShape::default();
        Self::Bool(BoolShapeRef::Static(&DEFAULT))
    }

    /// Create a default String shape reference.
    pub fn string_default() -> Self {
        static DEFAULT: StaticStringShape = StaticStringShape::default();
        Self::String(StringShapeRef::Static(&DEFAULT))
    }

    /// Create a default Vec2 shape reference.
    pub fn vec2_default() -> Self {
        static DEFAULT: StaticVec2Shape = StaticVec2Shape::default();
        Self::Vec2(Vec2ShapeRef::Static(&DEFAULT))
    }

    /// Create a default Vec3 shape reference.
    pub fn vec3_default() -> Self {
        static DEFAULT: StaticVec3Shape = StaticVec3Shape::default();
        Self::Vec3(Vec3ShapeRef::Static(&DEFAULT))
    }

    /// Create a default Vec4 shape reference.
    pub fn vec4_default() -> Self {
        static DEFAULT: StaticVec4Shape = StaticVec4Shape::default();
        Self::Vec4(Vec4ShapeRef::Static(&DEFAULT))
    }
}
