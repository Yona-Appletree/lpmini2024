//! Schema types for Mat3 shapes.

crate::define_primitive_shape! {
    name: Mat3,
    kind: Mat3,
    shape_trait: Mat3Shape,
    meta_trait: Mat3Meta,
    meta_static: Mat3MetaStatic,
    meta_dyn: Mat3MetaDyn,
    shape_static: Mat3ShapeStatic,
    shape_dyn: Mat3ShapeDyn,
    shape_const: MAT3_SHAPE,
}
