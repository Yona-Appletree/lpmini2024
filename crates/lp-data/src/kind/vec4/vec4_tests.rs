//! Integration tests for Vec4 shapes and values.

#[cfg(test)]
mod tests {
    use lp_math::dec32::{Dec32, Vec4};

    use crate::kind::kind::LpKind;
    use crate::kind::shape::LpShape;
    use crate::kind::value::LpValue;
    use crate::kind::vec4::vec4_static::VEC4_SHAPE;

    #[test]
    fn test_vec4_shape() {
        assert_eq!(VEC4_SHAPE.kind(), LpKind::Vec4);
    }

    #[test]
    fn test_vec4_value() {
        let value = Vec4::new(Dec32::ZERO, Dec32::ZERO, Dec32::ZERO, Dec32::ZERO);
        let shape = value.shape();
        assert_eq!(shape.kind(), LpKind::Vec4);
    }

    #[test]
    fn test_vec4_shape_with_meta() {
        use crate::kind::vec4::vec4_meta::Vec4MetaStatic;
        use crate::kind::vec4::vec4_shape::Vec4Shape;
        use crate::kind::vec4::vec4_static::Vec4ShapeStatic;

        let meta = Vec4MetaStatic {
            label: "Color",
            desc_md: Some("RGBA color values"),
            unit: None,
        };
        let shape = Vec4ShapeStatic::with_meta(meta);
        assert_eq!(shape.kind(), LpKind::Vec4);

        // Demonstrate Vec4Shape trait usage for polymorphic access
        let shape_ref: &dyn Vec4Shape = &shape;
        let meta_ref = shape_ref.meta().unwrap();
        assert_eq!(meta_ref.label(), "Color");
        assert_eq!(meta_ref.desc_md(), Some("RGBA color values"));
        assert_eq!(meta_ref.unit(), None);
    }

    #[test]
    fn test_vec4_value_box() {
        use crate::kind::value::LpValueBox;

        let value = Vec4::new(Dec32::ZERO, Dec32::ZERO, Dec32::ZERO, Dec32::ZERO);
        let value_box: LpValueBox = value.into();
        match value_box {
            LpValueBox::Vec4(_) => {}
            _ => panic!("Expected Vec4 variant"),
        }
    }
}
