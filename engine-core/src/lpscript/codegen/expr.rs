/// Expression code generation dispatcher
extern crate alloc;

use super::CodeGenerator;
use crate::lpscript::ast::{Expr, ExprKind};

impl<'a> CodeGenerator<'a> {
    pub(in crate::lpscript) fn gen_expr(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Number(n) => self.gen_number(*n),
            ExprKind::IntNumber(n) => self.gen_int_number(*n),
            ExprKind::Variable(name) => self.gen_variable(name, expr.ty.as_ref().unwrap()),
            
            ExprKind::Add(left, right) => self.gen_add(left, right, expr.ty.as_ref().unwrap()),
            ExprKind::Sub(left, right) => self.gen_sub(left, right, expr.ty.as_ref().unwrap()),
            ExprKind::Mul(left, right) => self.gen_mul(left, right, expr.ty.as_ref().unwrap()),
            ExprKind::Div(left, right) => self.gen_div(left, right, expr.ty.as_ref().unwrap()),
            ExprKind::Mod(left, right) => self.gen_mod(left, right, expr.ty.as_ref().unwrap()),
            ExprKind::Pow(left, right) => self.gen_pow(left, right),
            
            ExprKind::Less(left, right) => self.gen_less(left, right),
            ExprKind::Greater(left, right) => self.gen_greater(left, right),
            ExprKind::LessEq(left, right) => self.gen_less_eq(left, right),
            ExprKind::GreaterEq(left, right) => self.gen_greater_eq(left, right),
            ExprKind::Eq(left, right) => self.gen_eq(left, right),
            ExprKind::NotEq(left, right) => self.gen_not_eq(left, right),
            
            ExprKind::And(left, right) => self.gen_and(left, right),
            ExprKind::Or(left, right) => self.gen_or(left, right),
            ExprKind::Not(operand) => self.gen_not(operand),
            
            ExprKind::Neg(operand) => self.gen_neg(operand),
            
            ExprKind::Ternary { condition, true_expr, false_expr } => {
                self.gen_ternary(condition, true_expr, false_expr)
            }
            
            ExprKind::Assign { target, value } => self.gen_assign_expr(target, value),
            
            ExprKind::Call { name, args } => self.gen_function_call(name, args),
            
            ExprKind::Vec2Constructor(args) |
            ExprKind::Vec3Constructor(args) |
            ExprKind::Vec4Constructor(args) => self.gen_vec_constructor(args),
            
            ExprKind::Swizzle { expr, components } => self.gen_swizzle(expr, components),
        }
    }
}

