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
