use lp_data::registry::LpDataType;
use lp_data::ty::{LpEnumVariant, LpField, LpStructType, LpType};

/// LFO shape enumeration
pub struct LfoShape;

impl LpDataType for LfoShape {
    fn type_name() -> &'static str {
        "LfoShape"
    }

    fn lp_type() -> LpType {
        LpType::enumeration(
            "LfoShape",
            vec![
                LpEnumVariant::unit("Sine"),
                LpEnumVariant::unit("Square"),
                LpEnumVariant::unit("Triangle"),
                LpEnumVariant::unit("Sawtooth"),
            ],
        )
    }
}

/// Configuration for an LFO (Low Frequency Oscillator)
pub struct LfoConfig;

impl LpDataType for LfoConfig {
    fn type_name() -> &'static str {
        "LfoConfig"
    }

    fn lp_type() -> LpType {
        let mut struct_ty = LpStructType::new("LfoConfig");
        struct_ty.add_field(LpField::new("period_ms", LpType::int32()));
        struct_ty.add_field(LpField::new("shape", LfoShape::lp_type()));
        struct_ty.add_field(LpField::new("min", LpType::fixed32()));
        struct_ty.add_field(LpField::new("max", LpType::fixed32()));
        LpType::structure(struct_ty)
    }
}
