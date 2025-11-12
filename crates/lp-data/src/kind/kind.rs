//! LpKind enumeration and related types.

/// Basic kinds of data in the lp-data system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LpKind {
    /// Fixed-point number (16.16 format)
    Fixed,

    /// 32-bit signed integer
    Int32,

    /// Boolean value
    Bool,

    /// 2D vector (Vec2)
    Vec2,

    /// 3D vector (Vec3)
    Vec3,

    /// 4D vector (Vec4)
    Vec4,

    /// Record (struct-like composite type)
    Record,

    /// EnumUnit (simple unit enum with no fields)
    EnumUnit,

    /// EnumStruct (discriminated union with per-variant shapes and values)
    EnumStruct,

    /// Array (homogeneous collection of elements)
    Array,
}
