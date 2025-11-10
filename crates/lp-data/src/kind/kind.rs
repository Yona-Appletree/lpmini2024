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

    /// Enum (simple unit enum with no fields)
    ///
    /// Note: Union (discriminated unions with fields) is planned for future work.
    Enum,
}
