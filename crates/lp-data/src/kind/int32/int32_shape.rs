//! Schema types for Int32 shapes.

crate::define_primitive_shape! {
    name: Int32,
    kind: Int32,
    shape_trait: Int32Shape,
    meta_trait: Int32Meta,
    meta_static: Int32MetaStatic,
    meta_dyn: Int32MetaDyn,
    shape_static: Int32ShapeStatic,
    shape_dyn: Int32ShapeDyn,
    shape_const: INT32_SHAPE,
}
