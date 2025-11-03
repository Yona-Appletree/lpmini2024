/// Code generator: converts AST to VM opcodes
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

use super::ast::Expr;
use crate::test_engine::{OpCode, LoadSource};

// Helper to convert f32 to fixed-point
#[inline]
fn fixed_from_f32(val: f32) -> i32 {
    (val * 65536.0) as i32
}

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate(expr: &Expr) -> Vec<OpCode> {
        let mut code = Vec::new();
        Self::gen_expr(expr, &mut code);
        code.push(OpCode::Return);
        code
    }
    
    fn gen_expr(expr: &Expr, code: &mut Vec<OpCode>) {
        match expr {
            Expr::Number(n) => {
                code.push(OpCode::Push(fixed_from_f32(*n)));
            }
            
            Expr::Variable(name) => {
                let source = Self::variable_to_load_source(name);
                code.push(OpCode::Load(source));
            }
            
            // Binary operations
            Expr::Add(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::Add);
            }
            
            Expr::Sub(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::Sub);
            }
            
            Expr::Mul(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::Mul);
            }
            
            Expr::Div(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::Div);
            }
            
            Expr::Mod(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                // Implement modulo as: a - floor(a/b) * b
                // For now, emit placeholder - could add Mod opcode later
                code.push(OpCode::Div);
                code.push(OpCode::Frac);
                Self::gen_expr(right, code);
                code.push(OpCode::Mul);
            }
            
            Expr::Pow(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                // Use native function for power
                code.push(OpCode::CallNative(NativeFunction::Pow as u8));
            }
            
            // Comparisons (evaluate to 0 or 1)
            Expr::Less(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::Less as u8));
            }
            
            Expr::Greater(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::Greater as u8));
            }
            
            Expr::LessEq(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::LessEq as u8));
            }
            
            Expr::GreaterEq(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::GreaterEq as u8));
            }
            
            Expr::Eq(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::Eq as u8));
            }
            
            Expr::NotEq(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::NotEq as u8));
            }
            
            // Logical operations
            Expr::And(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::And as u8));
            }
            
            Expr::Or(left, right) => {
                Self::gen_expr(left, code);
                Self::gen_expr(right, code);
                code.push(OpCode::CallNative(NativeFunction::Or as u8));
            }
            
            // Ternary: condition ? true_expr : false_expr
            Expr::Ternary { condition, true_expr, false_expr } => {
                Self::gen_expr(condition, code);
                Self::gen_expr(true_expr, code);
                Self::gen_expr(false_expr, code);
                code.push(OpCode::CallNative(NativeFunction::Select as u8));
            }
            
            // Function calls
            Expr::Call { name, args } => {
                Self::gen_function_call(name, args, code);
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
                    if let Expr::Number(n) = args[3] {
                        n as u8
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
/// 
/// Common shader functions (GLSL/HLSL style):
/// - **Math**: min, max, pow, abs, floor, ceil, sqrt, sign
/// - **Clamping**: clamp, saturate, step
/// - **Interpolation**: lerp/mix, smoothstep
/// - **Comparisons**: <, >, <=, >=, ==, !=
/// - **Logical**: &&, ||
/// - **Ternary**: condition ? true_val : false_val
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
    Sign = 7,        // Returns -1, 0, or 1
    Saturate = 8,    // Clamp to 0..1 (like HLSL)
    Step = 9,        // step(edge, x) - returns 0 if x < edge, else 1
    
    // Utility
    Clamp = 10,      // clamp(value, min, max)
    Lerp = 11,       // lerp(a, b, t) aka mix in GLSL
    Smoothstep = 12, // smoothstep(edge0, edge1, x)
    
    // Comparisons (return 0 or 1)
    Less = 20,
    Greater = 21,
    LessEq = 22,
    GreaterEq = 23,
    Eq = 24,
    NotEq = 25,
    
    // Logical (treat non-zero as true)
    And = 30,
    Or = 31,
    
    // Ternary select (pops: condition, true_val, false_val)
    Select = 40,
}

