//! Tests for enum shapes.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::kind::LpKind;
    use crate::shape::r#enum::enum_meta::EnumUi;
    use crate::shape::r#enum::{EnumVariant, StaticEnumShape};

    #[test]
    fn test_static_enum_shape() {
        const VARIANTS: &[EnumVariant] = &[EnumVariant::unit("None"), EnumVariant::unit("Some")];

        let shape = StaticEnumShape {
            name: "Option",
            variants: VARIANTS,
            ui: EnumUi::Dropdown,
        };

        assert_eq!(shape.kind(), LpKind::Enum);
        assert_eq!(shape.variants().len(), 2);
    }
}
