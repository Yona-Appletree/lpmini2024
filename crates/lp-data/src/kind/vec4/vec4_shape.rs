//! Schema types for Vec4 shapes.

crate::define_primitive_shape! {
    name: Vec4,
    kind: Vec4,
    shape_trait: Vec4Shape,
    meta_trait: Vec4Meta,
    meta_static: Vec4MetaStatic,
    meta_dyn: Vec4MetaDyn,
    shape_static: Vec4ShapeStatic,
    shape_dyn: Vec4ShapeDyn,
    shape_const: VEC4_SHAPE,
}
