//! Type kinds without metadata.

/// Type category without any metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LpKind {
    // Primitives
    Int32,
    Fixed,
    Bool,
    String,
    // Vectors
    Vec2,
    Vec3,
    Vec4,
    // Composites
    Option,
    Tuple,
    Record,
    Array,
    Map,
    Enum,
}
