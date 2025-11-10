//! Schema types for Bool shapes.

crate::define_primitive_shape! {
    name: Bool,
    kind: Bool,
    shape_trait: BoolShape,
    meta_trait: BoolMeta,
    meta_static: BoolMetaStatic,
    meta_dyn: BoolMetaDyn,
    shape_static: BoolShapeStatic,
    shape_dyn: BoolShapeDyn,
    shape_const: BOOL_SHAPE,
}
