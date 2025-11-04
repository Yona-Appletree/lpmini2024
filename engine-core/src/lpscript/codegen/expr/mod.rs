/// Expression code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;

use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::local_allocator::LocalAllocator;

mod literals;
mod variable;
mod binary;
mod comparison;
mod logical;
mod ternary;
mod assign_expr;
mod call;
mod constructors;
mod swizzle;

pub use literals::{gen_number, gen_int_number};
pub use variable::gen_variable;
pub use binary::{gen_add, gen_sub, gen_mul, gen_div, gen_mod, gen_pow};
pub use comparison::{gen_less, gen_greater, gen_less_eq, gen_greater_eq, gen_eq, gen_not_eq};
pub use logical::{gen_and, gen_or};
pub use ternary::gen_ternary;
pub use assign_expr::gen_assign_expr;
pub use call::gen_function_call;
pub use constructors::gen_vec_constructor;
pub use swizzle::gen_swizzle;

/// Generate code for an expression
pub fn gen_expr(
    expr: &Expr,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
) {
    match &expr.kind {
        ExprKind::Number(n) => {
            gen_number(*n, code);
        }
        
        ExprKind::IntNumber(n) => {
            gen_int_number(*n, code);
        }
        
        ExprKind::Variable(name) => {
            gen_variable(name, code, locals);
        }
        
        // Binary operations - use type information to generate typed opcodes
        ExprKind::Add(left, right) => {
            gen_add(left, right, expr.ty.as_ref().unwrap(), code, locals, func_offsets, gen_expr);
        }
        
        ExprKind::Sub(left, right) => {
            gen_sub(left, right, expr.ty.as_ref().unwrap(), code, locals, func_offsets, gen_expr);
        }
        
        ExprKind::Mul(left, right) => {
            gen_mul(left, right, expr.ty.as_ref().unwrap(), code, locals, func_offsets, gen_expr);
        }
        
        ExprKind::Div(left, right) => {
            gen_div(left, right, expr.ty.as_ref().unwrap(), code, locals, func_offsets, gen_expr);
        }
        
        ExprKind::Mod(left, right) => {
            gen_mod(left, right, expr.ty.as_ref().unwrap(), code, locals, func_offsets, gen_expr);
        }
        
        ExprKind::Pow(left, right) => {
            gen_pow(left, right, code, locals, func_offsets, gen_expr);
        }
        
        // Comparisons
        ExprKind::Less(left, right) => {
            gen_less(left, right, code, locals, func_offsets, gen_expr);
        }
        
        ExprKind::Greater(left, right) => {
            gen_greater(left, right, code, locals, func_offsets, gen_expr);
        }
        
        ExprKind::LessEq(left, right) => {
            gen_less_eq(left, right, code, locals, func_offsets, gen_expr);
        }
        
        ExprKind::GreaterEq(left, right) => {
            gen_greater_eq(left, right, code, locals, func_offsets, gen_expr);
        }
        
        ExprKind::Eq(left, right) => {
            gen_eq(left, right, code, locals, func_offsets, gen_expr);
        }
        
        ExprKind::NotEq(left, right) => {
            gen_not_eq(left, right, code, locals, func_offsets, gen_expr);
        }
        
        // Logical operations
        ExprKind::And(left, right) => {
            gen_and(left, right, code, locals, func_offsets, gen_expr);
        }
        
        ExprKind::Or(left, right) => {
            gen_or(left, right, code, locals, func_offsets, gen_expr);
        }
        
        // Ternary
        ExprKind::Ternary { condition, true_expr, false_expr } => {
            gen_ternary(condition, true_expr, false_expr, code, locals, func_offsets, gen_expr);
        }
        
        // Assignment expression
        ExprKind::Assign { target, value } => {
            gen_assign_expr(target, value, code, locals, func_offsets, gen_expr);
        }
        
        // Function calls
        ExprKind::Call { name, args } => {
            gen_function_call(name, args, code, locals, func_offsets, gen_expr);
        }
        
        // Vector constructors - push all components from all arguments
        // Supports GLSL-style mixed args: vec3(vec2, float), vec4(vec3, float), etc.
        ExprKind::Vec2Constructor(args) | 
        ExprKind::Vec3Constructor(args) | 
        ExprKind::Vec4Constructor(args) => {
            gen_vec_constructor(args, code, locals, func_offsets, gen_expr);
        }
        
        ExprKind::Swizzle { expr: base_expr, components } => {
            gen_swizzle(base_expr, components, code, locals, func_offsets, gen_expr);
        }
    }
}

