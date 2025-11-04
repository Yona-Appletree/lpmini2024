// Suppress warnings for unused test helper functions
#![allow(dead_code)]

/// Helper functions for building expected AST expressions in tests
/// 
/// These functions create AST nodes with dummy spans (Span::new(0, 0)) so tests
/// can focus on structure rather than source locations.
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;

// ============================================================================
// Leaf nodes (return Box<Expr>)
// ============================================================================

/// Create an integer literal expression
pub fn int32(value: i32) -> Box<Expr> {
    Box::new(Expr::new(ExprKind::IntNumber(value), Span::new(0, 0)))
}

/// Create a float literal expression
pub fn num(value: f32) -> Box<Expr> {
    Box::new(Expr::new(ExprKind::Number(value), Span::new(0, 0)))
}

/// Create a variable reference expression
pub fn var(name: &str) -> Box<Expr> {
    Box::new(Expr::new(ExprKind::Variable(name.to_string()), Span::new(0, 0)))
}

// ============================================================================
// Binary operators (return ExprKind)
// ============================================================================

/// Less than: left < right
pub fn less(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::Less(left, right)
}

/// Greater than: left > right
pub fn greater(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::Greater(left, right)
}

/// Less than or equal: left <= right
pub fn less_eq(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::LessEq(left, right)
}

/// Greater than or equal: left >= right
pub fn greater_eq(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::GreaterEq(left, right)
}

/// Equal: left == right
pub fn eq(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::Eq(left, right)
}

/// Not equal: left != right
pub fn not_eq(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::NotEq(left, right)
}

/// Add: left + right
pub fn add(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::Add(left, right)
}

/// Subtract: left - right
pub fn sub(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::Sub(left, right)
}

/// Multiply: left * right
pub fn mul(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::Mul(left, right)
}

/// Divide: left / right
pub fn div(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::Div(left, right)
}

/// Modulo: left % right
pub fn modulo(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::Mod(left, right)
}

/// Power: left ^ right
pub fn pow(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::Pow(left, right)
}

/// Logical AND: left && right
pub fn and(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::And(left, right)
}

/// Logical OR: left || right
pub fn or(left: Box<Expr>, right: Box<Expr>) -> ExprKind {
    ExprKind::Or(left, right)
}

// ============================================================================
// Other expressions
// ============================================================================

/// Ternary: condition ? true_expr : false_expr
pub fn ternary(condition: Box<Expr>, true_expr: Box<Expr>, false_expr: Box<Expr>) -> ExprKind {
    ExprKind::Ternary {
        condition,
        true_expr,
        false_expr,
    }
}

/// Swizzle: expr.components
pub fn swizzle(expr: Box<Expr>, components: &str) -> ExprKind {
    ExprKind::Swizzle {
        expr,
        components: components.to_string(),
    }
}

/// Assignment: target = value
pub fn assign(target: &str, value: Box<Expr>) -> ExprKind {
    ExprKind::Assign {
        target: target.to_string(),
        value,
    }
}

