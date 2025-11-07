/// Expression code generation dispatcher
extern crate alloc;

use super::CodeGenerator;
use crate::compiler::ast::{AstPool, Expr, ExprId};
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    // Expression code generation by ID - main dispatcher
    pub(in crate) fn gen_expr_id(&mut self, pool: &AstPool, expr_id: ExprId) {
        use crate::compiler::ast::ExprKind;

        let expr = pool.expr(expr_id);
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
                    self.gen_add_id(pool, *left, *right, ty);
                }
            }
            ExprKind::Sub(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_sub_id(pool, *left, *right, ty);
                }
            }
            ExprKind::Mul(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_mul_id(pool, *left, *right, ty);
                }
            }
            ExprKind::Div(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_div_id(pool, *left, *right, ty);
                }
            }
            ExprKind::Mod(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_mod_id(pool, *left, *right, ty);
                }
            }

            ExprKind::BitwiseAnd(left, right) => self.gen_bitwise_and_id(pool, *left, *right),
            ExprKind::BitwiseOr(left, right) => self.gen_bitwise_or_id(pool, *left, *right),
            ExprKind::BitwiseXor(left, right) => self.gen_bitwise_xor_id(pool, *left, *right),
            ExprKind::BitwiseNot(operand) => self.gen_bitwise_not_id(pool, *operand),
            ExprKind::LeftShift(left, right) => self.gen_left_shift_id(pool, *left, *right),
            ExprKind::RightShift(left, right) => self.gen_right_shift_id(pool, *left, *right),

            ExprKind::Less(left, right) => self.gen_less_id(pool, *left, *right),
            ExprKind::Greater(left, right) => self.gen_greater_id(pool, *left, *right),
            ExprKind::LessEq(left, right) => self.gen_less_eq_id(pool, *left, *right),
            ExprKind::GreaterEq(left, right) => self.gen_greater_eq_id(pool, *left, *right),
            ExprKind::Eq(left, right) => self.gen_eq_id(pool, *left, *right),
            ExprKind::NotEq(left, right) => self.gen_not_eq_id(pool, *left, *right),

            ExprKind::And(left, right) => self.gen_and_id(pool, *left, *right),
            ExprKind::Or(left, right) => self.gen_or_id(pool, *left, *right),
            ExprKind::Not(operand) => self.gen_not_id(pool, *operand),

            ExprKind::Neg(operand) => self.gen_neg_id(pool, *operand),

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
            } => self.gen_ternary_id(pool, *condition, *true_expr, *false_expr),

            ExprKind::Assign { target, value } => self.gen_assign_expr_id(pool, target, *value),

            ExprKind::Call { name, args } => self.gen_function_call_id(pool, name, args),

            ExprKind::Vec2Constructor(args)
            | ExprKind::Vec3Constructor(args)
            | ExprKind::Vec4Constructor(args) => self.gen_vec_constructor_id(pool, args),

            ExprKind::Swizzle { expr, components } => self.gen_swizzle_id(pool, *expr, components),
        }
    }

    // Old gen_expr method kept for compatibility with individual *_gen.rs files
    // TODO: Once all *_gen.rs files are updated to pool-based API, this can be removed
    #[allow(dead_code)]
    pub(in crate) fn gen_expr(&mut self, _expr: &Expr) {
        // Stub - not used in pool-based code path
        // Individual *_gen.rs files still reference this but it's not called
    }
}
