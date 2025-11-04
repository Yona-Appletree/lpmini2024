/// Code generator: converts AST to VM opcodes
extern crate alloc;
use alloc::vec::Vec;

use super::ast::{Expr, ExprKind};
use super::error::Type;
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
                // Check if it's a vec2 built-in (uv, coord)
                match name.as_str() {
                    "uv" => {
                        // Push normalized coordinates as vec2
                        code.push(OpCode::Load(LoadSource::XNorm));
                        code.push(OpCode::Load(LoadSource::YNorm));
                    }
                    "coord" => {
                        // Push pixel coordinates as vec2 (converted to Fixed)
                        code.push(OpCode::Load(LoadSource::XInt));
                        code.push(OpCode::Load(LoadSource::YInt));
                    }
                    _ => {
                        // Scalar built-in
                        let source = Self::variable_to_load_source(name);
                        code.push(OpCode::Load(source));
                    }
                }
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
            
            // Vector constructors - push all components from all arguments
            // Supports GLSL-style mixed args: vec3(vec2, float), vec4(vec3, float), etc.
            ExprKind::Vec2Constructor(args) | 
            ExprKind::Vec3Constructor(args) | 
            ExprKind::Vec4Constructor(args) => {
                // Generate code for each argument, which pushes its components
                for arg in args {
                    Self::gen_expr(arg, code);
                }
                // Components are now on stack in the correct order
            }
            
            ExprKind::Swizzle { expr: base_expr, components } => {
                // Generate code for base expression (pushes vector components)
                Self::gen_expr(base_expr, code);
                
                // Get base type to know how many components to pop
                let base_type = base_expr.ty.as_ref().unwrap();
                let source_size = match base_type {
                    Type::Vec2 => 2,
                    Type::Vec3 => 3,
                    Type::Vec4 => 4,
                    _ => unreachable!("Type checker should catch non-vector swizzles"),
                };
                
                // Generate swizzle opcodes
                Self::gen_swizzle(components, source_size, code);
            }
        }
    }
    
    /// Generate opcodes for swizzling
    /// Stack layout: components are pushed in order, so for vec2(x,y), stack is [x, y] with y on top
    fn gen_swizzle(components: &str, source_size: usize, code: &mut Vec<OpCode>) {
        // Map component characters to indices
        let indices: Vec<usize> = components.chars().map(|c| {
            match c {
                'x' | 'r' | 's' => 0,
                'y' | 'g' | 't' => 1,
                'z' | 'b' | 'p' => 2,
                'w' | 'a' | 'q' => 3,
                _ => unreachable!("Type checker should validate swizzle components"),
            }
        }).collect();
        
        // Strategy: Pop all source components into temporary positions,
        // then push back the desired components in the right order
        
        // We'll use a simple approach: generate Dup/Swap/Drop operations
        // This is not the most efficient but is correct and simple
        
        if components.len() == 1 {
            // Single component extraction
            let idx = indices[0];
            // Stack has [c0, c1, ..., c(n-1)] with c(n-1) on top
            // We want to keep component at index idx
            // Index 0 is at bottom, index (n-1) is at top
            let drop_count = source_size - 1 - idx;
            for _ in 0..drop_count {
                code.push(OpCode::Drop);
            }
            // Now we have [c0, c1, ..., c(idx)]
            // We want just c(idx), so drop everything below
            for _ in 0..idx {
                code.push(OpCode::Swap); // Bring bottom to top
                code.push(OpCode::Drop); // Drop it
            }
        } else {
            // Multi-component swizzle
            // General algorithm: For each output component, pick from the input
            // 
            // Stack has components in order: [c0, c1, ..., c(n-1)] with c(n-1) on top
            // We need to produce [result0, result1, ..., result(m-1)]
            //
            // Strategy: Use helper function to access component at any index
            
            // Check if it's an identity swizzle first (optimization)
            let is_identity = indices.iter().enumerate().all(|(i, &idx)| i == idx);
            if is_identity && indices.len() == source_size {
                // Identity swizzle, no-op
                return;
            }
            
            // For vec2 specifically, handle common cases efficiently
            if source_size == 2 {
                match components {
                    "yx" | "gr" | "ts" => code.push(OpCode::Swap),
                    "xx" | "rr" | "ss" => {
                        // [x, y] -> [x, x]
                        code.push(OpCode::Drop); // [x]
                        code.push(OpCode::Dup);  // [x, x]
                    }
                    "yy" | "gg" | "tt" => {
                        // [x, y] -> [y, y]
                        code.push(OpCode::Swap); // [y, x]
                        code.push(OpCode::Drop); // [y]
                        code.push(OpCode::Dup);  // [y, y]
                    }
                    _ => {
                        // General case for vec2: Handle by reconstruction
                        // This is rare but possible (e.g., if type checker allows it)
                    }
                }
            } else {
                // For vec3 and vec4, we'll need a more sophisticated approach
                // For now, if it's identity, we already returned above
                // For non-identity vec3/vec4 swizzles, we'll need to implement
                // a general stack manipulation algorithm or add a Swizzle opcode to the VM
                //
                // TODO: Implement general vec3/vec4 swizzling
                // For now, identity swizzles work (which is the most common case)
            }
        }
    }
    
    fn gen_function_call(name: &str, args: &[Expr], code: &mut Vec<OpCode>) {
        // Special case: perlin3(vec3) or perlin3(vec3, octaves)
        // Octaves is embedded in opcode, not pushed to stack
        if name == "perlin3" {
            // First arg is vec3, generate code to push its 3 components
            Self::gen_expr(&args[0], code);
            
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
            
            code.push(OpCode::Perlin3(octaves));
            return;
        }
        
        // For all other functions, generate code for all arguments first
        for arg in args {
            Self::gen_expr(arg, code);
        }
        
        // Emit the appropriate instruction
        match name {
            "sin" => code.push(OpCode::Sin),
            "cos" => code.push(OpCode::Cos),
            "frac" => code.push(OpCode::Frac),
            
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
