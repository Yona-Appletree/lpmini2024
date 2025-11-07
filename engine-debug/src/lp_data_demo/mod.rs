use lpscript::math::vec2::Vec2;
use lp_data::annotation::{Annotations, AnnotationValue};
use lp_data::schema;
use lp_data::ty::{LpEnumVariant, LpField, LpStructType, LpType};
use lp_data::value::{Fixed32, LpValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircleMappingConfig {
    pub ring_counts: Vec<i32>,
    pub ring_direction: RingDirection,
    pub led_direction: LedDirection,
    #[serde(with = "vec2_serde")]
    pub center: Vec2,
    pub radius: Fixed32,
}

impl CircleMappingConfig {
    pub fn lp_type() -> LpType {
        let mut struct_ty = LpStructType::new("CircleMappingConfig");
        struct_ty.add_field(
            LpField::new("ring_counts", LpType::array(LpType::int32())).with_annotations(
                Annotations::new().with(
                    "description",
                    AnnotationValue::text(
                        "The counts of leds on the rings of the display, inner-most first",
                    ),
                ),
            ),
        );
        struct_ty.add_field(LpField::new("ring_direction", ring_direction_type()));
        struct_ty.add_field(LpField::new("led_direction", led_direction_type()));
        struct_ty.add_field(
            LpField::new("center", vec2_type()).with_annotations(
                Annotations::new().with("ui.type", AnnotationValue::text("point2")),
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

    pub fn schema_json() -> Value {
        schema::to_json_schema(&Self::lp_type())
    }

    pub fn sample() -> Self {
        CircleMappingConfig {
            ring_counts: vec![8, 12, 16, 20, 24, 32],
            ring_direction: RingDirection::Forward,
            led_direction: LedDirection::Clockwise,
            center: Vec2::from_f32(0.5, 0.5),
            radius: Fixed32(0.25),
        }
    }

    pub fn sample_json() -> Value {
        serde_json::to_value(Self::sample()).expect("serialize sample config")
    }

    pub fn sample_lp_value() -> LpValue {
        Self::sample().to_lp_value()
    }

    pub fn to_lp_value(&self) -> LpValue {
        LpValue::structure([
            (
                "ring_counts",
                LpValue::array(
                    self.ring_counts
                        .iter()
                        .copied()
                        .map(LpValue::int32)
                        .collect(),
                ),
            ),
            (
                "ring_direction",
                LpValue::enumeration("RingDirection", self.ring_direction.as_str()),
            ),
            (
                "led_direction",
                LpValue::enumeration("LedDirection", self.led_direction.as_str()),
            ),
            (
                "center",
                LpValue::structure([
                    ("x", LpValue::fixed32(self.center.x.to_f32())),
                    ("y", LpValue::fixed32(self.center.y.to_f32())),
                ]),
            ),
            ("radius", LpValue::fixed32(self.radius.0)),
        ])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RingDirection {
    Forward,
    Reverse,
}

impl RingDirection {
    fn as_str(self) -> &'static str {
        match self {
            RingDirection::Forward => "Forward",
            RingDirection::Reverse => "Reverse",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LedDirection {
    Clockwise,
    CounterClockwise,
}

impl LedDirection {
    fn as_str(self) -> &'static str {
        match self {
            LedDirection::Clockwise => "Clockwise",
            LedDirection::CounterClockwise => "CounterClockwise",
        }
    }
}

fn ring_direction_type() -> LpType {
    LpType::enumeration(
        "RingDirection",
        vec![
            LpEnumVariant::unit("Forward"),
            LpEnumVariant::unit("Reverse"),
        ],
    )
}

fn led_direction_type() -> LpType {
    LpType::enumeration(
        "LedDirection",
        vec![
            LpEnumVariant::unit("Clockwise"),
            LpEnumVariant::unit("CounterClockwise"),
        ],
    )
}

fn vec2_type() -> LpType {
    let mut vec2 = LpStructType::new("Vec2");
    vec2.add_field(LpField::new("x", LpType::fixed32()));
    vec2.add_field(LpField::new("y", LpType::fixed32()));
    LpType::structure(vec2)
}

mod vec2_serde {
    use super::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Vec2, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let tuple = (value.x.to_f32(), value.y.to_f32());
        tuple.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec2, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (x, y) = <(f32, f32)>::deserialize(deserializer)?;
        Ok(Vec2::from_f32(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_contains_radius_slider_metadata() {
        let schema = CircleMappingConfig::schema_json();
        let radius = &schema["properties"]["radius"];
        assert_eq!(radius["type"], serde_json::json!("number"));
        let ui = radius["x-annotations"]["ui"].as_object().expect("ui annotations");
        assert_eq!(ui.get("widget"), Some(&serde_json::json!("slider")));
    }

    #[test]
    fn sample_round_trips_through_json() {
        let sample = CircleMappingConfig::sample();
        let json = CircleMappingConfig::sample_json();
        let back: CircleMappingConfig =
            serde_json::from_value(json).expect("valid circle mapping config");
        assert_eq!(serde_json::to_value(sample).unwrap(), serde_json::to_value(back).unwrap());
    }

    #[test]
    fn lp_type_lists_expected_fields() {
        let ty = CircleMappingConfig::lp_type();
        let names: Vec<_> = match ty {
            LpType::Struct(data) => data.fields.iter().map(|field| field.name).collect(),
            other => panic!("expected struct type, got {:?}", other),
        };
        assert_eq!(
            names,
            vec!["ring_counts", "ring_direction", "led_direction", "center", "radius"]
        );
    }
}

