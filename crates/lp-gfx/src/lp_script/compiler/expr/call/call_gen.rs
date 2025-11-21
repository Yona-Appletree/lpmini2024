/// Function call code generation
extern crate alloc;

use crate::lp_script::compiler::ast::{Expr, ExprKind};
use crate::lp_script::compiler::codegen::CodeGenerator;
use crate::lp_script::compiler::error::CodegenError;
use crate::lp_script::shared::Type;
use crate::lp_script::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_function_call(
        &mut self,
        name: &str,
        args: &[Expr],
    ) -> Result<(), CodegenError> {
        // Check if it's a user-defined function
        if let Some(&offset) = self.func_offsets.get(name) {
            // Generate code for arguments (push onto stack)
            for arg in args {
                self.gen_expr(arg)?;
            }
            // Emit Call opcode with function offset
            self.code.push(LpsOpCode::Call(offset));
            return Ok(());
        }

        // Special case: perlin3(vec3) or perlin3(vec3, octaves)
        // Octaves is embedded in opcode, not pushed to stack
        if name == "perlin3" {
            // First arg is vec3, generate code to push its 3 components
            self.gen_expr(&args[0])?;

            // Extract octaves from 2nd arg or use default
            let octaves = if args.len() >= 2 {
                match &args[1].kind {
                    ExprKind::Number(n) => *n as u8,
                    ExprKind::IntNumber(n) => *n as u8,
                    _ => 3,
                }
            } else {
                3
            };

            self.code.push(LpsOpCode::Perlin3(octaves));
            return Ok(());
        }

        // For all other functions, generate code for all arguments first
        for arg in args {
            self.gen_expr(arg)?;
        }

        // Emit the appropriate instruction
        self.gen_builtin_function(name, args)
    }

    fn gen_builtin_function(&mut self, name: &str, args: &[Expr]) -> Result<(), CodegenError> {
        match name {
            "sin" => self.code.push(LpsOpCode::SinDec32),
            "cos" => self.code.push(LpsOpCode::CosDec32),
            "frac" | "fract" => self.code.push(LpsOpCode::FractDec32),

            // Math functions - use explicit opcodes
            "min" => self.code.push(LpsOpCode::MinDec32),
            "max" => self.code.push(LpsOpCode::MaxDec32),
            "abs" => self.code.push(LpsOpCode::AbsDec32),
            "floor" => self.code.push(LpsOpCode::FloorDec32),
            "ceil" => self.code.push(LpsOpCode::CeilDec32),
            "sqrt" => self.code.push(LpsOpCode::SqrtDec32),
            "tan" => self.code.push(LpsOpCode::TanDec32),
            "pow" => self.code.push(LpsOpCode::PowDec32),
            "sign" => self.code.push(LpsOpCode::SignDec32),
            "mod" => self.code.push(LpsOpCode::ModDec32),
            "atan" => {
                if args.len() == 2 {
                    self.code.push(LpsOpCode::Atan2Dec32);
                } else {
                    self.code.push(LpsOpCode::AtanDec32);
                }
            }

            // Clamping/interpolation
            "clamp" => self.code.push(LpsOpCode::ClampDec32),
            "saturate" => self.code.push(LpsOpCode::SaturateDec32),
            "step" => self.code.push(LpsOpCode::StepDec32),
            "lerp" | "mix" => self.code.push(LpsOpCode::LerpDec32),
            "smoothstep" => self.code.push(LpsOpCode::SmoothstepDec32),

            // Vector functions - use typed opcodes based on argument type
            "length" => {
                if !args.is_empty() {
                    let arg_ty = args[0].ty.as_ref().unwrap();
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
                    let arg_ty = args[0].ty.as_ref().unwrap();
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
                    let arg_ty = args[0].ty.as_ref().unwrap();
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
                    let arg_ty = args[0].ty.as_ref().unwrap();
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

            // Matrix functions
            "transpose" => {
                if !args.is_empty() {
                    let arg_ty = args[0].ty.as_ref().unwrap();
                    if arg_ty == &Type::Mat3 {
                        self.code.push(LpsOpCode::TransposeMat3);
                    }
                }
            }
            "determinant" => {
                if !args.is_empty() {
                    let arg_ty = args[0].ty.as_ref().unwrap();
                    if arg_ty == &Type::Mat3 {
                        self.code.push(LpsOpCode::DeterminantMat3);
                    }
                }
            }
            "inverse" => {
                if !args.is_empty() {
                    let arg_ty = args[0].ty.as_ref().unwrap();
                    if arg_ty == &Type::Mat3 {
                        self.code.push(LpsOpCode::InverseMat3);
                    }
                }
            }

            _ => {} // Unknown function - ignore
        }
        Ok(())
    }
}
