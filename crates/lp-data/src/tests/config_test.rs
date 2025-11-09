// TODO: Update config_test.rs to use new shape system once derive macro is updated
// This entire file is commented out until the derive macro is updated

/*
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use lp_math::fixed::{Fixed, Vec2, Vec3};
use serde::{Deserialize, Serialize};

// Import lp_data for macro-generated code
use crate as lp_data;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, LpSchema)]
#[lp(schema(
    name = "Color Stop",
    docs = "Defines a single gradient stop within the UI."
))]
struct ColorStop {
    #[lp(field(
        ui(slider(min = 0.0, max = 1.0, step = 0.05)),
        docs = "Location of the stop, normalized to 0-1."
    ))]
    position: Fixed,
    #[lp(field(ui(color)))]
    color: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, LpSchema)]
#[lp(schema(
    name = "Waveform",
    docs = "Shape of the LFO waveform.",
    ui = "segmented"
))]
enum Waveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, LpSchema)]
#[lp(schema(
    name = "LFO Config",
    docs = "Top level configuration structure used by tests."
))]
struct LfoConfig {
    #[lp(field(
        ui(slider(min = 10, max = 60000, step = 1)),
        docs = "Oscillation period in milliseconds."
    ))]
    period_ms: i32,
    #[lp(field(ui(multiline)))]
    notes: String,
    #[lp(field(ui(color)))]
    tint: Vec3,
    #[lp(field(ui(position)))]
    offset: Vec2,
    enabled: bool,
    waveform: Waveform,
    #[lp(field(docs = "User configured gradient stops."))]
    stops: Vec<ColorStop>,
}

fn find_field<'a>(record: &'a RecordType<TypeRef>, name: &str) -> &'a RecordField<TypeRef> {
    record
        .fields
        .iter()
        .find(|field| field.name == name)
        .unwrap_or_else(|| panic!("missing field {name}"))
}

#[test]
fn color_stop_schema_reflects_slider_metadata() {
    let schema = ColorStop::lp_schema();
    let record = match &schema.ty {
        LpType::Record(record) => record,
        _ => panic!("expected ColorStop to derive a record schema"),
    };
    assert_eq!(record.name, "Color Stop");

    let position = find_field(record, "position");
    let scalar = match &position.ty.ty {
        LpType::Fixed(metadata) => metadata,
        other => panic!("expected fixed scalar types, found {other:?}"),
    };
    let NumberUi::Slider(slider) = &scalar.ui else {
        panic!("expected slider ui types");
    };
    assert_eq!(slider.min, 0.0);
    assert_eq!(slider.max, 1.0);
    assert_eq!(slider.step, Some(0.05));
}

#[test]
fn waveform_schema_generates_enum_variants_and_ui() {
    let schema = Waveform::lp_schema();
    let enum_meta = match &schema.ty {
        LpType::Enum(meta) => meta,
        _ => panic!("expected enum schema"),
    };
    assert_eq!(enum_meta.name, "Waveform");
    let names: Vec<_> = enum_meta
        .variants
        .iter()
        .map(|variant| match variant {
            crate::EnumVariant::Unit { name } => *name,
            _ => panic!("expected unit variants"),
        })
        .collect();
    assert_eq!(names, ["Sine", "Square", "Triangle", "Sawtooth"]);
    assert!(matches!(enum_meta.ui, crate::EnumUi::SegmentedControl));
    // Verify TYPE_NAME defaults to the enum name when not specified
    assert_eq!(Waveform::TYPE_NAME, "Waveform");
}

#[test]
fn lfo_config_schema_composes_nested_metadata() {
    let schema = LfoConfig::lp_schema();
    let record = match &schema.ty {
        LpType::Record(record) => record,
        _ => panic!("expected record schema"),
    };
    assert_eq!(record.name, "LFO Config");

    let period = find_field(record, "period_ms");
    let period_scalar = match &period.ty.ty {
        LpType::Int32(meta) => meta,
        _ => panic!("period_ms should be int32"),
    };
    let NumberUi::Slider(slider) = &period_scalar.ui else {
        panic!("period_ms should use slider ui");
    };
    assert_eq!(slider.min, 10.0);
    assert_eq!(slider.max, 60000.0);
    assert_eq!(slider.step, Some(1.0));

    let notes = find_field(record, "notes");
    let notes_scalar = match &notes.ty.ty {
        LpType::String(meta) => meta,
        _ => panic!("notes should be a string scalar"),
    };
    assert!(matches!(notes_scalar.ui, StringUi::MultiLine));

    let tint = find_field(record, "tint");
    let tint_vec3 = match &tint.ty.ty {
        LpType::Vec3(meta) => meta,
        _ => panic!("tint should be vec3 types"),
    };
    assert!(matches!(tint_vec3.ui, Vec3Ui::Color));

    let offset = find_field(record, "offset");
    let offset_vec2 = match &offset.ty.ty {
        LpType::Vec2(meta) => meta,
        _ => panic!("offset should be vec2 types"),
    };
    assert!(matches!(offset_vec2.ui, Vec2Ui::Position));

    let enabled = find_field(record, "enabled");
    let enabled_bool = match &enabled.ty.ty {
        LpType::Bool(meta) => meta,
        _ => panic!("enabled should be bool types"),
    };
    assert!(matches!(enabled_bool.ui, BoolUi::Checkbox));

    let waveform = find_field(record, "waveform");
    assert!(
        core::ptr::eq(waveform.ty, Waveform::lp_schema()),
        "waveform should reference Waveform::lp_schema()"
    );

    let stops = find_field(record, "stops");
    let array = match &stops.ty.ty {
        LpType::Array(array) => array,
        _ => panic!("stops should derive an array schema"),
    };
    assert!(
        core::ptr::eq(array.element, ColorStop::lp_schema()),
        "array element should reference ColorStop::lp_schema()"
    );
}

#[test]
#[ignore] // TODO: Update to use new shape system once derive macro is updated
fn explicit_registration_accepts_user_types() {
    let mut registry = crate::TypeRegistry::new();
    register_lp_schemas!(registry, LfoConfig, ColorStop, Waveform);

    let config = registry
        .get("LfoConfig")
        .expect("LfoConfig should be registered");
    let record = match &config.ty {
        LpType::Record(record) => record,
        _ => panic!("LfoConfig should register as record"),
    };
    assert_eq!(record.fields.len(), 7);
}

#[test]
fn lfo_config_round_trips_json() {
    let _config = LfoConfig {
        period_ms: 1200,
        notes: "Example configuration".to_owned(),
        tint: Vec3::new(
            Fixed::from_f32(0.25),
            Fixed::from_f32(0.5),
            Fixed::from_f32(0.75),
        ),
        offset: Vec2::new(Fixed::from_i32(1), Fixed::from_i32(2)),
        enabled: true,
        waveform: Waveform::Square,
        stops: vec![
            ColorStop {
                position: Fixed::from_f32(0.0),
                color: Vec3::new(
                    Fixed::from_f32(1.0),
                    Fixed::from_f32(0.0),
                    Fixed::from_f32(0.0),
                ),
            },
            ColorStop {
                position: Fixed::from_f32(1.0),
                color: Vec3::new(
                    Fixed::from_f32(0.0),
                    Fixed::from_f32(0.0),
                    Fixed::from_f32(1.0),
                ),
            },
        ],
    };

    #[cfg(feature = "serde_json")]
    {
        let json = serde_json::to_string(&_config).expect("serialize");
        let back: LfoConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, _config);
    }
}
*/
