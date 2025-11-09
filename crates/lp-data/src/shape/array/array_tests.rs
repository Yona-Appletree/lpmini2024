//! Tests for array shapes.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::array::StaticArrayShape;
    use crate::shape::kind::LpKind;
    use crate::shape::shape_ref::ShapeRef;

    #[test]
    fn test_static_array_shape() {
        let element = ShapeRef::fixed_default();
        let shape = StaticArrayShape {
            element,
            ui: crate::shape::array::ArrayUi::List,
        };

        assert_eq!(shape.kind(), LpKind::Array);
        // Can't compare ShapeRef directly, so we check the kind instead
        match shape.element() {
            ShapeRef::Fixed(_) => {}
            _ => panic!("Expected Fixed"),
        }
    }
}
