/// Type system representation
extern crate alloc;
use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Bool,
    Dec32,
    Int32,
    Vec2,
    Vec3,
    Vec4,
    Mat3,
    Void,
}

impl Type {
    /// Calculate the size of this type in i32 units for storage
    ///
    /// This is used by LocalsStorage to allocate the correct amount of
    /// space in the raw i32 array for each local variable.
    pub fn size_in_i32s(&self) -> usize {
        match self {
            Type::Bool => 1,  // Stored as i32 (0 or 1)
            Type::Dec32 => 1, // Dec32-point 16.16 stored in i32
            Type::Int32 => 1, // Native i32
            Type::Vec2 => 2,  // 2x Dec32 (2x i32)
            Type::Vec3 => 3,  // 3x Dec32 (3x i32)
            Type::Vec4 => 4,  // 4x Dec32 (4x i32)
            Type::Mat3 => 9,  // 9x Dec32 (9x i32) - 3x3 matrix
            Type::Void => 0,  // No storage needed
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "bool"),
            Type::Dec32 => write!(f, "float"),
            Type::Int32 => write!(f, "int"),
            Type::Vec2 => write!(f, "vec2"),
            Type::Vec3 => write!(f, "vec3"),
            Type::Vec4 => write!(f, "vec4"),
            Type::Mat3 => write!(f, "mat3"),
            Type::Void => write!(f, "void"),
        }
    }
}
