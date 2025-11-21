// Re-export LoadSource from load module
pub use load::LoadSource;
/// OpCode definitions for LPS VM
///
/// Design: Hybrid approach - small constants (indices, offsets) embedded in opcodes,
/// data values flow through stack.
use lp_math::dec32::Dec32;

// Dec32-point opcodes (split into basic, advanced, logic)
pub mod fixed_advanced;
pub mod fixed_basic;
pub mod fixed_logic;

// Comparison opcodes
pub mod comparisons;

// Int32 opcodes
pub mod int32;
pub mod int32_compare;

// Vector opcodes
pub mod mat3;
pub mod vec2;
pub mod vec3;
pub mod vec4;

// Stack and control flow
pub mod control_flow;
pub mod stack;

pub use control_flow::ReturnAction;

// Local variables
pub mod locals;

// Load built-in variables
pub mod load;

// Texture and array operations
pub mod arrays;
pub mod textures;

/// New typed OpCode enum (not yet in use - will replace test_engine::OpCode during migration)
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum LpsOpCode {
    // Stack operations
    Push(Dec32),
    PushInt32(i32),
    Dup1,  // Duplicate top 1 stack value (for Dec32/Int32)
    Dup2,  // Duplicate top 2 stack values (for Vec2)
    Dup3,  // Duplicate top 3 stack values (for Vec3)
    Dup4,  // Duplicate top 4 stack values (for Vec4)
    Dup9,  // Duplicate top 9 stack values (for Mat3)
    Drop1, // Drop top 1 stack value
    Drop2, // Drop top 2 stack values
    Drop3, // Drop top 3 stack values
    Drop4, // Drop top 4 stack values
    Drop9, // Drop top 9 stack values (for Mat3)
    Swap,

    // Dec32 arithmetic (fixed-point 16.16 generally)
    AddDec32,
    SubDec32,
    MulDec32,
    DivDec32,
    NegDec32,
    AbsDec32,
    MinDec32,
    MaxDec32,
    SinDec32,
    CosDec32,
    TanDec32,
    AtanDec32,  // Single arg atan
    Atan2Dec32, // Two arg atan2
    SqrtDec32,
    FloorDec32,
    CeilDec32,
    FractDec32,      // Fractional part
    ModDec32,        // Modulo
    PowDec32,        // Power
    SignDec32,       // Sign function
    SaturateDec32,   // Clamp to 0..1
    ClampDec32,      // Clamp to min..max
    StepDec32,       // Step function
    LerpDec32,       // Linear interpolation
    SmoothstepDec32, // Smooth interpolation

    // Noise functions
    Perlin3(u8), // 3D Perlin noise, octaves embedded

    // Dec32-point comparisons (return Dec32::ONE.0 or 0)
    GreaterDec32,
    LessDec32,
    GreaterEqDec32,
    LessEqDec32,
    EqDec32,
    NotEqDec32,

    // Logical operations (treat 0 as false, non-zero as true)
    AndDec32,
    OrDec32,
    NotDec32,

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

    // Int32 bitwise operations
    BitwiseAndInt32,
    BitwiseOrInt32,
    BitwiseXorInt32,
    BitwiseNotInt32,
    LeftShiftInt32,
    RightShiftInt32,

    // Type conversions
    Int32ToDec32, // Convert Int32 to Dec32 (multiply by 2^16)
    FixedToInt32, // Convert Dec32 to Int32 (divide by 2^16, truncate)

    // Vec2 operations (operate on stack)
    AddVec2,       // pop 4, push 2
    SubVec2,       // pop 4, push 2
    NegVec2,       // pop 2, push 2 (negate components)
    MulVec2,       // pop 4, push 2 (component-wise)
    DivVec2,       // pop 4, push 2 (component-wise)
    ModVec2,       // pop 4, push 2 (component-wise)
    MulVec2Scalar, // pop 3 (vec2 + scalar), push 2
    DivVec2Scalar, // pop 3 (vec2 + scalar), push 2
    Dot2,          // pop 4, push 1
    Length2,       // pop 2, push 1
    Normalize2,    // pop 2, push 2
    Distance2,     // pop 4, push 1

    // Vec3 operations
    AddVec3,       // pop 6, push 3
    SubVec3,       // pop 6, push 3
    NegVec3,       // pop 3, push 3 (negate components)
    MulVec3,       // pop 6, push 3 (component-wise)
    DivVec3,       // pop 6, push 3 (component-wise)
    ModVec3,       // pop 6, push 3 (component-wise)
    MulVec3Scalar, // pop 4 (vec3 + scalar), push 3
    DivVec3Scalar, // pop 4 (vec3 + scalar), push 3
    Dot3,          // pop 6, push 1
    Cross3,        // pop 6, push 3
    Length3,       // pop 3, push 1
    Normalize3,    // pop 3, push 3
    Distance3,     // pop 6, push 1

    // Vec4 operations
    AddVec4,       // pop 8, push 4
    SubVec4,       // pop 8, push 4
    NegVec4,       // pop 4, push 4 (negate components)
    MulVec4,       // pop 8, push 4 (component-wise)
    DivVec4,       // pop 8, push 4 (component-wise)
    ModVec4,       // pop 8, push 4 (component-wise)
    MulVec4Scalar, // pop 5 (vec4 + scalar), push 4
    DivVec4Scalar, // pop 5 (vec4 + scalar), push 4
    Dot4,          // pop 8, push 1
    Length4,       // pop 4, push 1
    Normalize4,    // pop 4, push 4
    Distance4,     // pop 8, push 1

    // Mat3 operations
    AddMat3,         // pop 18, push 9
    SubMat3,         // pop 18, push 9
    NegMat3,         // pop 9, push 9 (negate components)
    MulMat3,         // pop 18, push 9 (matrix multiplication)
    MulMat3Scalar,   // pop 10 (mat3 + scalar), push 9
    DivMat3Scalar,   // pop 10 (mat3 + scalar), push 9
    MulMat3Vec3,     // pop 12 (mat3 + vec3), push 3
    TransposeMat3,   // pop 9, push 9
    DeterminantMat3, // pop 9, push 1
    InverseMat3,     // pop 9, push 9 (returns identity if singular)

    // Swizzle operations (reorder stack values)
    Swizzle3to2(u8, u8),     // pop 3, push 2 (indices specify which 2 to keep)
    Swizzle3to3(u8, u8, u8), // pop 3, push 3 (indices specify reordering)
    Swizzle4to2(u8, u8),     // pop 4, push 2 (indices specify which 2 to keep)
    Swizzle4to3(u8, u8, u8), // pop 4, push 3 (indices specify which 3 to keep)
    Swizzle4to4(u8, u8, u8, u8), // pop 4, push 4 (indices specify reordering)

    // Texture sampling (local index embedded, UV coords on stack)
    TextureSampleR(u32),    // pop 2 Dec32 (UV), push 1 Dec32 (R)
    TextureSampleRGBA(u32), // pop 2 Dec32 (UV), push 4 Dec32 (RGBA)

    // Local variables (index and type embedded for safety)
    LoadLocalDec32(u32),
    StoreLocalDec32(u32),
    LoadLocalInt32(u32),
    StoreLocalInt32(u32),
    LoadLocalVec2(u32),
    StoreLocalVec2(u32),
    LoadLocalVec3(u32),
    StoreLocalVec3(u32),
    LoadLocalVec4(u32),
    StoreLocalVec4(u32),
    LoadLocalMat3(u32),
    StoreLocalMat3(u32),

    // Array operations
    GetElemInt32ArrayDec32, // pop array_ref, index; push Dec32
    GetElemInt32ArrayU8,    // pop array_ref, index; push 4 Dec32 (RGBA as bytes)

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
            LpsOpCode::Dup1 => "Dup1",
            LpsOpCode::Dup2 => "Dup2",
            LpsOpCode::Dup3 => "Dup3",
            LpsOpCode::Dup4 => "Dup4",
            LpsOpCode::Dup9 => "Dup9",
            LpsOpCode::Drop1 => "Drop1",
            LpsOpCode::Drop2 => "Drop2",
            LpsOpCode::Drop3 => "Drop3",
            LpsOpCode::Drop4 => "Drop4",
            LpsOpCode::Drop9 => "Drop9",
            LpsOpCode::Swap => "Swap",
            LpsOpCode::AddDec32 => "AddDec32",
            LpsOpCode::SubDec32 => "SubDec32",
            LpsOpCode::MulDec32 => "MulDec32",
            LpsOpCode::DivDec32 => "DivDec32",
            LpsOpCode::NegDec32 => "NegDec32",
            LpsOpCode::AbsDec32 => "AbsDec32",
            LpsOpCode::MinDec32 => "MinDec32",
            LpsOpCode::MaxDec32 => "MaxDec32",
            LpsOpCode::SinDec32 => "SinDec32",
            LpsOpCode::CosDec32 => "CosDec32",
            LpsOpCode::TanDec32 => "TanDec32",
            LpsOpCode::AtanDec32 => "AtanDec32",
            LpsOpCode::Atan2Dec32 => "Atan2Dec32",
            LpsOpCode::SqrtDec32 => "SqrtDec32",
            LpsOpCode::FloorDec32 => "FloorDec32",
            LpsOpCode::CeilDec32 => "CeilDec32",
            LpsOpCode::FractDec32 => "FractDec32",
            LpsOpCode::ModDec32 => "ModDec32",
            LpsOpCode::PowDec32 => "PowDec32",
            LpsOpCode::SignDec32 => "SignDec32",
            LpsOpCode::SaturateDec32 => "SaturateDec32",
            LpsOpCode::ClampDec32 => "ClampDec32",
            LpsOpCode::StepDec32 => "StepDec32",
            LpsOpCode::LerpDec32 => "LerpDec32",
            LpsOpCode::SmoothstepDec32 => "SmoothstepDec32",
            LpsOpCode::Perlin3(_) => "Perlin3",
            LpsOpCode::GreaterDec32 => "GreaterDec32",
            LpsOpCode::LessDec32 => "LessDec32",
            LpsOpCode::GreaterEqDec32 => "GreaterEqDec32",
            LpsOpCode::LessEqDec32 => "LessEqDec32",
            LpsOpCode::EqDec32 => "EqDec32",
            LpsOpCode::NotEqDec32 => "NotEqDec32",
            LpsOpCode::AndDec32 => "AndDec32",
            LpsOpCode::OrDec32 => "OrDec32",
            LpsOpCode::NotDec32 => "NotDec32",
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
            LpsOpCode::BitwiseAndInt32 => "BitwiseAndInt32",
            LpsOpCode::BitwiseOrInt32 => "BitwiseOrInt32",
            LpsOpCode::BitwiseXorInt32 => "BitwiseXorInt32",
            LpsOpCode::BitwiseNotInt32 => "BitwiseNotInt32",
            LpsOpCode::LeftShiftInt32 => "LeftShiftInt32",
            LpsOpCode::RightShiftInt32 => "RightShiftInt32",
            LpsOpCode::Int32ToDec32 => "Int32ToDec32",
            LpsOpCode::FixedToInt32 => "FixedToInt32",
            LpsOpCode::AddVec2 => "AddVec2",
            LpsOpCode::SubVec2 => "SubVec2",
            LpsOpCode::NegVec2 => "NegVec2",
            LpsOpCode::MulVec2 => "MulVec2",
            LpsOpCode::DivVec2 => "DivVec2",
            LpsOpCode::ModVec2 => "ModVec2",
            LpsOpCode::MulVec2Scalar => "MulVec2Scalar",
            LpsOpCode::DivVec2Scalar => "DivVec2Scalar",
            LpsOpCode::Dot2 => "Dot2",
            LpsOpCode::Length2 => "Length2",
            LpsOpCode::Normalize2 => "Normalize2",
            LpsOpCode::Distance2 => "Distance2",
            LpsOpCode::AddVec3 => "AddVec3",
            LpsOpCode::SubVec3 => "SubVec3",
            LpsOpCode::NegVec3 => "NegVec3",
            LpsOpCode::MulVec3 => "MulVec3",
            LpsOpCode::DivVec3 => "DivVec3",
            LpsOpCode::ModVec3 => "ModVec3",
            LpsOpCode::MulVec3Scalar => "MulVec3Scalar",
            LpsOpCode::DivVec3Scalar => "DivVec3Scalar",
            LpsOpCode::Dot3 => "Dot3",
            LpsOpCode::Cross3 => "Cross3",
            LpsOpCode::Length3 => "Length3",
            LpsOpCode::Normalize3 => "Normalize3",
            LpsOpCode::Distance3 => "Distance3",
            LpsOpCode::AddVec4 => "AddVec4",
            LpsOpCode::SubVec4 => "SubVec4",
            LpsOpCode::NegVec4 => "NegVec4",
            LpsOpCode::MulVec4 => "MulVec4",
            LpsOpCode::DivVec4 => "DivVec4",
            LpsOpCode::ModVec4 => "ModVec4",
            LpsOpCode::MulVec4Scalar => "MulVec4Scalar",
            LpsOpCode::DivVec4Scalar => "DivVec4Scalar",
            LpsOpCode::Dot4 => "Dot4",
            LpsOpCode::Length4 => "Length4",
            LpsOpCode::Normalize4 => "Normalize4",
            LpsOpCode::Distance4 => "Distance4",
            LpsOpCode::AddMat3 => "AddMat3",
            LpsOpCode::SubMat3 => "SubMat3",
            LpsOpCode::NegMat3 => "NegMat3",
            LpsOpCode::MulMat3 => "MulMat3",
            LpsOpCode::MulMat3Scalar => "MulMat3Scalar",
            LpsOpCode::DivMat3Scalar => "DivMat3Scalar",
            LpsOpCode::MulMat3Vec3 => "MulMat3Vec3",
            LpsOpCode::TransposeMat3 => "TransposeMat3",
            LpsOpCode::DeterminantMat3 => "DeterminantMat3",
            LpsOpCode::InverseMat3 => "InverseMat3",
            LpsOpCode::Swizzle3to2(_, _) => "Swizzle3to2",
            LpsOpCode::Swizzle3to3(_, _, _) => "Swizzle3to3",
            LpsOpCode::Swizzle4to2(_, _) => "Swizzle4to2",
            LpsOpCode::Swizzle4to3(_, _, _) => "Swizzle4to3",
            LpsOpCode::Swizzle4to4(_, _, _, _) => "Swizzle4to4",
            LpsOpCode::TextureSampleR(_) => "TextureSampleR",
            LpsOpCode::TextureSampleRGBA(_) => "TextureSampleRGBA",
            LpsOpCode::LoadLocalDec32(_) => "LoadLocalDec32",
            LpsOpCode::StoreLocalDec32(_) => "StoreLocalDec32",
            LpsOpCode::LoadLocalInt32(_) => "LoadLocalInt32",
            LpsOpCode::StoreLocalInt32(_) => "StoreLocalInt32",
            LpsOpCode::LoadLocalVec2(_) => "LoadLocalVec2",
            LpsOpCode::StoreLocalVec2(_) => "StoreLocalVec2",
            LpsOpCode::LoadLocalVec3(_) => "LoadLocalVec3",
            LpsOpCode::StoreLocalVec3(_) => "StoreLocalVec3",
            LpsOpCode::LoadLocalVec4(_) => "LoadLocalVec4",
            LpsOpCode::StoreLocalVec4(_) => "StoreLocalVec4",
            LpsOpCode::LoadLocalMat3(_) => "LoadLocalMat3",
            LpsOpCode::StoreLocalMat3(_) => "StoreLocalMat3",
            LpsOpCode::GetElemInt32ArrayDec32 => "GetElemInt32ArrayDec32",
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
