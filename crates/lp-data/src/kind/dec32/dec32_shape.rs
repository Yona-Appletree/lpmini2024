//! Schema types for Dec32 shapes.

crate::define_primitive_shape! {
    name: Dec32,
    kind: Dec32,
    shape_trait: Dec32Shape,
    meta_trait: Dec32Meta,
    meta_static: Dec32MetaStatic,
    meta_dyn: Dec32MetaDyn,
    shape_static: Dec32ShapeStatic,
    shape_dyn: Dec32ShapeDyn,
    shape_const: DEC32_SHAPE,
}
