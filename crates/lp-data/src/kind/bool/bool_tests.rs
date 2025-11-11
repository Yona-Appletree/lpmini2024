//! Integration tests for Bool shapes and values.

#[cfg(test)]
mod tests {
    use crate::kind::bool::bool_static::BOOL_SHAPE;
    use crate::kind::kind::LpKind;
    use crate::kind::shape::LpShape;
    use crate::kind::value::LpValue;

    #[test]
    fn test_bool_shape() {
        assert_eq!(BOOL_SHAPE.kind(), LpKind::Bool);
    }

    #[test]
    fn test_bool_value() {
        let value = true;
        let shape = value.shape();
        assert_eq!(shape.kind(), LpKind::Bool);
    }

    #[test]
    fn test_bool_shape_with_meta() {
        use crate::kind::bool::bool_meta::BoolMetaStatic;
        use crate::kind::bool::bool_shape::BoolShape;
        use crate::kind::bool::bool_static::BoolShapeStatic;

        let meta = BoolMetaStatic {
            label: "Enabled",
            desc_md: Some("Whether the feature is enabled"),
            unit: None,
        };
        let shape = BoolShapeStatic::with_meta(meta);
        assert_eq!(shape.kind(), LpKind::Bool);

        // Demonstrate BoolShape trait usage for polymorphic access
        let shape_ref: &dyn BoolShape = &shape;
        let meta_ref = shape_ref.meta().unwrap();
        assert_eq!(meta_ref.label(), "Enabled");
        assert_eq!(meta_ref.desc_md(), Some("Whether the feature is enabled"));
        assert_eq!(meta_ref.unit(), None);
    }

    #[test]
    fn test_bool_value_box() {
        use crate::kind::value::LpValueBox;

        let value = true;
        let value_box: LpValueBox = value.into();
        match value_box {
            LpValueBox::Bool(_) => {}
            _ => panic!("Expected Bool variant"),
        }
    }
}
