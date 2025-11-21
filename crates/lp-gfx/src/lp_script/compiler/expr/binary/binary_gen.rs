/// Binary operation code generation
extern crate alloc;

use alloc::format;

use crate::lp_script::compiler::ast::Expr;
use crate::lp_script::compiler::codegen::CodeGenerator;
use crate::lp_script::compiler::error::{CodegenError, CodegenErrorKind};
use crate::lp_script::shared::{Span, Type};
use crate::lp_script::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_add(
        &mut self,
        left: &Expr,
        right: &Expr,
        ty: &Type,
    ) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        self.code.push(match ty {
            Type::Dec32 => LpsOpCode::AddDec32,
            Type::Int32 => LpsOpCode::AddInt32,
            Type::Vec2 => LpsOpCode::AddVec2,
            Type::Vec3 => LpsOpCode::AddVec3,
            Type::Vec4 => LpsOpCode::AddVec4,
            Type::Mat3 => LpsOpCode::AddMat3,
            other => {
                return Err(unsupported_binary_result(
                    "addition", "+", other, left, right,
                ));
            }
        });
        Ok(())
    }

    pub(crate) fn gen_sub(
        &mut self,
        left: &Expr,
        right: &Expr,
        ty: &Type,
    ) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        self.code.push(match ty {
            Type::Dec32 => LpsOpCode::SubDec32,
            Type::Int32 => LpsOpCode::SubInt32,
            Type::Vec2 => LpsOpCode::SubVec2,
            Type::Vec3 => LpsOpCode::SubVec3,
            Type::Vec4 => LpsOpCode::SubVec4,
            Type::Mat3 => LpsOpCode::SubMat3,
            other => {
                return Err(unsupported_binary_result(
                    "subtraction",
                    "-",
                    other,
                    left,
                    right,
                ));
            }
        });
        Ok(())
    }

    pub(crate) fn gen_mul(
        &mut self,
        left: &Expr,
        right: &Expr,
        ty: &Type,
    ) -> Result<(), CodegenError> {
        let left_ty = left.ty.as_ref().unwrap();
        let right_ty = right.ty.as_ref().unwrap();

        // For scalar-vector/matrix operations, generate in reverse order to get correct stack layout
        let is_scalar_vector = matches!(
            (left_ty, right_ty),
            (
                Type::Dec32 | Type::Int32,
                Type::Vec2 | Type::Vec3 | Type::Vec4 | Type::Mat3
            )
        );

        if is_scalar_vector {
            // Generate: scalar * vector -> [vec_components..., scalar]
            self.gen_expr(right)?; // Vector first
            self.gen_expr(left)?; // Scalar on top
                                  // Convert Int32 scalar to Dec32 if needed
            if matches!(left_ty, Type::Int32) {
                self.code.push(LpsOpCode::Int32ToDec32);
            }
        } else {
            // Normal order
            self.gen_expr(left)?;
            self.gen_expr(right)?;
        }

        // Emit appropriate opcode
        let opcode = match (left_ty, right_ty, ty) {
            // Scalar operations
            (Type::Dec32, Type::Dec32, Type::Dec32) => LpsOpCode::MulDec32,
            (Type::Int32, Type::Int32, Type::Int32) => LpsOpCode::MulInt32,

            // Vector-Vector operations
            (Type::Vec2, Type::Vec2, Type::Vec2) => LpsOpCode::MulVec2,
            (Type::Vec3, Type::Vec3, Type::Vec3) => LpsOpCode::MulVec3,
            (Type::Vec4, Type::Vec4, Type::Vec4) => LpsOpCode::MulVec4,

            // Matrix-Matrix operations (matrix multiplication)
            (Type::Mat3, Type::Mat3, Type::Mat3) => LpsOpCode::MulMat3,

            // Vector-Scalar operations
            (Type::Vec2, Type::Dec32 | Type::Int32, Type::Vec2) => {
                // Convert Int32 to Dec32 if needed
                if matches!(right_ty, Type::Int32) {
                    self.code.push(LpsOpCode::Int32ToDec32);
                }
                LpsOpCode::MulVec2Scalar
            }
            (Type::Vec3, Type::Dec32 | Type::Int32, Type::Vec3) => {
                if matches!(right_ty, Type::Int32) {
                    self.code.push(LpsOpCode::Int32ToDec32);
                }
                LpsOpCode::MulVec3Scalar
            }
            (Type::Vec4, Type::Dec32 | Type::Int32, Type::Vec4) => {
                if matches!(right_ty, Type::Int32) {
                    self.code.push(LpsOpCode::Int32ToDec32);
                }
                LpsOpCode::MulVec4Scalar
            }

            // Matrix-Scalar operations
            (Type::Mat3, Type::Dec32 | Type::Int32, Type::Mat3) => {
                if matches!(right_ty, Type::Int32) {
                    self.code.push(LpsOpCode::Int32ToDec32);
                }
                LpsOpCode::MulMat3Scalar
            }

            // Scalar-Vector operations (already generated in correct order, conversion already done above)
            (Type::Dec32 | Type::Int32, Type::Vec2, Type::Vec2) => LpsOpCode::MulVec2Scalar,
            (Type::Dec32 | Type::Int32, Type::Vec3, Type::Vec3) => LpsOpCode::MulVec3Scalar,
            (Type::Dec32 | Type::Int32, Type::Vec4, Type::Vec4) => LpsOpCode::MulVec4Scalar,

            // Scalar-Matrix operations (already generated in correct order, conversion already done above)
            (Type::Dec32 | Type::Int32, Type::Mat3, Type::Mat3) => LpsOpCode::MulMat3Scalar,

            _ => {
                return Err(unsupported_binary_operands(
                    "multiplication",
                    "*",
                    left_ty,
                    right_ty,
                    ty,
                    left,
                    right,
                ))
            }
        };

        self.code.push(opcode);
        Ok(())
    }

    pub(crate) fn gen_div(
        &mut self,
        left: &Expr,
        right: &Expr,
        ty: &Type,
    ) -> Result<(), CodegenError> {
        let left_ty = left.ty.as_ref().unwrap();
        let right_ty = right.ty.as_ref().unwrap();

        self.gen_expr(left)?;
        self.gen_expr(right)?;

        // Emit appropriate opcode
        let opcode = match (left_ty, right_ty, ty) {
            // Scalar operations
            (Type::Dec32, Type::Dec32, Type::Dec32) => LpsOpCode::DivDec32,
            (Type::Int32, Type::Int32, Type::Int32) => LpsOpCode::DivInt32,

            // Vector-Vector operations
            (Type::Vec2, Type::Vec2, Type::Vec2) => LpsOpCode::DivVec2,
            (Type::Vec3, Type::Vec3, Type::Vec3) => LpsOpCode::DivVec3,
            (Type::Vec4, Type::Vec4, Type::Vec4) => LpsOpCode::DivVec4,

            // Vector-Scalar operations (vec / scalar)
            (Type::Vec2, Type::Dec32 | Type::Int32, Type::Vec2) => {
                if matches!(right_ty, Type::Int32) {
                    self.code.push(LpsOpCode::Int32ToDec32);
                }
                LpsOpCode::DivVec2Scalar
            }
            (Type::Vec3, Type::Dec32 | Type::Int32, Type::Vec3) => {
                if matches!(right_ty, Type::Int32) {
                    self.code.push(LpsOpCode::Int32ToDec32);
                }
                LpsOpCode::DivVec3Scalar
            }
            (Type::Vec4, Type::Dec32 | Type::Int32, Type::Vec4) => {
                if matches!(right_ty, Type::Int32) {
                    self.code.push(LpsOpCode::Int32ToDec32);
                }
                LpsOpCode::DivVec4Scalar
            }

            // Matrix-Scalar operations (mat / scalar)
            (Type::Mat3, Type::Dec32 | Type::Int32, Type::Mat3) => {
                if matches!(right_ty, Type::Int32) {
                    self.code.push(LpsOpCode::Int32ToDec32);
                }
                LpsOpCode::DivMat3Scalar
            }

            _ => {
                return Err(unsupported_binary_operands(
                    "division", "/", left_ty, right_ty, ty, left, right,
                ))
            }
        };

        self.code.push(opcode);
        Ok(())
    }

    pub(crate) fn gen_mod(
        &mut self,
        left: &Expr,
        right: &Expr,
        ty: &Type,
    ) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        self.code.push(match ty {
            Type::Dec32 => LpsOpCode::ModDec32,
            Type::Int32 => LpsOpCode::ModInt32,
            Type::Vec2 => LpsOpCode::ModVec2,
            Type::Vec3 => LpsOpCode::ModVec3,
            Type::Vec4 => LpsOpCode::ModVec4,
            // Note: Mat3 modulo not supported (no ModMat3 opcode exists)
            other => {
                return Err(unsupported_binary_result("modulo", "%", other, left, right));
            }
        });
        Ok(())
    }
}

fn binary_span(left: &Expr, right: &Expr) -> Span {
    let start = left.span.start.min(right.span.start);
    let end = left.span.end.max(right.span.end);
    Span::new(start, end)
}

fn unsupported_binary_result(
    op_name: &str,
    symbol: &str,
    result_ty: &Type,
    left: &Expr,
    right: &Expr,
) -> CodegenError {
    CodegenError {
        kind: CodegenErrorKind::UnsupportedFeature(format!(
            "{} '{}' not supported for type {}",
            op_name, symbol, result_ty
        )),
        span: binary_span(left, right),
    }
}

fn unsupported_binary_operands(
    op_name: &str,
    symbol: &str,
    left_ty: &Type,
    right_ty: &Type,
    result_ty: &Type,
    left: &Expr,
    right: &Expr,
) -> CodegenError {
    CodegenError {
        kind: CodegenErrorKind::UnsupportedFeature(format!(
            "{} '{}' not supported for operand types {} and {} (result type {})",
            op_name, symbol, left_ty, right_ty, result_ty
        )),
        span: binary_span(left, right),
    }
}
