mod config_test;
mod nodes;
mod runtime_scene_test;
mod value_test;

// TODO: Update tests to use new shape system
// All tests below are commented out until they're updated to use the new shape system

/*
#[test]
#[ignore] // TODO: Update to use new shape system
fn scalar_defaults_use_textbox_ui() {
    let int_ty = LpType::int32();
    let fixed_ty = LpType::fixed();
    let bool_ty = LpType::boolean();

    let int_scalar = match int_ty {
        LpType::Int32(meta) => meta,
        other => panic!("expected int scalar, found {other:?}"),
    };
    assert!(matches!(int_scalar.ui, NumberUi::Textbox));

    let fixed_scalar = match fixed_ty {
        LpType::Fixed(meta) => meta,
        other => panic!("expected fixed scalar, found {other:?}"),
    };
    assert!(matches!(fixed_scalar.ui, NumberUi::Textbox));

    let bool_scalar = match bool_ty {
        LpType::Bool(meta) => meta,
        other => panic!("expected bool scalar, found {other:?}"),
    };
    assert!(matches!(bool_scalar.ui, crate::BoolUi::Checkbox));
}

#[test]
#[ignore] // TODO: Update to use new shape system
fn string_default_is_single_line() {
    let string_ty = LpType::string();
    let string_meta = match string_ty {
        LpType::String(meta) => meta,
        other => panic!("expected string scalar, found {other:?}"),
    };
    assert!(matches!(string_meta.ui, StringUi::SingleLine));
}

#[test]
#[ignore] // TODO: Update to use new shape system
fn vector_defaults_are_raw_modes() {
    let vec2 = LpType::vec2();
    let vec3 = LpType::vec3();
    let vec4 = LpType::vec4();

    let vec2_meta = match vec2 {
        LpType::Vec2(meta) => meta,
        other => panic!("expected vec2 types, found {other:?}"),
    };
    assert!(matches!(vec2_meta.ui, Vec2Ui::Raw));

    let vec3_meta = match vec3 {
        LpType::Vec3(meta) => meta,
        other => panic!("expected vec3 types, found {other:?}"),
    };
    assert!(matches!(vec3_meta.ui, Vec3Ui::Raw));

    let vec4_meta = match vec4 {
        LpType::Vec4(meta) => meta,
        other => panic!("expected vec4 types, found {other:?}"),
    };
    assert!(matches!(vec4_meta.ui, Vec4Ui::Raw));
}

#[test]
#[ignore] // TODO: Update to use new shape system
fn array_type_links_element_metadata() {
    const ELEMENT: LpTypeMeta = LpTypeMeta::new(LpType::int32());
    const ARRAY: LpTypeMeta = LpTypeMeta::new(LpType::Array(ArrayType::new(&ELEMENT)));

    let array = match ARRAY.ty {
        LpType::Array(array) => array,
        _ => panic!("expected array types"),
    };
    assert!(core::ptr::eq(array.element, &ELEMENT));
}

#[test]
#[ignore] // TODO: Update to use new shape system
fn enum_type_records_variant_names_and_ui() {
    const VARIANTS: &[EnumVariant<TypeRef>] =
        &[EnumVariant::unit("First"), EnumVariant::unit("Second")];
    const ENUM_META: LpTypeMeta = LpTypeMeta::new(LpType::Enum(
        EnumType::new("Example", VARIANTS).with_ui(EnumUi::Dropdown),
    ));

    let enum_meta = match ENUM_META.ty {
        LpType::Enum(meta) => meta,
        _ => panic!("expected enum types"),
    };
    assert_eq!(enum_meta.name, "Example");
    let names: Vec<_> = enum_meta
        .variants
        .iter()
        .map(|variant| match variant {
            EnumVariant::Unit { name } => *name,
            _ => panic!("unexpected variant kind"),
        })
        .collect();
    assert_eq!(names, ["First", "Second"]);
    assert!(matches!(enum_meta.ui, EnumUi::Dropdown));
}

#[test]
#[ignore] // TODO: Update to use new shape system
fn record_field_docs_are_preserved() {
    const CHILD: LpTypeMeta = LpTypeMeta::new(LpType::boolean());
    const FIELDS: &[RecordField<TypeRef>] =
        &[RecordField::new("flag", &CHILD).with_docs("boolean flag field")];
    const RECORD: LpTypeMeta = LpTypeMeta::new(LpType::Record(RecordType::new("Simple", FIELDS)));

    let record = match RECORD.ty {
        LpType::Record(record) => record,
        _ => panic!("expected record types"),
    };
    assert_eq!(record.fields.len(), 1);
    assert_eq!(record.fields[0].docs, Some("boolean flag field"));
}
*/
