//! Integration tests for Dec32 shapes and values.

#[cfg(test)]
mod tests {
    use lp_math::dec32::ToDec32;

    use crate::kind::dec32::dec32_static::DEC32_SHAPE;
    use crate::kind::kind::LpKind;
    use crate::kind::shape::LpShape;
    use crate::kind::value::LpValue;

    #[test]
    fn test_dec32_shape() {
        assert_eq!(DEC32_SHAPE.kind(), LpKind::Dec32);
    }

    #[test]
    fn test_dec32_value() {
        let value = 42i32.to_dec32();
        let shape = value.shape();
        assert_eq!(shape.kind(), LpKind::Dec32);
    }

    #[test]
    fn test_dec32_shape_with_meta() {
        use crate::kind::dec32::dec32_meta::Dec32MetaStatic;
        use crate::kind::dec32::dec32_shape::Dec32Shape;
        use crate::kind::dec32::dec32_static::Dec32ShapeStatic;

        let meta = Dec32MetaStatic {
            label: "Frequency",
            desc_md: Some("Oscillation frequency"),
            unit: Some("Hz"),
        };
        let shape = Dec32ShapeStatic::with_meta(meta);
        assert_eq!(shape.kind(), LpKind::Dec32);

        // Demonstrate FixedShape trait usage for polymorphic access
        let shape_ref: &dyn Dec32Shape = &shape;
        let meta_ref = shape_ref.meta().unwrap();
        assert_eq!(meta_ref.label(), "Frequency");
        assert_eq!(meta_ref.desc_md(), Some("Oscillation frequency"));
        assert_eq!(meta_ref.unit(), Some("Hz"));
    }
}
