//! Schema types for Vec2 shapes.

crate::define_primitive_shape! {
    name: Vec2,
    kind: Vec2,
    shape_trait: Vec2Shape,
    meta_trait: Vec2Meta,
    meta_static: Vec2MetaStatic,
    meta_dyn: Vec2MetaDyn,
    shape_static: Vec2ShapeStatic,
    shape_dyn: Vec2ShapeDyn,
    shape_const: VEC2_SHAPE,
}
