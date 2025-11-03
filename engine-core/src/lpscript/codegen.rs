/// Code generator: converts AST to VM opcodes
extern crate alloc;
use alloc::vec::Vec;

use super::ast::{Expr, ExprKind};
use crate::test_engine::{OpCode, LoadSource};
use crate::math::ToFixed;

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate(expr: &Expr) -> Vec<OpCode> {
        let mut code = Vec::new();
        Self::gen_expr(expr, &mut code);
        code.push(OpCode::Return);
        code
    }
    
    fn gen_expr(expr: &Expr, code: &mut Vec<OpCode>) {
        match &expr.kind {
            ExprKind::Number(n) => {
                code.push(OpCode::Push((*n).to_fixed()));
            }
            
            ExprKind::IntNumber(n) => {
                // Convert int to fixed point
                code.push(OpCode::Push((*n).to_fixed()));
            }
            
            ExprKind::Variable(name) => {
                let source = Self::variable_to_load_source(name);
                code.push(OpCode::Load(source));
            }
            
            // Binary operations
            ExprKind::Add(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::Add);
            }
            
            ExprKind::Sub(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::Sub);
            }
            
            ExprKind::Mul(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::Mul);
            }
            
            ExprKind::Div(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::Div);
            }
            
            ExprKind::Mod(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                // Implement modulo as: a - floor(a/b) * b
                code.push(OpCode::Div);
                code.push(OpCode::Frac);
                Self::gen_expr(right, code);
                code.push(OpCode::Mul);
            }
            
            ExprKind::Pow(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::Pow as u8));
            }
            
            // Comparisons
            ExprKind::Less(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::Less as u8));
            }
            
            ExprKind::Greater(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::Greater as u8));
            }
            
            ExprKind::LessEq(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::LessEq as u8));
            }
            
            ExprKind::GreaterEq(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::GreaterEq as u8));
            }
            
            ExprKind::Eq(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::Eq as u8));
            }
            
            ExprKind::NotEq(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::NotEq as u8));
            }
            
            // Logical operations
            ExprKind::And(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::And as u8));
            }
            
            ExprKind::Or(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::Or as u8));
            }
            
            // Ternary
            ExprKind::Ternary { condition, true_expr, false_expr } => {
                Self::gen_expr(condition, code);
                Self::gen_expr(true_expr, code);
                Self::gen_expr(false_expr, code);
                code.push(OpCode::CallNative(NativeFunction::Select as u8));
            }
            
            // Function calls
            ExprKind::Call { name, args } => {
                Self::gen_function_call(name, args, code);
            }
            
            // Vector constructors - for now, just push components individually
            ExprKind::Vec2Constructor(x, y) => {
                Self::gen_expr(x, code);
                Self::gen_expr(y, code);
                // Components are now on stack
            }
            
            ExprKind::Vec3Constructor(x, y, z) => {
                Self::gen_expr(x, code);
                Self::gen_expr(y, code);
                Self::gen_expr(z, code);
            }
            
            ExprKind::Vec4Constructor(x, y, z, w) => {
                Self::gen_expr(x, code);
                Self::gen_expr(y, code);
                Self::gen_expr(z, code);
                Self::gen_expr(w, code);
            }
        }
    }
    
    fn gen_function_call(name: &str, args: &[Expr], code: &mut Vec<OpCode>) {
        // Generate code for all arguments first
        for arg in args {
            Self::gen_expr(arg, code);
        }
        
        // Emit the appropriate instruction
        match name {
            "sin" => code.push(OpCode::Sin),
            "cos" => code.push(OpCode::Cos),
            "frac" => code.push(OpCode::Frac),
            
            "perlin3" => {
                let octaves = if args.len() >= 4 {
                    // Try to extract constant octaves from last arg
                    if let ExprKind::Number(n) = &args[3].kind {
                        *n as u8
                    } else if let ExprKind::IntNumber(n) = &args[3].kind {
                        *n as u8
                    } else {
                        3 // Default
                    }
                } else {
                    3 // Default
                };
                code.push(OpCode::Perlin3(octaves));
            }
            
            // Native functions - math
            "min" => code.push(OpCode::CallNative(NativeFunction::Min as u8)),
            "max" => code.push(OpCode::CallNative(NativeFunction::Max as u8)),
            "pow" => code.push(OpCode::CallNative(NativeFunction::Pow as u8)),
            "abs" => code.push(OpCode::CallNative(NativeFunction::Abs as u8)),
            "floor" => code.push(OpCode::CallNative(NativeFunction::Floor as u8)),
            "ceil" => code.push(OpCode::CallNative(NativeFunction::Ceil as u8)),
            "sqrt" => code.push(OpCode::CallNative(NativeFunction::Sqrt as u8)),
            "sign" => code.push(OpCode::CallNative(NativeFunction::Sign as u8)),
            
            // Clamping/interpolation
            "clamp" => code.push(OpCode::CallNative(NativeFunction::Clamp as u8)),
            "saturate" => code.push(OpCode::CallNative(NativeFunction::Saturate as u8)),
            "step" => code.push(OpCode::CallNative(NativeFunction::Step as u8)),
            "lerp" | "mix" => code.push(OpCode::CallNative(NativeFunction::Lerp as u8)),
            "smoothstep" => code.push(OpCode::CallNative(NativeFunction::Smoothstep as u8)),
            
            _ => {} // Unknown function - ignore
        }
    }
    
    fn variable_to_load_source(name: &str) -> LoadSource {
        match name {
            "x" | "xNorm" => LoadSource::XNorm,
            "y" | "yNorm" => LoadSource::YNorm,
            "time" | "t" => LoadSource::Time,
            "timeNorm" => LoadSource::TimeNorm,
            "centerAngle" | "angle" => LoadSource::CenterAngle,
            "centerDist" | "dist" => LoadSource::CenterDist,
            _ => LoadSource::XNorm, // Default fallback
        }
    }
}

/// Native function IDs for CallNative opcode
#[repr(u8)]
pub enum NativeFunction {
    // Math basics
    Min = 0,
    Max = 1,
    Pow = 2,
    Abs = 3,
    Floor = 4,
    Ceil = 5,
    Sqrt = 6,
    Sign = 7,
    Saturate = 8,
    Step = 9,
    
    // Utility
    Clamp = 10,
    Lerp = 11,
    Smoothstep = 12,
    
    // Comparisons
    Less = 20,
    Greater = 21,
    LessEq = 22,
    GreaterEq = 23,
    Eq = 24,
    NotEq = 25,
    
    // Logical
    And = 30,
    Or = 31,
    
    // Ternary select
    Select = 40,
}
