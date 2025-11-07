use lp_data::registry::LpDataType;
use lp_data::ty::{LpField, LpStructType, LpType};

/// Configuration for a UI slider control
pub struct UiSliderConfig;

impl LpDataType for UiSliderConfig {
    fn type_name() -> &'static str {
        "UiSliderConfig"
    }

    fn lp_type() -> LpType {
        let mut struct_ty = LpStructType::new("UiSliderConfig");
        struct_ty.add_field(LpField::new("min", LpType::fixed32()));
        struct_ty.add_field(LpField::new("max", LpType::fixed32()));
        struct_ty.add_field(LpField::new("step", LpType::fixed32()));
        struct_ty.add_field(LpField::new("default", LpType::fixed32()));
        LpType::structure(struct_ty)
    }
}

