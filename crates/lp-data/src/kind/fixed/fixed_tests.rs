//! Integration tests for Fixed shapes and values.

#[cfg(test)]
mod tests {
    use lp_math::fixed::ToFixed;

    use crate::kind::fixed::fixed_static::FIXED_SHAPE;
    use crate::kind::kind::LpKind;
    use crate::kind::shape::LpShape;
    use crate::kind::value::LpValue;

    #[test]
    fn test_fixed_shape() {
        assert_eq!(FIXED_SHAPE.kind(), LpKind::Fixed);
    }

    #[test]
    fn test_fixed_value() {
        let value = 42i32.to_fixed();
        let shape = value.shape();
        assert_eq!(shape.kind(), LpKind::Fixed);
    }

    #[test]
    fn test_fixed_shape_with_meta() {
        use crate::kind::fixed::fixed_meta::FixedMetaStatic;
        use crate::kind::fixed::fixed_shape::FixedShape;
        use crate::kind::fixed::fixed_static::FixedShapeStatic;

        let meta = FixedMetaStatic {
            label: "Frequency",
            desc_md: Some("Oscillation frequency"),
            unit: Some("Hz"),
        };
        let shape = FixedShapeStatic::with_meta(meta);
        assert_eq!(shape.kind(), LpKind::Fixed);

        // Demonstrate FixedShape trait usage for polymorphic access
        let shape_ref: &dyn FixedShape = &shape;
        let meta_ref = shape_ref.meta().unwrap();
        assert_eq!(meta_ref.label(), "Frequency");
        assert_eq!(meta_ref.desc_md(), Some("Oscillation frequency"));
        assert_eq!(meta_ref.unit(), Some("Hz"));
    }
}
