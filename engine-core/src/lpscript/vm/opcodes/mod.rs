/// OpCode definitions for LPS VM
/// 
/// Design: Hybrid approach - small constants (indices, offsets) embedded in opcodes,
/// data values flow through stack.
use crate::math::Fixed;
use crate::test_engine::LoadSource;

pub mod fixed;
pub mod comparisons;
pub mod int32;
pub mod vec2;
pub mod stack;
pub mod control;

/// New typed OpCode enum (not yet in use - will replace test_engine::OpCode during migration)
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum LpsOpCode {
    // Stack operations
    Push(Fixed),
    PushInt32(i32),
    Dup,
    Drop,
    Swap,

    // Fixed-point arithmetic
    AddFixed,
    SubFixed,
    MulFixed,
    DivFixed,
    NegFixed,
    AbsFixed,
    MinFixed,
    MaxFixed,
    SinFixed,
    CosFixed,
    TanFixed,
    AtanFixed,     // Single arg atan
    Atan2Fixed,    // Two arg atan2
    SqrtFixed,
    FloorFixed,
    CeilFixed,
    FractFixed,    // Fractional part
    ModFixed,      // Modulo
    PowFixed,      // Power
    SignFixed,     // Sign function
    SaturateFixed, // Clamp to 0..1
    ClampFixed,    // Clamp to min..max
    StepFixed,     // Step function
    LerpFixed,     // Linear interpolation
    SmoothstepFixed, // Smooth interpolation
    
    // Noise functions
    Perlin3(u8),  // 3D Perlin noise, octaves embedded

    // Fixed-point comparisons (return FIXED_ONE or 0)
    GreaterFixed,
    LessFixed,
    GreaterEqFixed,
    LessEqFixed,
    EqFixed,
    NotEqFixed,
    
    // Logical operations (treat 0 as false, non-zero as true)
    AndFixed,
    OrFixed,
    NotFixed,

    // Int32 arithmetic
    AddInt32,
    SubInt32,
    MulInt32,
    DivInt32,
    ModInt32,
    NegInt32,
    AbsInt32,
    MinInt32,
    MaxInt32,

    // Int32 comparisons (return 1 or 0)
    GreaterInt32,
    LessInt32,
    GreaterEqInt32,
    LessEqInt32,
    EqInt32,
    NotEqInt32,

    // Vec2 operations (operate on stack)
    AddVec2,        // pop 4, push 2
    SubVec2,        // pop 4, push 2
    MulVec2,        // pop 4, push 2 (component-wise)
    DivVec2,        // pop 4, push 2 (component-wise)
    MulVec2Scalar,  // pop 3 (vec2 + scalar), push 2
    DivVec2Scalar,  // pop 3 (vec2 + scalar), push 2
    Dot2,           // pop 4, push 1
    Length2,        // pop 2, push 1
    Normalize2,     // pop 2, push 2
    Distance2,      // pop 4, push 1

    // Vec3 operations
    AddVec3,        // pop 6, push 3
    SubVec3,        // pop 6, push 3
    MulVec3,        // pop 6, push 3 (component-wise)
    DivVec3,        // pop 6, push 3 (component-wise)
    MulVec3Scalar,  // pop 4 (vec3 + scalar), push 3
    DivVec3Scalar,  // pop 4 (vec3 + scalar), push 3
    Dot3,           // pop 6, push 1
    Cross3,         // pop 6, push 3
    Length3,        // pop 3, push 1
    Normalize3,     // pop 3, push 3
    Distance3,      // pop 6, push 1

    // Vec4 operations
    AddVec4,        // pop 8, push 4
    SubVec4,        // pop 8, push 4
    MulVec4,        // pop 8, push 4 (component-wise)
    DivVec4,        // pop 8, push 4 (component-wise)
    MulVec4Scalar,  // pop 5 (vec4 + scalar), push 4
    DivVec4Scalar,  // pop 5 (vec4 + scalar), push 4
    Dot4,           // pop 8, push 1
    Length4,        // pop 4, push 1
    Normalize4,     // pop 4, push 4
    Distance4,      // pop 8, push 1

    // Texture sampling (local index embedded, UV coords on stack)
    TextureSampleR(u32),    // pop 2 Fixed (UV), push 1 Fixed (R)
    TextureSampleRGBA(u32), // pop 2 Fixed (UV), push 4 Fixed (RGBA)

    // Local variables (index and type embedded for safety)
    LoadLocalFixed(u32),
    StoreLocalFixed(u32),
    LoadLocalInt32(u32),
    StoreLocalInt32(u32),
    LoadLocalVec2(u32),
    StoreLocalVec2(u32),
    LoadLocalVec3(u32),
    StoreLocalVec3(u32),
    LoadLocalVec4(u32),
    StoreLocalVec4(u32),

    // Array operations
    GetElemInt32ArrayFixed,  // pop array_ref, index; push Fixed
    GetElemInt32ArrayU8,     // pop array_ref, index; push 4 Fixed (RGBA as bytes)

    // Control flow
    Jump(i32),          // Unconditional jump by offset
    JumpIfZero(i32),    // Pop value, jump if zero
    JumpIfNonZero(i32), // Pop value, jump if non-zero
    Select,             // Pop false_val, true_val, condition; push selected
    Call(u32),          // Call user-defined function at offset (pushes return address)
    Return,             // Return from function (pops return address, or exits if main)

    // Coordinate loading (legacy compatibility)
    Load(LoadSource),
}

