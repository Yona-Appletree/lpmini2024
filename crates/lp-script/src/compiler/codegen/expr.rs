/// Expression code generation dispatcher
extern crate alloc;

use super::CodeGenerator;
use crate::compiler::ast::Expr;
use crate::compiler::error::CodegenError;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    // Expression code generation - main dispatcher
    pub(crate) fn gen_expr(&mut self, expr: &Expr) -> Result<(), CodegenError> {
        use crate::compiler::ast::ExprKind;

        let expr_ty = expr.ty.as_ref();

        match &expr.kind {
            ExprKind::Number(n) => self.gen_number(*n),
            ExprKind::IntNumber(n) => {
                self.gen_int_number(*n);
                // If Int32 was promoted to Dec32, emit conversion
                if expr_ty == Some(&crate::shared::Type::Dec32) {
                    self.code.push(LpsOpCode::Int32ToDec32);
                }
            }
            ExprKind::Variable(name) => {
                if let Some(ty) = expr_ty {
                    self.gen_variable(name, ty)?;
                }
            }

            ExprKind::Add(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_add(left.as_ref(), right.as_ref(), ty)?;
                }
            }
            ExprKind::Sub(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_sub(left.as_ref(), right.as_ref(), ty)?;
                }
            }
            ExprKind::Mul(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_mul(left.as_ref(), right.as_ref(), ty)?;
                }
            }
            ExprKind::Div(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_div(left.as_ref(), right.as_ref(), ty)?;
                }
            }
            ExprKind::Mod(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_mod(left.as_ref(), right.as_ref(), ty)?;
                }
            }

            ExprKind::BitwiseAnd(left, right) => {
                self.gen_bitwise_and(left.as_ref(), right.as_ref())?
            }
            ExprKind::BitwiseOr(left, right) => {
                self.gen_bitwise_or(left.as_ref(), right.as_ref())?
            }
            ExprKind::BitwiseXor(left, right) => {
                self.gen_bitwise_xor(left.as_ref(), right.as_ref())?
            }
            ExprKind::BitwiseNot(operand) => self.gen_bitwise_not(operand.as_ref())?,
            ExprKind::LeftShift(left, right) => {
                self.gen_left_shift(left.as_ref(), right.as_ref())?
            }
            ExprKind::RightShift(left, right) => {
                self.gen_right_shift(left.as_ref(), right.as_ref())?
            }

            ExprKind::Less(left, right) => self.gen_less(left.as_ref(), right.as_ref())?,
            ExprKind::Greater(left, right) => self.gen_greater(left.as_ref(), right.as_ref())?,
            ExprKind::LessEq(left, right) => self.gen_less_eq(left.as_ref(), right.as_ref())?,
            ExprKind::GreaterEq(left, right) => {
                self.gen_greater_eq(left.as_ref(), right.as_ref())?
            }
            ExprKind::Eq(left, right) => self.gen_eq(left.as_ref(), right.as_ref())?,
            ExprKind::NotEq(left, right) => self.gen_not_eq(left.as_ref(), right.as_ref())?,

            ExprKind::And(left, right) => self.gen_and(left.as_ref(), right.as_ref())?,
            ExprKind::Or(left, right) => self.gen_or(left.as_ref(), right.as_ref())?,
            ExprKind::Not(operand) => self.gen_not(operand.as_ref())?,

            ExprKind::Neg(operand) => self.gen_neg(operand.as_ref())?,

            ExprKind::PreIncrement(var_name) => {
                if let Some(ty) = expr_ty {
                    self.gen_pre_increment(var_name, ty)?;
                }
            }
            ExprKind::PreDecrement(var_name) => {
                if let Some(ty) = expr_ty {
                    self.gen_pre_decrement(var_name, ty)?;
                }
            }
            ExprKind::PostIncrement(var_name) => {
                if let Some(ty) = expr_ty {
                    self.gen_post_increment(var_name, ty)?;
                }
            }
            ExprKind::PostDecrement(var_name) => {
                if let Some(ty) = expr_ty {
                    self.gen_post_decrement(var_name, ty)?;
                }
            }

            ExprKind::Ternary {
                condition,
                true_expr,
                false_expr,
            } => self.gen_ternary(condition.as_ref(), true_expr.as_ref(), false_expr.as_ref())?,

            ExprKind::Assign { target, value } => self.gen_assign_expr(target, value.as_ref())?,

            ExprKind::Call { name, args } => self.gen_function_call(name, args)?,

            ExprKind::Vec2Constructor(args)
            | ExprKind::Vec3Constructor(args)
            | ExprKind::Vec4Constructor(args)
            | ExprKind::Mat3Constructor(args) => self.gen_vec_constructor(args)?,

            ExprKind::Swizzle { expr, components } => {
                self.gen_swizzle(expr.as_ref(), components)?
            }
        }

        Ok(())
    }
}
