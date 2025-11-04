/// LPS Program definition
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use super::locals::LocalDef;
use super::opcodes::LpsOpCode;
use crate::lpscript::error::Span;

/// A compiled LightPlayer Script program
#[derive(Debug, Clone)]
pub struct LpsProgram {
    pub name: String,
    pub opcodes: Vec<LpsOpCode>,
    pub locals: Vec<LocalDef>,
    pub source_map: Option<Vec<Span>>,
    pub source: Option<String>,
}

impl LpsProgram {
    pub fn new(name: String) -> Self {
        LpsProgram {
            name,
            opcodes: Vec::new(),
            locals: Vec::new(),
            source_map: None,
            source: None,
        }
    }

    pub fn with_opcodes(mut self, opcodes: Vec<LpsOpCode>) -> Self {
        self.opcodes = opcodes;
        self
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_source_map(mut self, source_map: Vec<Span>) -> Self {
        self.source_map = Some(source_map);
        self
    }
    
    /// Convert LpsOpCode to legacy OpCode (temporary for migration)
    pub fn to_legacy_opcodes(&self) -> Vec<crate::test_engine::OpCode> {
        use crate::test_engine::OpCode;
        
        self.opcodes.iter().map(|op| {
            match op {
                LpsOpCode::Push(f) => OpCode::Push(*f),
                LpsOpCode::Dup => OpCode::Dup,
                LpsOpCode::Drop => OpCode::Drop,
                LpsOpCode::Swap => OpCode::Swap,
                LpsOpCode::AddFixed => OpCode::Add,
                LpsOpCode::SubFixed => OpCode::Sub,
                LpsOpCode::MulFixed => OpCode::Mul,
                LpsOpCode::DivFixed => OpCode::Div,
                LpsOpCode::SinFixed => OpCode::Sin,
                LpsOpCode::CosFixed => OpCode::Cos,
                LpsOpCode::FractFixed => OpCode::Frac,
                LpsOpCode::Perlin3(octaves) => OpCode::Perlin3(*octaves),
                LpsOpCode::Load(source) => OpCode::Load(*source),
                LpsOpCode::Return => OpCode::Return,
                
                // Vec2 arithmetic
                LpsOpCode::AddVec2 => OpCode::AddVec2,
                LpsOpCode::SubVec2 => OpCode::SubVec2,
                LpsOpCode::MulVec2 => OpCode::MulVec2,
                LpsOpCode::DivVec2 => OpCode::DivVec2,
                LpsOpCode::MulVec2Scalar => OpCode::MulVec2Scalar,
                LpsOpCode::DivVec2Scalar => OpCode::DivVec2Scalar,
                
                // Vec3 arithmetic
                LpsOpCode::AddVec3 => OpCode::AddVec3,
                LpsOpCode::SubVec3 => OpCode::SubVec3,
                LpsOpCode::MulVec3 => OpCode::MulVec3,
                LpsOpCode::DivVec3 => OpCode::DivVec3,
                LpsOpCode::MulVec3Scalar => OpCode::MulVec3Scalar,
                LpsOpCode::DivVec3Scalar => OpCode::DivVec3Scalar,
                
                // Vec4 arithmetic
                LpsOpCode::AddVec4 => OpCode::AddVec4,
                LpsOpCode::SubVec4 => OpCode::SubVec4,
                LpsOpCode::MulVec4 => OpCode::MulVec4,
                LpsOpCode::DivVec4 => OpCode::DivVec4,
                LpsOpCode::MulVec4Scalar => OpCode::MulVec4Scalar,
                LpsOpCode::DivVec4Scalar => OpCode::DivVec4Scalar,
                
                // Vector functions - map to typed opcodes in legacy VM
                LpsOpCode::Dot2 => OpCode::Dot2,
                LpsOpCode::Dot3 => OpCode::Dot3,
                LpsOpCode::Dot4 => OpCode::Dot4,
                LpsOpCode::Length2 => OpCode::Length2,
                LpsOpCode::Length3 => OpCode::Length3,
                LpsOpCode::Length4 => OpCode::Length4,
                LpsOpCode::Normalize2 => OpCode::Normalize2,
                LpsOpCode::Normalize3 => OpCode::Normalize3,
                LpsOpCode::Normalize4 => OpCode::Normalize4,
                LpsOpCode::Distance2 => OpCode::Distance2,
                LpsOpCode::Distance3 => OpCode::Distance3,
                LpsOpCode::Distance4 => OpCode::Distance4,
                LpsOpCode::Cross3 => OpCode::Cross3,
                
                // Comparisons - map to CallNative for now
                LpsOpCode::LessFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Less as u8),
                LpsOpCode::GreaterFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Greater as u8),
                LpsOpCode::LessEqFixed => OpCode::CallNative(crate::lpscript::NativeFunction::LessEq as u8),
                LpsOpCode::GreaterEqFixed => OpCode::CallNative(crate::lpscript::NativeFunction::GreaterEq as u8),
                LpsOpCode::EqFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Eq as u8),
                LpsOpCode::NotEqFixed => OpCode::CallNative(crate::lpscript::NativeFunction::NotEq as u8),
                LpsOpCode::AndFixed => OpCode::CallNative(crate::lpscript::NativeFunction::And as u8),
                LpsOpCode::OrFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Or as u8),
                LpsOpCode::Select => OpCode::CallNative(crate::lpscript::NativeFunction::Select as u8),
                
                // Math functions
                LpsOpCode::MinFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Min as u8),
                LpsOpCode::MaxFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Max as u8),
                LpsOpCode::AbsFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Abs as u8),
                LpsOpCode::FloorFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Floor as u8),
                LpsOpCode::CeilFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Ceil as u8),
                LpsOpCode::SqrtFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Sqrt as u8),
                LpsOpCode::TanFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Tan as u8),
                LpsOpCode::AtanFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Atan as u8),
                LpsOpCode::ModFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Mod as u8),
                LpsOpCode::PowFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Pow as u8),
                LpsOpCode::SignFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Sign as u8),
                LpsOpCode::SaturateFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Saturate as u8),
                LpsOpCode::ClampFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Clamp as u8),
                LpsOpCode::StepFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Step as u8),
                LpsOpCode::LerpFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Lerp as u8),
                LpsOpCode::SmoothstepFixed => OpCode::CallNative(crate::lpscript::NativeFunction::Smoothstep as u8),
                
                _ => OpCode::Return, // Unsupported opcodes default to Return (shouldn't happen)
            }
        }).collect()
    }
}
