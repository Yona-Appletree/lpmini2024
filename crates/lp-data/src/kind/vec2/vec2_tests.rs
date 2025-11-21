//! Integration tests for Vec2 shapes and values.

#[cfg(test)]
mod tests {
    use lp_math::dec32::{Dec32, Vec2};

    use crate::kind::kind::LpKind;
    use crate::kind::shape::LpShape;
    use crate::kind::value::LpValue;
    use crate::kind::vec2::vec2_static::VEC2_SHAPE;

    #[test]
    fn test_vec2_shape() {
        assert_eq!(VEC2_SHAPE.kind(), LpKind::Vec2);
    }

    #[test]
    fn test_vec2_value() {
        let value = Vec2::new(Dec32::ZERO, Dec32::ZERO);
        let shape = value.shape();
        assert_eq!(shape.kind(), LpKind::Vec2);
    }

    #[test]
    fn test_vec2_shape_with_meta() {
        use crate::kind::vec2::vec2_meta::Vec2MetaStatic;
        use crate::kind::vec2::vec2_shape::Vec2Shape;
        use crate::kind::vec2::vec2_static::Vec2ShapeStatic;

        let meta = Vec2MetaStatic {
            label: "Position",
            desc_md: Some("2D position coordinates"),
            unit: Some("px"),
        };
        let shape = Vec2ShapeStatic::with_meta(meta);
        assert_eq!(shape.kind(), LpKind::Vec2);

        // Demonstrate Vec2Shape trait usage for polymorphic access
        let shape_ref: &dyn Vec2Shape = &shape;
        let meta_ref = shape_ref.meta().unwrap();
        assert_eq!(meta_ref.label(), "Position");
        assert_eq!(meta_ref.desc_md(), Some("2D position coordinates"));
        assert_eq!(meta_ref.unit(), Some("px"));
    }

    #[test]
    fn test_vec2_value_box() {
        use crate::kind::value::LpValueBox;

        let value = Vec2::new(Dec32::ZERO, Dec32::ZERO);
        let value_box: LpValueBox = value.into();
        match value_box {
            LpValueBox::Vec2(_) => {}
            _ => panic!("Expected Vec2 variant"),
        }
    }
}
