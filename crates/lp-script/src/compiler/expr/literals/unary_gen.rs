/// Unary operation code generation
extern crate alloc;

use alloc::format;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::error::{CodegenError, CodegenErrorKind};
use crate::shared::Type;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_neg(&mut self, operand: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(operand)?;
        let operand_ty = operand.ty.as_ref().unwrap_or(&Type::Fixed);
        self.code.push(match operand_ty {
            Type::Fixed => LpsOpCode::NegFixed,
            Type::Int32 => LpsOpCode::NegInt32,
            Type::Vec2 => LpsOpCode::NegVec2,
            Type::Vec3 => LpsOpCode::NegVec3,
            Type::Vec4 => LpsOpCode::NegVec4,
            Type::Mat3 => LpsOpCode::NegMat3,
            other => {
                return Err(unsupported_unary("negation", "-", other, operand));
            }
        });
        Ok(())
    }
}

fn unsupported_unary(op_name: &str, symbol: &str, ty: &Type, operand: &Expr) -> CodegenError {
    CodegenError {
        kind: CodegenErrorKind::UnsupportedFeature(format!(
            "unary {} '{}' not supported for type {}",
            op_name, symbol, ty
        )),
        span: operand.span,
    }
}
