/// Expression code generation dispatcher
extern crate alloc;

use super::CodeGenerator;
use crate::compiler::ast::Expr;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    // Expression code generation - main dispatcher
    pub(crate) fn gen_expr(&mut self, expr: &Expr) {
        use crate::compiler::ast::ExprKind;

        let expr_ty = expr.ty.as_ref();

        match &expr.kind {
            ExprKind::Number(n) => self.gen_number(*n),
            ExprKind::IntNumber(n) => {
                self.gen_int_number(*n);
                // If Int32 was promoted to Fixed, emit conversion
                if expr_ty == Some(&crate::shared::Type::Fixed) {
                    self.code.push(LpsOpCode::Int32ToFixed);
                }
            }
            ExprKind::Variable(name) => {
                if let Some(ty) = expr_ty {
                    self.gen_variable(name, ty);
                }
            }

            ExprKind::Add(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_add(&**left, &**right, ty);
                }
            }
            ExprKind::Sub(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_sub(&**left, &**right, ty);
                }
            }
            ExprKind::Mul(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_mul(&**left, &**right, ty);
                }
            }
            ExprKind::Div(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_div(&**left, &**right, ty);
                }
            }
            ExprKind::Mod(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_mod(&**left, &**right, ty);
                }
            }

            ExprKind::BitwiseAnd(left, right) => self.gen_bitwise_and(&**left, &**right),
            ExprKind::BitwiseOr(left, right) => self.gen_bitwise_or(&**left, &**right),
            ExprKind::BitwiseXor(left, right) => self.gen_bitwise_xor(&**left, &**right),
            ExprKind::BitwiseNot(operand) => self.gen_bitwise_not(&**operand),
            ExprKind::LeftShift(left, right) => self.gen_left_shift(&**left, &**right),
            ExprKind::RightShift(left, right) => self.gen_right_shift(&**left, &**right),

            ExprKind::Less(left, right) => self.gen_less(&**left, &**right),
            ExprKind::Greater(left, right) => self.gen_greater(&**left, &**right),
            ExprKind::LessEq(left, right) => self.gen_less_eq(&**left, &**right),
            ExprKind::GreaterEq(left, right) => self.gen_greater_eq(&**left, &**right),
            ExprKind::Eq(left, right) => self.gen_eq(&**left, &**right),
            ExprKind::NotEq(left, right) => self.gen_not_eq(&**left, &**right),

            ExprKind::And(left, right) => self.gen_and(&**left, &**right),
            ExprKind::Or(left, right) => self.gen_or(&**left, &**right),
            ExprKind::Not(operand) => self.gen_not(&**operand),

            ExprKind::Neg(operand) => self.gen_neg(&**operand),

            ExprKind::PreIncrement(var_name) => {
                if let Some(ty) = expr_ty {
                    self.gen_pre_increment(var_name, ty);
                }
            }
            ExprKind::PreDecrement(var_name) => {
                if let Some(ty) = expr_ty {
                    self.gen_pre_decrement(var_name, ty);
                }
            }
            ExprKind::PostIncrement(var_name) => {
                if let Some(ty) = expr_ty {
                    self.gen_post_increment(var_name, ty);
                }
            }
            ExprKind::PostDecrement(var_name) => {
                if let Some(ty) = expr_ty {
                    self.gen_post_decrement(var_name, ty);
                }
            }

            ExprKind::Ternary {
                condition,
                true_expr,
                false_expr,
            } => self.gen_ternary(&**condition, &**true_expr, &**false_expr),

            ExprKind::Assign { target, value } => self.gen_assign_expr(target, &**value),

            ExprKind::Call { name, args } => self.gen_function_call(name, args),

            ExprKind::Vec2Constructor(args)
            | ExprKind::Vec3Constructor(args)
            | ExprKind::Vec4Constructor(args) => self.gen_vec_constructor(args),

            ExprKind::Swizzle { expr, components } => self.gen_swizzle(&**expr, components),
        }
    }
}
