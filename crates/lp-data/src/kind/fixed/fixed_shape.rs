//! Schema types for Fixed shapes.

crate::define_primitive_shape! {
    name: Fixed,
    kind: Fixed,
    shape_trait: FixedShape,
    meta_trait: FixedMeta,
    meta_static: FixedMetaStatic,
    meta_dyn: FixedMetaDyn,
    shape_static: FixedShapeStatic,
    shape_dyn: FixedShapeDyn,
    shape_const: FIXED_SHAPE,
}
