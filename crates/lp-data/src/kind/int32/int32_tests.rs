//! Integration tests for Int32 shapes and values.

#[cfg(test)]
mod tests {
    use crate::kind::int32::int32_static::INT32_SHAPE;
    use crate::kind::kind::LpKind;
    use crate::kind::shape::LpShape;
    use crate::kind::value::LpValue;

    #[test]
    fn test_int32_shape() {
        assert_eq!(INT32_SHAPE.kind(), LpKind::Int32);
    }

    #[test]
    fn test_int32_value() {
        let value = 42i32;
        let shape = value.shape();
        assert_eq!(shape.kind(), LpKind::Int32);
    }

    #[test]
    fn test_int32_shape_with_meta() {
        use crate::kind::int32::int32_meta::Int32MetaStatic;
        use crate::kind::int32::int32_shape::Int32Shape;
        use crate::kind::int32::int32_static::Int32ShapeStatic;

        let meta = Int32MetaStatic {
            label: "Count",
            desc_md: Some("Number of items"),
            unit: None,
        };
        let shape = Int32ShapeStatic::with_meta(meta);
        assert_eq!(shape.kind(), LpKind::Int32);

        // Demonstrate Int32Shape trait usage for polymorphic access
        let shape_ref: &dyn Int32Shape = &shape;
        let meta_ref = shape_ref.meta().unwrap();
        assert_eq!(meta_ref.label(), "Count");
        assert_eq!(meta_ref.desc_md(), Some("Number of items"));
        assert_eq!(meta_ref.unit(), None);
    }

    #[test]
    fn test_int32_value_box() {
        use crate::kind::value::LpValueBox;

        let value = 42i32;
        let value_box: LpValueBox = value.into();
        match value_box {
            LpValueBox::Int32(_) => {}
            _ => panic!("Expected Int32 variant"),
        }
    }
}
