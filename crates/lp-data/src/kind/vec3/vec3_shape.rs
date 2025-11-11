//! Schema types for Vec3 shapes.

crate::define_primitive_shape! {
    name: Vec3,
    kind: Vec3,
    shape_trait: Vec3Shape,
    meta_trait: Vec3Meta,
    meta_static: Vec3MetaStatic,
    meta_dyn: Vec3MetaDyn,
    shape_static: Vec3ShapeStatic,
    shape_dyn: Vec3ShapeDyn,
    shape_const: VEC3_SHAPE,
}
