//! Tests for option shapes.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::kind::LpKind;
    use crate::shape::option::StaticOptionShape;
    use crate::shape::shape_ref::ShapeRef;

    #[test]
    fn test_static_option_shape() {
        let inner = ShapeRef::Fixed;
        let shape = StaticOptionShape { inner };

        assert_eq!(shape.kind(), LpKind::Option);
        assert_eq!(shape.inner(), &ShapeRef::Fixed);
    }
}
