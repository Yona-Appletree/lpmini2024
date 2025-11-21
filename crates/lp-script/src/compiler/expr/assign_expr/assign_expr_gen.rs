/// Assignment expression code generation
extern crate alloc;

use alloc::format;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::error::{CodegenError, CodegenErrorKind};
use crate::shared::Type;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_assign_expr(
        &mut self,
        target: &str,
        value: &Expr,
    ) -> Result<(), CodegenError> {
        self.gen_expr(value)?;

        if let Some(local_idx) = self.locals.get(target) {
            let var_type = self.locals.get_type(local_idx).unwrap_or(&Type::Dec32);

            // Duplicate value based on type (assignment returns the assigned value)
            let dup_opcode = match var_type {
                Type::Dec32 | Type::Bool | Type::Int32 => LpsOpCode::Dup1,
                Type::Vec2 => LpsOpCode::Dup2,
                Type::Vec3 => LpsOpCode::Dup3,
                Type::Vec4 => LpsOpCode::Dup4,
                Type::Mat3 => LpsOpCode::Dup9,
                other => return Err(unsupported_assignment(target, other, value)),
            };
            self.code.push(dup_opcode);

            // Store using type-specific opcode
            let store_opcode = match var_type {
                Type::Dec32 | Type::Bool => LpsOpCode::StoreLocalDec32(local_idx),
                Type::Int32 => LpsOpCode::StoreLocalInt32(local_idx),
                Type::Vec2 => LpsOpCode::StoreLocalVec2(local_idx),
                Type::Vec3 => LpsOpCode::StoreLocalVec3(local_idx),
                Type::Vec4 => LpsOpCode::StoreLocalVec4(local_idx),
                Type::Mat3 => LpsOpCode::StoreLocalMat3(local_idx),
                other => return Err(unsupported_assignment(target, other, value)),
            };
            self.code.push(store_opcode);
        }
        Ok(())
    }
}

fn unsupported_assignment(target: &str, ty: &Type, value: &Expr) -> CodegenError {
    CodegenError {
        kind: CodegenErrorKind::UnsupportedFeature(format!(
            "assignment to variable '{}' with type {} is not supported",
            target, ty
        )),
        span: value.span,
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

    fn fixed_literal(value: f32) -> Expr {
        let mut expr = Expr::new(ExprKind::Number(value), Span::new(0, 0));
        expr.ty = Some(Type::Dec32);
        expr
    }

    fn mat3_literal() -> Expr {
        let mut components = Vec::new();
        for _ in 0..9 {
            components.push(fixed_literal(1.0));
        }
        let mut expr = Expr::new(ExprKind::Mat3Constructor(components), Span::new(0, 0));
        expr.ty = Some(Type::Mat3);
        expr
    }

    #[test]
    fn assignment_to_void_returns_error() {
        let mut code = Vec::new();
        let mut locals = LocalAllocator::new();
        let func_offsets = BTreeMap::new();
        locals.allocate_typed("bad".into(), Type::Void);
        let mut gen = CodeGenerator::new(&mut code, &mut locals, &func_offsets);
        let value = fixed_literal(0.0);

        let result = gen.gen_assign_expr("bad", &value);
        assert!(
            result.is_err(),
            "Expected Codegen error for void assignment"
        );
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

    #[test]
    fn assignment_to_mat3_uses_dup9_and_store_mat3() {
        let mut code = Vec::new();
        let mut locals = LocalAllocator::new();
        let func_offsets = BTreeMap::new();
        let local_idx = locals.allocate_typed("mat".into(), Type::Mat3);
        let mut gen = CodeGenerator::new(&mut code, &mut locals, &func_offsets);
        let value = mat3_literal();

        gen.gen_assign_expr("mat", &value)
            .expect("Mat3 assignment should succeed");

        assert!(
            code.iter().any(|op| matches!(op, LpsOpCode::Dup9)),
            "Expected Dup9 in opcode stream"
        );
        assert!(
            code.iter()
                .any(|op| matches!(op, LpsOpCode::StoreLocalMat3(idx) if *idx == local_idx)),
            "Expected StoreLocalMat3 in opcode stream"
        );
    }
}
