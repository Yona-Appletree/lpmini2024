/// Expression code generation
extern crate alloc;

use crate::lpscript::ast::{Expr, ExprKind};
use super::CodeGenerator;

mod literals;
mod variable;
mod binary;
mod logical;
mod ternary;
mod assign_expr;
mod call;
mod constructors;
mod swizzle;

// Comparison codegen is now in compiler module
use crate::lpscript::compiler::expr::compare;

impl<'a> CodeGenerator<'a> {
    /// Generate code for an expression
    pub(crate) fn gen_expr(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Number(n) => {
                self.gen_number(*n);
            }
            
            ExprKind::IntNumber(n) => {
                self.gen_int_number(*n);
            }
            
            ExprKind::Variable(name) => {
                self.gen_variable(name);
            }
            
            // Binary operations - use type information to generate typed opcodes
            ExprKind::Add(left, right) => {
                self.gen_add(left, right, expr.ty.as_ref().unwrap());
            }
            
            ExprKind::Sub(left, right) => {
                self.gen_sub(left, right, expr.ty.as_ref().unwrap());
            }
            
            ExprKind::Mul(left, right) => {
                self.gen_mul(left, right, expr.ty.as_ref().unwrap());
            }
            
            ExprKind::Div(left, right) => {
                self.gen_div(left, right, expr.ty.as_ref().unwrap());
            }
            
            ExprKind::Mod(left, right) => {
                self.gen_mod(left, right, expr.ty.as_ref().unwrap());
            }
            
            ExprKind::Pow(left, right) => {
                self.gen_pow(left, right);
            }
            
            // Comparisons
            ExprKind::Less(left, right) => {
                self.gen_less(left, right);
            }
            
            ExprKind::Greater(left, right) => {
                self.gen_greater(left, right);
            }
            
            ExprKind::LessEq(left, right) => {
                self.gen_less_eq(left, right);
            }
            
            ExprKind::GreaterEq(left, right) => {
                self.gen_greater_eq(left, right);
            }
            
            ExprKind::Eq(left, right) => {
                self.gen_eq(left, right);
            }
            
            ExprKind::NotEq(left, right) => {
                self.gen_not_eq(left, right);
            }
            
            // Logical operations
            ExprKind::And(left, right) => {
                self.gen_and(left, right);
            }
            
            ExprKind::Or(left, right) => {
                self.gen_or(left, right);
            }
            
            // Ternary
            ExprKind::Ternary { condition, true_expr, false_expr } => {
                self.gen_ternary(condition, true_expr, false_expr);
            }
            
            // Assignment expression
            ExprKind::Assign { target, value } => {
                self.gen_assign_expr(target, value);
            }
            
            // Function calls
            ExprKind::Call { name, args } => {
                self.gen_function_call(name, args);
            }
            
            // Vector constructors - push all components from all arguments
            // Supports GLSL-style mixed args: vec3(vec2, float), vec4(vec3, float), etc.
            ExprKind::Vec2Constructor(args) | 
            ExprKind::Vec3Constructor(args) | 
            ExprKind::Vec4Constructor(args) => {
                self.gen_vec_constructor(args);
            }
            
            ExprKind::Swizzle { expr: base_expr, components } => {
                self.gen_swizzle(base_expr, components);
            }
        }
    }
}

