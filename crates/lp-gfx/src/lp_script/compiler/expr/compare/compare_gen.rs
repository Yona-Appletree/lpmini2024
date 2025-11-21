/// Comparison operation code generation
extern crate alloc;

use crate::lp_script::compiler::ast::Expr;
use crate::lp_script::compiler::codegen::CodeGenerator;
use crate::lp_script::compiler::error::CodegenError;
use crate::lp_script::shared::Type;
use crate::lp_script::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    fn get_comparison_opcode(&self, op: ComparisonOp, ty: &Type) -> LpsOpCode {
        match (op, ty) {
            (ComparisonOp::Less, Type::Int32) => LpsOpCode::LessInt32,
            (ComparisonOp::Greater, Type::Int32) => LpsOpCode::GreaterInt32,
            (ComparisonOp::LessEq, Type::Int32) => LpsOpCode::LessEqInt32,
            (ComparisonOp::GreaterEq, Type::Int32) => LpsOpCode::GreaterEqInt32,
            (ComparisonOp::Eq, Type::Int32) => LpsOpCode::EqInt32,
            (ComparisonOp::NotEq, Type::Int32) => LpsOpCode::NotEqInt32,
            (ComparisonOp::Less, _) => LpsOpCode::LessDec32,
            (ComparisonOp::Greater, _) => LpsOpCode::GreaterDec32,
            (ComparisonOp::LessEq, _) => LpsOpCode::LessEqDec32,
            (ComparisonOp::GreaterEq, _) => LpsOpCode::GreaterEqDec32,
            (ComparisonOp::Eq, _) => LpsOpCode::EqDec32,
            (ComparisonOp::NotEq, _) => LpsOpCode::NotEqDec32,
        }
    }

    pub(crate) fn gen_less(&mut self, left: &Expr, right: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        // Determine type from left operand (both should be same type after type checking)
        let ty = left.ty.as_ref().unwrap_or(&Type::Dec32);
        self.code
            .push(self.get_comparison_opcode(ComparisonOp::Less, ty));
        Ok(())
    }

    pub(crate) fn gen_greater(&mut self, left: &Expr, right: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        let ty = left.ty.as_ref().unwrap_or(&Type::Dec32);
        self.code
            .push(self.get_comparison_opcode(ComparisonOp::Greater, ty));
        Ok(())
    }

    pub(crate) fn gen_less_eq(&mut self, left: &Expr, right: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        let ty = left.ty.as_ref().unwrap_or(&Type::Dec32);
        self.code
            .push(self.get_comparison_opcode(ComparisonOp::LessEq, ty));
        Ok(())
    }

    pub(crate) fn gen_greater_eq(&mut self, left: &Expr, right: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        let ty = left.ty.as_ref().unwrap_or(&Type::Dec32);
        self.code
            .push(self.get_comparison_opcode(ComparisonOp::GreaterEq, ty));
        Ok(())
    }

    pub(crate) fn gen_eq(&mut self, left: &Expr, right: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        let ty = left.ty.as_ref().unwrap_or(&Type::Dec32);
        self.code
            .push(self.get_comparison_opcode(ComparisonOp::Eq, ty));
        Ok(())
    }

    pub(crate) fn gen_not_eq(&mut self, left: &Expr, right: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        let ty = left.ty.as_ref().unwrap_or(&Type::Dec32);
        self.code
            .push(self.get_comparison_opcode(ComparisonOp::NotEq, ty));
        Ok(())
    }
}

enum ComparisonOp {
    Less,
    Greater,
    LessEq,
    GreaterEq,
    Eq,
    NotEq,
}
