use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::annotation::Annotations;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpPrimitive {
    Int32,
    Fixed32,
    Bool,
    Vec2,
    Vec3,
    Vec4,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LpType {
    Primitive(LpPrimitive),
    Array(LpArrayType),
    Struct(LpStructType),
    Enum(LpEnumType),
}

impl LpType {
    pub fn int32() -> Self {
        LpType::Primitive(LpPrimitive::Int32)
    }

    pub fn fixed32() -> Self {
        LpType::Primitive(LpPrimitive::Fixed32)
    }

    pub fn boolean() -> Self {
        LpType::Primitive(LpPrimitive::Bool)
    }

    pub fn vec2() -> Self {
        LpType::Primitive(LpPrimitive::Vec2)
    }

    pub fn vec3() -> Self {
        LpType::Primitive(LpPrimitive::Vec3)
    }

    pub fn vec4() -> Self {
        LpType::Primitive(LpPrimitive::Vec4)
    }

    pub fn array(element: LpType) -> Self {
        LpType::Array(LpArrayType {
            element: Box::new(element),
        })
    }

    pub fn structure(struct_type: LpStructType) -> Self {
        LpType::Struct(struct_type)
    }

    pub fn enumeration(name: &'static str, variants: Vec<LpEnumVariant>) -> Self {
        LpType::Enum(LpEnumType { name, variants })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LpArrayType {
    pub element: Box<LpType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LpStructType {
    pub name: &'static str,
    pub fields: Vec<LpField>,
}

impl LpStructType {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: LpField) {
        self.fields.push(field);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LpField {
    pub name: &'static str,
    pub ty: Box<LpType>,
    pub annotations: Annotations,
}

impl LpField {
    pub fn new(name: &'static str, ty: LpType) -> Self {
        Self {
            name,
            ty: Box::new(ty),
            annotations: Annotations::default(),
        }
    }

    pub fn with_annotations(mut self, annotations: Annotations) -> Self {
        self.annotations = annotations;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LpEnumType {
    pub name: &'static str,
    pub variants: Vec<LpEnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LpEnumVariant {
    pub name: &'static str,
}

impl LpEnumVariant {
    pub fn unit(name: &'static str) -> Self {
        Self { name }
    }
}
