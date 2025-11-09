//! Tests for option shapes.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::kind::LpKind;
    use crate::shape::option::StaticOptionShape;
    use crate::shape::shape_ref::ShapeRef;

    #[test]
    fn test_static_option_shape() {
        let inner = ShapeRef::fixed_default();
        let shape = StaticOptionShape { inner };

        assert_eq!(shape.kind(), LpKind::Option);
        match shape.inner() {
            ShapeRef::Fixed(_) => {}
            _ => panic!("Expected Fixed"),
        }
    }
}
