//! Tests for map shapes.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::kind::LpKind;
    use crate::shape::map::{DynamicMapShape, StaticMapShape};

    #[test]
    fn test_static_map_shape() {
        let shape = StaticMapShape;
        assert_eq!(shape.kind(), LpKind::Map);
    }

    #[test]
    fn test_dynamic_map_shape() {
        let shape = DynamicMapShape::new();
        assert_eq!(shape.kind(), LpKind::Map);
    }
}
