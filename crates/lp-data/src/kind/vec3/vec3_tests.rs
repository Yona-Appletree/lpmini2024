//! Integration tests for Vec3 shapes and values.

#[cfg(test)]
mod tests {
    use crate::kind::{
        kind::LpKind, shape::LpShape, value::LpValue, vec3::vec3_static::VEC3_SHAPE,
    };
    use lp_math::fixed::{Fixed, Vec3};

    #[test]
    fn test_vec3_shape() {
        assert_eq!(VEC3_SHAPE.kind(), LpKind::Vec3);
    }

    #[test]
    fn test_vec3_value() {
        let value = Vec3::new(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO);
        let shape = value.shape();
        assert_eq!(shape.kind(), LpKind::Vec3);
    }

    #[test]
    fn test_vec3_shape_with_meta() {
        use crate::kind::vec3::vec3_meta::Vec3MetaStatic;
        use crate::kind::vec3::vec3_shape::Vec3Shape;
        use crate::kind::vec3::vec3_static::Vec3ShapeStatic;

        let meta = Vec3MetaStatic {
            label: "Position",
            desc_md: Some("3D position coordinates"),
            unit: Some("px"),
        };
        let shape = Vec3ShapeStatic::with_meta(meta);
        assert_eq!(shape.kind(), LpKind::Vec3);

        // Demonstrate Vec3Shape trait usage for polymorphic access
        let shape_ref: &dyn Vec3Shape = &shape;
        let meta_ref = shape_ref.meta().unwrap();
        assert_eq!(meta_ref.label(), "Position");
        assert_eq!(meta_ref.desc_md(), Some("3D position coordinates"));
        assert_eq!(meta_ref.unit(), Some("px"));
    }

    #[test]
    fn test_vec3_value_box() {
        use crate::kind::value::LpValueBox;

        let value = Vec3::new(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO);
        let value_box: LpValueBox = value.into();
        match value_box {
            LpValueBox::Vec3(_) => {}
            _ => panic!("Expected Vec3 variant"),
        }
    }
}
