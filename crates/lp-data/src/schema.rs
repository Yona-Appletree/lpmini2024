use crate::annotation::{AnnotationValue, Annotations};
use crate::ty::{LpArrayType, LpEnumType, LpPrimitive, LpStructType, LpType};

#[cfg(feature = "serde_json")]
use alloc::vec::Vec;

#[cfg(feature = "serde_json")]
use serde_json::{json, Map, Value};

#[cfg(feature = "serde_json")]
pub fn to_json_schema(ty: &LpType) -> Value {
    match ty {
        LpType::Primitive(prim) => primitive_schema(prim),
        LpType::Array(array) => array_schema(array),
        LpType::Struct(data) => struct_schema(data),
        LpType::Enum(data) => enum_schema(data),
    }
}

#[cfg(feature = "serde_json")]
fn primitive_schema(prim: &LpPrimitive) -> Value {
    match prim {
        LpPrimitive::Int32 => json!({ "type": "integer", "format": "int32" }),
        LpPrimitive::Fixed32 => json!({ "type": "number", "format": "fixed32" }),
        LpPrimitive::Bool => json!({ "type": "boolean" }),
        LpPrimitive::Vec2 => json!({
            "type": "object",
            "properties": {
                "x": { "type": "number", "format": "fixed32" },
                "y": { "type": "number", "format": "fixed32" }
            },
            "required": ["x", "y"]
        }),
        LpPrimitive::Vec3 => json!({
            "type": "object",
            "properties": {
                "x": { "type": "number", "format": "fixed32" },
                "y": { "type": "number", "format": "fixed32" },
                "z": { "type": "number", "format": "fixed32" }
            },
            "required": ["x", "y", "z"]
        }),
        LpPrimitive::Vec4 => json!({
            "type": "object",
            "properties": {
                "x": { "type": "number", "format": "fixed32" },
                "y": { "type": "number", "format": "fixed32" },
                "z": { "type": "number", "format": "fixed32" },
                "w": { "type": "number", "format": "fixed32" }
            },
            "required": ["x", "y", "z", "w"]
        }),
    }
}

#[cfg(feature = "serde_json")]
fn array_schema(array: &LpArrayType) -> Value {
    json!({
        "type": "array",
        "items": to_json_schema(array.element.as_ref()),
    })
}

#[cfg(feature = "serde_json")]
fn struct_schema(data: &LpStructType) -> Value {
    let mut properties = Map::new();
    let mut required = Vec::new();

    for field in &data.fields {
        required.push(field.name.to_string());
        let mut property = to_json_schema(field.ty.as_ref());
        apply_field_annotations(&mut property, &field.annotations);
        properties.insert(field.name.to_string(), property);
    }

    let mut schema = Map::new();
    schema.insert("type".into(), Value::String("object".into()));
    schema.insert("title".into(), Value::String(data.name.into()));
    schema.insert("properties".into(), Value::Object(properties));
    schema.insert(
        "required".into(),
        Value::Array(required.into_iter().map(Value::String).collect()),
    );
    Value::Object(schema)
}

#[cfg(feature = "serde_json")]
fn enum_schema(data: &LpEnumType) -> Value {
    json!({
        "type": "string",
        "title": data.name,
        "enum": data.variants.iter().map(|variant| variant.name).collect::<Vec<_>>(),
    })
}

#[cfg(feature = "serde_json")]
fn apply_field_annotations(target: &mut Value, annotations: &Annotations) {
    if annotations.is_empty() {
        return;
    }

    let mut object = target.as_object().cloned().unwrap_or_default();

    if let Some(description) = annotations
        .get("description")
        .and_then(AnnotationValue::as_text)
    {
        object.insert(
            "description".into(),
            Value::String(description.to_string()),
        );
    }

    object.insert("x-annotations".into(), annotations.to_json());

    *target = Value::Object(object);
}

