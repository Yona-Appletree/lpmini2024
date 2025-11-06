/// Function call code generation
extern crate alloc;

use crate::lpscript::compiler::ast::{AstPool, ExprId, ExprKind};
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::shared::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_function_call_id(&mut self, pool: &AstPool, name: &str, args: &[ExprId]) {
        // Check if it's a user-defined function
        if let Some(&offset) = self.func_offsets.get(name) {
            // Generate code for arguments (push onto stack)
            for arg_id in args {
                self.gen_expr_id(pool, *arg_id);
            }
            // Emit Call opcode with function offset
            self.code.push(LpsOpCode::Call(offset));
            return;
        }

        // Special case: perlin3(vec3) or perlin3(vec3, octaves)
        // Octaves is embedded in opcode, not pushed to stack
        if name == "perlin3" {
            // First arg is vec3, generate code to push its 3 components
            self.gen_expr_id(pool, args[0]);

            // Extract octaves from 2nd arg or use default
            let octaves = if args.len() >= 2 {
                let arg_expr = pool.expr(args[1]);
                match &arg_expr.kind {
                    ExprKind::Number(n) => *n as u8,
                    ExprKind::IntNumber(n) => *n as u8,
                    _ => 3,
                }
            } else {
                3
            };

            self.code.push(LpsOpCode::Perlin3(octaves));
            return;
        }

        // For all other functions, generate code for all arguments first
        for arg_id in args {
            self.gen_expr_id(pool, *arg_id);
        }

        // Emit the appropriate instruction
        self.gen_builtin_function_id(pool, name, args);
    }

    fn gen_builtin_function_id(&mut self, pool: &AstPool, name: &str, args: &[ExprId]) {
        match name {
            "sin" => self.code.push(LpsOpCode::SinFixed),
            "cos" => self.code.push(LpsOpCode::CosFixed),
            "frac" | "fract" => self.code.push(LpsOpCode::FractFixed),

            // Math functions - use explicit opcodes
            "min" => self.code.push(LpsOpCode::MinFixed),
            "max" => self.code.push(LpsOpCode::MaxFixed),
            "abs" => self.code.push(LpsOpCode::AbsFixed),
            "floor" => self.code.push(LpsOpCode::FloorFixed),
            "ceil" => self.code.push(LpsOpCode::CeilFixed),
            "sqrt" => self.code.push(LpsOpCode::SqrtFixed),
            "tan" => self.code.push(LpsOpCode::TanFixed),
            "pow" => self.code.push(LpsOpCode::PowFixed),
            "sign" => self.code.push(LpsOpCode::SignFixed),
            "mod" => self.code.push(LpsOpCode::ModFixed),
            "atan" => {
                if args.len() == 2 {
                    self.code.push(LpsOpCode::Atan2Fixed);
                } else {
                    self.code.push(LpsOpCode::AtanFixed);
                }
            }

            // Clamping/interpolation
            "clamp" => self.code.push(LpsOpCode::ClampFixed),
            "saturate" => self.code.push(LpsOpCode::SaturateFixed),
            "step" => self.code.push(LpsOpCode::StepFixed),
            "lerp" | "mix" => self.code.push(LpsOpCode::LerpFixed),
            "smoothstep" => self.code.push(LpsOpCode::SmoothstepFixed),

            // Vector functions - use typed opcodes based on argument type
            "length" => {
                if !args.is_empty() {
                    let arg_ty = pool.expr(args[0]).ty.as_ref().unwrap();
                    match arg_ty {
                        Type::Vec2 => self.code.push(LpsOpCode::Length2),
                        Type::Vec3 => self.code.push(LpsOpCode::Length3),
                        Type::Vec4 => self.code.push(LpsOpCode::Length4),
                        _ => {}
                    }
                }
            }
            "normalize" => {
                if !args.is_empty() {
                    let arg_ty = pool.expr(args[0]).ty.as_ref().unwrap();
                    match arg_ty {
                        Type::Vec2 => self.code.push(LpsOpCode::Normalize2),
                        Type::Vec3 => self.code.push(LpsOpCode::Normalize3),
                        Type::Vec4 => self.code.push(LpsOpCode::Normalize4),
                        _ => {}
                    }
                }
            }
            "dot" => {
                if !args.is_empty() {
                    let arg_ty = pool.expr(args[0]).ty.as_ref().unwrap();
                    match arg_ty {
                        Type::Vec2 => self.code.push(LpsOpCode::Dot2),
                        Type::Vec3 => self.code.push(LpsOpCode::Dot3),
                        Type::Vec4 => self.code.push(LpsOpCode::Dot4),
                        _ => {}
                    }
                }
            }
            "distance" => {
                if !args.is_empty() {
                    let arg_ty = pool.expr(args[0]).ty.as_ref().unwrap();
                    match arg_ty {
                        Type::Vec2 => self.code.push(LpsOpCode::Distance2),
                        Type::Vec3 => self.code.push(LpsOpCode::Distance3),
                        Type::Vec4 => self.code.push(LpsOpCode::Distance4),
                        _ => {}
                    }
                }
            }
            "cross" => {
                // Always vec3
                self.code.push(LpsOpCode::Cross3);
            }

            _ => {} // Unknown function - ignore
        }
    }
}
