/// Function call code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;

use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::local_allocator::LocalAllocator;

pub fn gen_function_call(
    name: &str,
    args: &[Expr],
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    // Check if it's a user-defined function
    if let Some(&offset) = func_offsets.get(name) {
        // Generate code for arguments (push onto stack)
        for arg in args {
            gen_expr(arg, code, locals, func_offsets);
        }
        // Emit Call opcode with function offset
        code.push(LpsOpCode::Call(offset));
        return;
    }
    
    // Special case: perlin3(vec3) or perlin3(vec3, octaves)
    // Octaves is embedded in opcode, not pushed to stack
    if name == "perlin3" {
        // First arg is vec3, generate code to push its 3 components
        gen_expr(&args[0], code, locals, func_offsets);
        
        // Extract octaves from 2nd arg or use default
        let octaves = if args.len() >= 2 {
            if let ExprKind::Number(n) = &args[1].kind {
                *n as u8
            } else if let ExprKind::IntNumber(n) = &args[1].kind {
                *n as u8
            } else {
                3 // Default
            }
        } else {
            3 // Default
        };
        
        code.push(LpsOpCode::Perlin3(octaves));
        return;
    }
    
    // For all other functions, generate code for all arguments first
    for arg in args {
        gen_expr(arg, code, locals, func_offsets);
    }
    
    // Emit the appropriate instruction
    match name {
        "sin" => code.push(LpsOpCode::SinFixed),
        "cos" => code.push(LpsOpCode::CosFixed),
        "frac" | "fract" => code.push(LpsOpCode::FractFixed),
        
        // Math functions - use explicit opcodes
        "min" => code.push(LpsOpCode::MinFixed),
        "max" => code.push(LpsOpCode::MaxFixed),
        "abs" => code.push(LpsOpCode::AbsFixed),
        "floor" => code.push(LpsOpCode::FloorFixed),
        "ceil" => code.push(LpsOpCode::CeilFixed),
        "sqrt" => code.push(LpsOpCode::SqrtFixed),
        "tan" => code.push(LpsOpCode::TanFixed),
        "pow" => code.push(LpsOpCode::PowFixed),
        "sign" => code.push(LpsOpCode::SignFixed),
        "mod" => code.push(LpsOpCode::ModFixed),
        "atan" => {
            if args.len() == 2 {
                code.push(LpsOpCode::Atan2Fixed);
            } else {
                code.push(LpsOpCode::AtanFixed);
            }
        }
        
        // Clamping/interpolation
        "clamp" => code.push(LpsOpCode::ClampFixed),
        "saturate" => code.push(LpsOpCode::SaturateFixed),
        "step" => code.push(LpsOpCode::StepFixed),
        "lerp" | "mix" => code.push(LpsOpCode::LerpFixed),
        "smoothstep" => code.push(LpsOpCode::SmoothstepFixed),
        
        // Vector functions - use typed opcodes based on argument type
        "length" => {
            let arg_ty = args[0].ty.as_ref().unwrap();
            match arg_ty {
                Type::Vec2 => code.push(LpsOpCode::Length2),
                Type::Vec3 => code.push(LpsOpCode::Length3),
                Type::Vec4 => code.push(LpsOpCode::Length4),
                _ => {}
            }
        }
        "normalize" => {
            let arg_ty = args[0].ty.as_ref().unwrap();
            match arg_ty {
                Type::Vec2 => code.push(LpsOpCode::Normalize2),
                Type::Vec3 => code.push(LpsOpCode::Normalize3),
                Type::Vec4 => code.push(LpsOpCode::Normalize4),
                _ => {}
            }
        }
        "dot" => {
            let arg_ty = args[0].ty.as_ref().unwrap();
            match arg_ty {
                Type::Vec2 => code.push(LpsOpCode::Dot2),
                Type::Vec3 => code.push(LpsOpCode::Dot3),
                Type::Vec4 => code.push(LpsOpCode::Dot4),
                _ => {}
            }
        }
        "distance" => {
            let arg_ty = args[0].ty.as_ref().unwrap();
            match arg_ty {
                Type::Vec2 => code.push(LpsOpCode::Distance2),
                Type::Vec3 => code.push(LpsOpCode::Distance3),
                Type::Vec4 => code.push(LpsOpCode::Distance4),
                _ => {}
            }
        }
        "cross" => {
            // Always vec3
            code.push(LpsOpCode::Cross3);
        }
        
        _ => {} // Unknown function - ignore
    }
}

