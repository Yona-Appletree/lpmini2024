/// OpCode definitions for LPS VM
///
/// Design: Hybrid approach - small constants (indices, offsets) embedded in opcodes,
/// data values flow through stack.
use crate::math::Fixed;
use crate::test_engine::LoadSource;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
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
    SqrtFixed,
    FloorFixed,
    CeilFixed,

    // Fixed-point comparisons (return FIXED_ONE or 0)
    GreaterFixed,
    LessFixed,
    GreaterEqFixed,
    LessEqFixed,
    EqFixed,
    NotEqFixed,

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
    AddVec2,       // pop 4, push 2
    SubVec2,       // pop 4, push 2
    MulVec2,       // pop 4, push 2 (component-wise)
    DivVec2,       // pop 4, push 2 (component-wise)
    MulVec2Scalar, // pop 3 (vec2 + scalar), push 2
    DivVec2Scalar, // pop 3 (vec2 + scalar), push 2
    Dot2,          // pop 4, push 1
    Length2,       // pop 2, push 1
    Normalize2,    // pop 2, push 2
    Distance2,     // pop 4, push 1

    // Vec3 operations
    AddVec3,       // pop 6, push 3
    SubVec3,       // pop 6, push 3
    MulVec3,       // pop 6, push 3 (component-wise)
    DivVec3,       // pop 6, push 3 (component-wise)
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
    MulVec4,       // pop 8, push 4 (component-wise)
    DivVec4,       // pop 8, push 4 (component-wise)
    MulVec4Scalar, // pop 5 (vec4 + scalar), push 4
    DivVec4Scalar, // pop 5 (vec4 + scalar), push 4
    Dot4,          // pop 8, push 1
    Length4,       // pop 4, push 1
    Normalize4,    // pop 4, push 4
    Distance4,     // pop 8, push 1

    // Texture sampling (local index embedded, UV coords on stack)
    TextureSample_R(u32),    // pop 2 Fixed (UV), push 1 Fixed (R)
    TextureSample_RGBA(u32), // pop 2 Fixed (UV), push 4 Fixed (RGBA)

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
    GetElem_Int32Array_Fixed, // pop array_ref, index; push Fixed
    GetElem_Int32Array_U8,    // pop array_ref, index; push 4 Fixed (RGBA as bytes)

    // Control flow
    Jump(i32),          // Unconditional jump by offset
    JumpIfZero(i32),    // Pop value, jump if zero
    JumpIfNonZero(i32), // Pop value, jump if non-zero
    Select,             // Pop false_val, true_val, condition; push selected

    // Coordinate loading (legacy compatibility)
    Load(LoadSource),

    // End execution
    Return,
}

