/// Type system representation
extern crate alloc;
use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Bool,
    Fixed,
    Int32,
    Vec2,
    Vec3,
    Vec4,
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
            Type::Fixed => 1, // Fixed-point 16.16 stored in i32
            Type::Int32 => 1, // Native i32
            Type::Vec2 => 2,  // 2x Fixed (2x i32)
            Type::Vec3 => 3,  // 3x Fixed (3x i32)
            Type::Vec4 => 4,  // 4x Fixed (4x i32)
            Type::Void => 0,  // No storage needed
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "bool"),
            Type::Fixed => write!(f, "float"),
            Type::Int32 => write!(f, "int"),
            Type::Vec2 => write!(f, "vec2"),
            Type::Vec3 => write!(f, "vec3"),
            Type::Vec4 => write!(f, "vec4"),
            Type::Void => write!(f, "void"),
        }
    }
}
