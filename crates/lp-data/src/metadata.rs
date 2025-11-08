//! Type metadata re-exports.
//!
//! This module provides backwards-compatible re-exports from the new
//! feature-based type system structure.

pub use crate::types;

// Re-export individual scalar types
pub use types::bool::{BoolScalar, BoolUi};
pub use types::fixed::{FixedScalar, NumberUi, SliderUi};
pub use types::int32::Int32Scalar;
pub use types::string::{StringScalar, StringUi};

// Re-export type metadata from types modules
pub use types::{
    ArrayType, ArrayUi, EnumType, EnumUi, EnumVariant, OptionType, RecordField, RecordType,
    RecordUi, Vec2Type, Vec2Ui, Vec3Type, Vec3Ui, Vec4Type, Vec4Ui,
};

// Re-export MapType (dynamic records)
pub use types::record_dyn::MapType;

/// Reference to another schema type metadata node.
pub type TypeRef = &'static LpTypeMeta;

/// Rich type information with UI hints.
#[derive(Debug, Clone, PartialEq)]
pub enum LpType {
    String(StringScalar),
    Fixed(FixedScalar),
    Int32(Int32Scalar),
    Bool(BoolScalar),
    Vec2(Vec2Type),
    Vec3(Vec3Type),
    Vec4(Vec4Type),
    Array(ArrayType<TypeRef>),
    Record(RecordType<TypeRef>),
    Enum(EnumType<TypeRef>),
    Option(OptionType<TypeRef>),
    Map(MapType),
}

impl LpType {
    pub const fn string() -> Self {
        Self::String(StringScalar {
            ui: StringUi::SingleLine,
        })
    }

    pub const fn fixed() -> Self {
        Self::Fixed(FixedScalar {
            ui: NumberUi::Textbox,
        })
    }

    pub const fn int32() -> Self {
        Self::Int32(Int32Scalar {
            ui: NumberUi::Textbox,
        })
    }

    pub const fn boolean() -> Self {
        Self::Bool(BoolScalar {
            ui: BoolUi::Checkbox,
        })
    }

    pub const fn vec2() -> Self {
        Self::Vec2(Vec2Type::raw())
    }

    pub const fn vec3() -> Self {
        Self::Vec3(Vec3Type::raw())
    }

    pub const fn vec4() -> Self {
        Self::Vec4(Vec4Type::raw())
    }

    pub const fn option(inner: TypeRef) -> Self {
        Self::Option(OptionType::new(inner))
    }

    pub const fn map() -> Self {
        Self::Map(MapType::new())
    }
}

/// Wrapper carrying top-level metadata for a type node.
#[derive(Debug, Clone, PartialEq)]
pub struct LpTypeMeta {
    pub ty: LpType,
    pub docs: Option<&'static str>,
}

impl LpTypeMeta {
    pub const fn new(ty: LpType) -> Self {
        Self { ty, docs: None }
    }

    pub const fn with_docs(mut self, docs: &'static str) -> Self {
        self.docs = Some(docs);
        self
    }
}
