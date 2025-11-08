pub mod array;
pub mod r#enum;
pub mod record;
pub mod scalar;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub use array::{ArrayType, ArrayUi};
pub use r#enum::{EnumType, EnumUi, EnumVariant};
pub use record::{RecordField, RecordType, RecordUi};
pub use scalar::{
    BoolScalar, BoolUi, FixedScalar, Int32Scalar, LpScalarType, NumberUi, SliderUi, StringScalar,
    StringUi,
};
pub use vec2::{Vec2Type, Vec2Ui};
pub use vec3::{Vec3Type, Vec3Ui};
pub use vec4::{Vec4Type, Vec4Ui};

/// Reference to another schema type metadata node.
pub type TypeRef = &'static LpTypeMeta;

/// Rich type information with UI hints.
#[derive(Debug, Clone, PartialEq)]
pub enum LpType {
    Scalar(LpScalarType),
    Vec2(Vec2Type),
    Vec3(Vec3Type),
    Vec4(Vec4Type),
    Array(ArrayType<TypeRef>),
    Record(RecordType<TypeRef>),
    Enum(EnumType<TypeRef>),
}

impl LpType {
    pub const fn string() -> Self {
        Self::Scalar(LpScalarType::string())
    }

    pub const fn fixed() -> Self {
        Self::Scalar(LpScalarType::fixed())
    }

    pub const fn int32() -> Self {
        Self::Scalar(LpScalarType::int32())
    }

    pub const fn boolean() -> Self {
        Self::Scalar(LpScalarType::boolean())
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
