/// Variable declaration code generation
extern crate alloc;
use alloc::format;
use alloc::string::ToString;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::error::{CodegenError, CodegenErrorKind};
use crate::shared::Type;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_var_decl(
        &mut self,
        ty: &Type,
        name: &str,
        init: Option<&Expr>,
    ) -> Result<(), CodegenError> {
        // Allocate a local for this variable
        // This will allocate in the same order as the analyzer did
        let local_idx = self.locals.allocate_typed(name.to_string(), ty.clone());

        if let Some(init_expr) = init {
            self.gen_expr(init_expr)?;
            // Use type-specific StoreLocal opcode
            self.code.push(match ty {
                Type::Dec32 | Type::Bool => LpsOpCode::StoreLocalDec32(local_idx),
                Type::Int32 => LpsOpCode::StoreLocalInt32(local_idx),
                Type::Vec2 => LpsOpCode::StoreLocalVec2(local_idx),
                Type::Vec3 => LpsOpCode::StoreLocalVec3(local_idx),
                Type::Vec4 => LpsOpCode::StoreLocalVec4(local_idx),
                Type::Mat3 => LpsOpCode::StoreLocalMat3(local_idx),
                other => {
                    return Err(CodegenError {
                        kind: CodegenErrorKind::UnsupportedFeature(format!(
                            "variable initialization for '{}' with type {} is not supported",
                            name, other
                        )),
                        span: init_expr.span,
                    });
                }
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use alloc::collections::BTreeMap;

    use super::*;
    use crate::compiler::ast::{Expr, ExprKind};
    use crate::compiler::codegen::LocalAllocator;
    use crate::compiler::error::CodegenErrorKind;
    use crate::shared::Span;

    fn literal_expr() -> Expr {
        let mut expr = Expr::new(ExprKind::Number(0.0), Span::new(0, 0));
        expr.ty = Some(Type::Dec32);
        expr
    }

    #[test]
    fn void_variable_initialization_returns_error() {
        let mut code = Vec::new();
        let mut locals = LocalAllocator::new();
        let func_offsets = BTreeMap::new();
        let mut gen = CodeGenerator::new(&mut code, &mut locals, &func_offsets);
        let expr = literal_expr();

        let result = gen.gen_var_decl(&Type::Void, "bad", Some(&expr));
        assert!(result.is_err(), "Expected Codegen error for void variable");
        if let Err(CodegenError {
            kind: CodegenErrorKind::UnsupportedFeature(msg),
            ..
        }) = result
        {
            assert!(
                msg.contains("bad"),
                "Message should mention variable name, got: {}",
                msg
            );
        } else {
            panic!("Expected UnsupportedFeature error");
        }
    }
}
