/// Unary operation code generation
extern crate alloc;

use alloc::format;

use crate::lp_script::compiler::ast::Expr;
use crate::lp_script::compiler::codegen::CodeGenerator;
use crate::lp_script::compiler::error::{CodegenError, CodegenErrorKind};
use crate::lp_script::shared::Type;
use crate::lp_script::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_neg(&mut self, operand: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(operand)?;
        let operand_ty = operand.ty.as_ref().unwrap_or(&Type::Dec32);
        self.code.push(match operand_ty {
            Type::Dec32 => LpsOpCode::NegDec32,
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
