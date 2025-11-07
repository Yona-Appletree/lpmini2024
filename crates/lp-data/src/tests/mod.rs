use alloc::vec;
use alloc::vec::Vec;

use crate::annotation::{Annotations, AnnotationValue};
use crate::ty::{LpEnumVariant, LpField, LpStructType, LpType};

#[test]
fn primitive_types_cover_core_variants() {
    assert!(matches!(LpType::int32(), LpType::Primitive(crate::ty::LpPrimitive::Int32)));
    assert!(matches!(LpType::fixed32(), LpType::Primitive(crate::ty::LpPrimitive::Fixed32)));
    assert!(matches!(LpType::boolean(), LpType::Primitive(crate::ty::LpPrimitive::Bool)));
}

#[test]
fn array_type_captures_element_type() {
    let array_ty = LpType::array(LpType::int32());
    match array_ty {
        LpType::Array(array) => {
            assert!(matches!(*array.element, LpType::Primitive(crate::ty::LpPrimitive::Int32)));
        }
        _ => panic!("expected array type"),
    }
}

#[test]
fn struct_type_retains_field_metadata() {
    let mut struct_ty = LpStructType::new("CircleMappingConfig");
    struct_ty.add_field(
        LpField::new("ring_counts", LpType::array(LpType::int32()))
            .with_annotations(
                Annotations::new().with("description", AnnotationValue::text("Counts")),
            ),
    );
    struct_ty.add_field(LpField::new("radius", LpType::fixed32()));

    let ty = LpType::structure(struct_ty);
    match ty {
        LpType::Struct(data) => {
            assert_eq!(data.name, "CircleMappingConfig");
            assert_eq!(data.fields.len(), 2);
            let field = &data.fields[0];
            assert_eq!(field.name, "ring_counts");
            assert!(matches!(
                *field.ty,
                LpType::Array(ref array) if matches!(
                    *array.element,
                    LpType::Primitive(crate::ty::LpPrimitive::Int32)
                )
            ));
            assert_eq!(
                field
                    .annotations
                    .get("description")
                    .and_then(AnnotationValue::as_text),
                Some("Counts")
            );
        }
        _ => panic!("expected struct type"),
    }
}

#[test]
fn enum_type_tracks_variants() {
    let ty = LpType::enumeration(
        "RingDirection",
        vec![
            LpEnumVariant::unit("Forward"),
            LpEnumVariant::unit("Reverse"),
        ],
    );

    match ty {
        LpType::Enum(data) => {
            assert_eq!(data.name, "RingDirection");
            let variant_names: Vec<_> = data.variants.iter().map(|v| v.name).collect();
            assert_eq!(variant_names, vec!["Forward", "Reverse"]);
        }
        _ => panic!("expected enum type"),
    }
}

#[test]
fn annotations_support_nested_objects() {
    let annotations = Annotations::new().with(
        "ui.slider",
        AnnotationValue::object()
            .with("min", AnnotationValue::number(0.01))
            .with("max", AnnotationValue::number(0.5))
            .with("step", AnnotationValue::number(0.001)),
    );

    let slider = annotations.get("ui.slider").expect("missing slider");
    assert_eq!(slider.get("min").and_then(AnnotationValue::as_number), Some(0.01));
    assert_eq!(slider.get("max").and_then(AnnotationValue::as_number), Some(0.5));
    assert_eq!(slider.get("step").and_then(AnnotationValue::as_number), Some(0.001));
}

#[cfg(feature = "serde_json")]
mod json_tests {
    use super::*;
    use crate::schema;
    use crate::value::LpValue;
    use serde_json::json;

    fn circle_config_type() -> LpType {
        let mut struct_ty = LpStructType::new("CircleMappingConfig");
        struct_ty.add_field(
            LpField::new("ring_counts", LpType::array(LpType::int32())).with_annotations(
                Annotations::new().with(
                    "description",
                    AnnotationValue::text("The counts of leds on the rings of the display, inner-most first"),
                ),
            ),
        );
        struct_ty.add_field(
            LpField::new("radius", LpType::fixed32()).with_annotations(
                Annotations::new().with(
                    "ui",
                    AnnotationValue::object()
                        .with("widget", AnnotationValue::text("slider"))
                        .with("min", AnnotationValue::number(0.01))
                        .with("max", AnnotationValue::number(0.5))
                        .with("step", AnnotationValue::number(0.001)),
                ),
            ),
        );
        LpType::structure(struct_ty)
    }

    #[test]
    fn schema_includes_annotations_and_constraints() {
        let schema = schema::to_json_schema(&circle_config_type());

        let radius = &schema["properties"]["radius"];
        assert_eq!(radius["type"], json!("number"));

        let annotations = radius["x-annotations"].as_object().expect("annotations");
        let ui = annotations["ui"].as_object().expect("ui annotations");
        assert_eq!(ui["widget"], json!("slider"));
        assert_eq!(ui["min"], json!(0.01));
        assert_eq!(ui["max"], json!(0.5));
        assert_eq!(ui["step"], json!(0.001));
    }

    #[test]
    fn lp_value_round_trip_serialization() {
        let value = LpValue::structure([
            ("ring_counts", LpValue::array(vec![
                LpValue::int32(1),
                LpValue::int32(2),
                LpValue::int32(3),
            ])),
            ("ring_direction", LpValue::enumeration("RingDirection", "Forward")),
            ("led_direction", LpValue::enumeration("LedDirection", "Clockwise")),
            ("radius", LpValue::fixed32(0.125)),
        ]);

        let json = serde_json::to_string(&value).expect("serialize");
        let back: LpValue = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, value);
    }
}