impl LpsOpCode {
    /// Get a human-readable name for the opcode
    pub fn name(&self) -> &'static str {
        match self {
            LpsOpCode::Push(_) => "Push",
            LpsOpCode::PushInt32(_) => "PushInt32",
            LpsOpCode::Dup => "Dup",
            LpsOpCode::Drop => "Drop",
            LpsOpCode::Swap => "Swap",
            LpsOpCode::AddFixed => "AddFixed",
            LpsOpCode::SubFixed => "SubFixed",
            LpsOpCode::MulFixed => "MulFixed",
            LpsOpCode::DivFixed => "DivFixed",
            LpsOpCode::NegFixed => "NegFixed",
            LpsOpCode::AbsFixed => "AbsFixed",
            LpsOpCode::MinFixed => "MinFixed",
            LpsOpCode::MaxFixed => "MaxFixed",
            LpsOpCode::SinFixed => "SinFixed",
            LpsOpCode::CosFixed => "CosFixed",
            LpsOpCode::TanFixed => "TanFixed",
            LpsOpCode::AtanFixed => "AtanFixed",
            LpsOpCode::Atan2Fixed => "Atan2Fixed",
            LpsOpCode::SqrtFixed => "SqrtFixed",
            LpsOpCode::FloorFixed => "FloorFixed",
            LpsOpCode::CeilFixed => "CeilFixed",
            LpsOpCode::FractFixed => "FractFixed",
            LpsOpCode::ModFixed => "ModFixed",
            LpsOpCode::PowFixed => "PowFixed",
            LpsOpCode::SignFixed => "SignFixed",
            LpsOpCode::SaturateFixed => "SaturateFixed",
            LpsOpCode::ClampFixed => "ClampFixed",
            LpsOpCode::StepFixed => "StepFixed",
            LpsOpCode::LerpFixed => "LerpFixed",
            LpsOpCode::SmoothstepFixed => "SmoothstepFixed",
            LpsOpCode::Perlin3(_) => "Perlin3",
            LpsOpCode::GreaterFixed => "GreaterFixed",
            LpsOpCode::LessFixed => "LessFixed",
            LpsOpCode::GreaterEqFixed => "GreaterEqFixed",
            LpsOpCode::LessEqFixed => "LessEqFixed",
            LpsOpCode::EqFixed => "EqFixed",
            LpsOpCode::NotEqFixed => "NotEqFixed",
            LpsOpCode::AndFixed => "AndFixed",
            LpsOpCode::OrFixed => "OrFixed",
            LpsOpCode::NotFixed => "NotFixed",
            LpsOpCode::AddInt32 => "AddInt32",
            LpsOpCode::SubInt32 => "SubInt32",
            LpsOpCode::MulInt32 => "MulInt32",
            LpsOpCode::DivInt32 => "DivInt32",
            LpsOpCode::ModInt32 => "ModInt32",
            LpsOpCode::NegInt32 => "NegInt32",
            LpsOpCode::AbsInt32 => "AbsInt32",
            LpsOpCode::MinInt32 => "MinInt32",
            LpsOpCode::MaxInt32 => "MaxInt32",
            LpsOpCode::GreaterInt32 => "GreaterInt32",
            LpsOpCode::LessInt32 => "LessInt32",
            LpsOpCode::GreaterEqInt32 => "GreaterEqInt32",
            LpsOpCode::LessEqInt32 => "LessEqInt32",
            LpsOpCode::EqInt32 => "EqInt32",
            LpsOpCode::NotEqInt32 => "NotEqInt32",
            LpsOpCode::AddVec2 => "AddVec2",
            LpsOpCode::SubVec2 => "SubVec2",
            LpsOpCode::MulVec2 => "MulVec2",
            LpsOpCode::DivVec2 => "DivVec2",
            LpsOpCode::MulVec2Scalar => "MulVec2Scalar",
            LpsOpCode::DivVec2Scalar => "DivVec2Scalar",
            LpsOpCode::Dot2 => "Dot2",
            LpsOpCode::Length2 => "Length2",
            LpsOpCode::Normalize2 => "Normalize2",
            LpsOpCode::Distance2 => "Distance2",
            LpsOpCode::AddVec3 => "AddVec3",
            LpsOpCode::SubVec3 => "SubVec3",
            LpsOpCode::MulVec3 => "MulVec3",
            LpsOpCode::DivVec3 => "DivVec3",
            LpsOpCode::MulVec3Scalar => "MulVec3Scalar",
            LpsOpCode::DivVec3Scalar => "DivVec3Scalar",
            LpsOpCode::Dot3 => "Dot3",
            LpsOpCode::Cross3 => "Cross3",
            LpsOpCode::Length3 => "Length3",
            LpsOpCode::Normalize3 => "Normalize3",
            LpsOpCode::Distance3 => "Distance3",
            LpsOpCode::AddVec4 => "AddVec4",
            LpsOpCode::SubVec4 => "SubVec4",
            LpsOpCode::MulVec4 => "MulVec4",
            LpsOpCode::DivVec4 => "DivVec4",
            LpsOpCode::MulVec4Scalar => "MulVec4Scalar",
            LpsOpCode::DivVec4Scalar => "DivVec4Scalar",
            LpsOpCode::Dot4 => "Dot4",
            LpsOpCode::Length4 => "Length4",
            LpsOpCode::Normalize4 => "Normalize4",
            LpsOpCode::Distance4 => "Distance4",
            LpsOpCode::TextureSampleR(_) => "TextureSampleR",
            LpsOpCode::TextureSampleRGBA(_) => "TextureSampleRGBA",
            LpsOpCode::LoadLocalFixed(_) => "LoadLocalFixed",
            LpsOpCode::StoreLocalFixed(_) => "StoreLocalFixed",
            LpsOpCode::LoadLocalInt32(_) => "LoadLocalInt32",
            LpsOpCode::StoreLocalInt32(_) => "StoreLocalInt32",
            LpsOpCode::LoadLocalVec2(_) => "LoadLocalVec2",
            LpsOpCode::StoreLocalVec2(_) => "StoreLocalVec2",
            LpsOpCode::LoadLocalVec3(_) => "LoadLocalVec3",
            LpsOpCode::StoreLocalVec3(_) => "StoreLocalVec3",
            LpsOpCode::LoadLocalVec4(_) => "LoadLocalVec4",
            LpsOpCode::StoreLocalVec4(_) => "StoreLocalVec4",
            LpsOpCode::GetElemInt32ArrayFixed => "GetElemInt32ArrayFixed",
            LpsOpCode::GetElemInt32ArrayU8 => "GetElemInt32ArrayU8",
            LpsOpCode::Jump(_) => "Jump",
            LpsOpCode::JumpIfZero(_) => "JumpIfZero",
            LpsOpCode::JumpIfNonZero(_) => "JumpIfNonZero",
            LpsOpCode::Select => "Select",
            LpsOpCode::Call(_) => "Call",
            LpsOpCode::Return => "Return",
            LpsOpCode::Load(_) => "Load",
        }
    }
}