impl OpCode {
    /// Get a human-readable name for the opcode
    pub fn name(&self) -> &'static str {
        match self {
            OpCode::Push(_) => "Push",
            OpCode::PushInt32(_) => "PushInt32",
            OpCode::Dup => "Dup",
            OpCode::Drop => "Drop",
            OpCode::Swap => "Swap",
            OpCode::AddFixed => "AddFixed",
            OpCode::SubFixed => "SubFixed",
            OpCode::MulFixed => "MulFixed",
            OpCode::DivFixed => "DivFixed",
            OpCode::NegFixed => "NegFixed",
            OpCode::AbsFixed => "AbsFixed",
            OpCode::MinFixed => "MinFixed",
            OpCode::MaxFixed => "MaxFixed",
            OpCode::SinFixed => "SinFixed",
            OpCode::CosFixed => "CosFixed",
            OpCode::SqrtFixed => "SqrtFixed",
            OpCode::FloorFixed => "FloorFixed",
            OpCode::CeilFixed => "CeilFixed",
            OpCode::GreaterFixed => "GreaterFixed",
            OpCode::LessFixed => "LessFixed",
            OpCode::GreaterEqFixed => "GreaterEqFixed",
            OpCode::LessEqFixed => "LessEqFixed",
            OpCode::EqFixed => "EqFixed",
            OpCode::NotEqFixed => "NotEqFixed",
            OpCode::AddInt32 => "AddInt32",
            OpCode::SubInt32 => "SubInt32",
            OpCode::MulInt32 => "MulInt32",
            OpCode::DivInt32 => "DivInt32",
            OpCode::ModInt32 => "ModInt32",
            OpCode::NegInt32 => "NegInt32",
            OpCode::AbsInt32 => "AbsInt32",
            OpCode::MinInt32 => "MinInt32",
            OpCode::MaxInt32 => "MaxInt32",
            OpCode::GreaterInt32 => "GreaterInt32",
            OpCode::LessInt32 => "LessInt32",
            OpCode::GreaterEqInt32 => "GreaterEqInt32",
            OpCode::LessEqInt32 => "LessEqInt32",
            OpCode::EqInt32 => "EqInt32",
            OpCode::NotEqInt32 => "NotEqInt32",
            OpCode::AddVec2 => "AddVec2",
            OpCode::SubVec2 => "SubVec2",
            OpCode::MulVec2 => "MulVec2",
            OpCode::DivVec2 => "DivVec2",
            OpCode::MulVec2Scalar => "MulVec2Scalar",
            OpCode::DivVec2Scalar => "DivVec2Scalar",
            OpCode::Dot2 => "Dot2",
            OpCode::Length2 => "Length2",
            OpCode::Normalize2 => "Normalize2",
            OpCode::Distance2 => "Distance2",
            OpCode::AddVec3 => "AddVec3",
            OpCode::SubVec3 => "SubVec3",
            OpCode::MulVec3 => "MulVec3",
            OpCode::DivVec3 => "DivVec3",
            OpCode::MulVec3Scalar => "MulVec3Scalar",
            OpCode::DivVec3Scalar => "DivVec3Scalar",
            OpCode::Dot3 => "Dot3",
            OpCode::Cross3 => "Cross3",
            OpCode::Length3 => "Length3",
            OpCode::Normalize3 => "Normalize3",
            OpCode::Distance3 => "Distance3",
            OpCode::AddVec4 => "AddVec4",
            OpCode::SubVec4 => "SubVec4",
            OpCode::MulVec4 => "MulVec4",
            OpCode::DivVec4 => "DivVec4",
            OpCode::MulVec4Scalar => "MulVec4Scalar",
            OpCode::DivVec4Scalar => "DivVec4Scalar",
            OpCode::Dot4 => "Dot4",
            OpCode::Length4 => "Length4",
            OpCode::Normalize4 => "Normalize4",
            OpCode::Distance4 => "Distance4",
            OpCode::TextureSample_R(_) => "TextureSample_R",
            OpCode::TextureSample_RGBA(_) => "TextureSample_RGBA",
            OpCode::LoadLocalFixed(_) => "LoadLocalFixed",
            OpCode::StoreLocalFixed(_) => "StoreLocalFixed",
            OpCode::LoadLocalInt32(_) => "LoadLocalInt32",
            OpCode::StoreLocalInt32(_) => "StoreLocalInt32",
            OpCode::LoadLocalVec2(_) => "LoadLocalVec2",
            OpCode::StoreLocalVec2(_) => "StoreLocalVec2",
            OpCode::LoadLocalVec3(_) => "LoadLocalVec3",
            OpCode::StoreLocalVec3(_) => "StoreLocalVec3",
            OpCode::LoadLocalVec4(_) => "LoadLocalVec4",
            OpCode::StoreLocalVec4(_) => "StoreLocalVec4",
            OpCode::GetElem_Int32Array_Fixed => "GetElem_Int32Array_Fixed",
            OpCode::GetElem_Int32Array_U8 => "GetElem_Int32Array_U8",
            OpCode::Jump(_) => "Jump",
            OpCode::JumpIfZero(_) => "JumpIfZero",
            OpCode::JumpIfNonZero(_) => "JumpIfNonZero",
            OpCode::Select => "Select",
            OpCode::Load(_) => "Load",
            OpCode::Return => "Return",
        }
    }
}
