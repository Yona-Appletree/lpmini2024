/// Local variable storage types for LPS VM
extern crate alloc;
use crate::math::Fixed;
use alloc::string::String;
use alloc::vec::Vec;

/// Storage for local variables
#[derive(Debug, Clone)]
pub enum LocalType {
    Fixed(Fixed),
    Int32(i32),
    Vec2(Fixed, Fixed),
    Vec3(Fixed, Fixed, Fixed),
    Vec4(Fixed, Fixed, Fixed, Fixed),
    Int32Array {
        data: Vec<i32>,
    },
    Texture2dR {
        data: Vec<u8>,
        width: usize,
        height: usize,
    },
    Texture2dRgba {
        data: Vec<u32>,
        width: usize,
        height: usize,
    },
}

impl LocalType {
    /// Get the type name for error reporting
    pub fn type_name(&self) -> &'static str {
        match self {
            LocalType::Fixed(_) => "float",
            LocalType::Int32(_) => "int",
            LocalType::Vec2(_, _) => "vec2",
            LocalType::Vec3(_, _, _) => "vec3",
            LocalType::Vec4(_, _, _, _) => "vec4",
            LocalType::Int32Array { .. } => "int[]",
            LocalType::Texture2dR { .. } => "sampler2D",
            LocalType::Texture2dRgba { .. } => "sampler2D",
        }
    }
}

/// Definition of a local variable
#[derive(Debug, Clone)]
pub struct LocalDef {
    pub name: String,
    pub ty: LocalType,
    pub access: LocalAccess,
}

impl LocalDef {
    pub fn new(name: String, ty: LocalType, access: LocalAccess) -> Self {
        LocalDef { name, ty, access }
    }
}

/// Access mode for local variables
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalAccess {
    /// Read-only (shared across invocations, like textures)
    Input,
    /// Read-write scratch space
    Scratch,
    /// Write-only output
    Output,
}
