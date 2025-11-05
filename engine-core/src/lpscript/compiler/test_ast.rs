// Suppress warnings for unused test helper functions
#![allow(dead_code)]

/// Helper functions for building expected AST expressions in tests
///
/// These functions create AST nodes with dummy spans (Span::EMPTY) so tests
/// can focus on structure rather than source locations.
/// Types are included to validate type inference.
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{Expr, ExprKind};
use crate::lpscript::shared::{Span, Type};

// ============================================================================
// Leaf nodes (return Expr with auto-typed)
// ============================================================================

/// Create an integer literal expression
pub fn int32(value: i32) -> Expr {
    let mut expr = Expr::new(ExprKind::IntNumber(value), Span::EMPTY);
    expr.ty = Some(Type::Int32);
    expr
}

/// Create a float literal expression
pub fn num(value: f32) -> Expr {
    let mut expr = Expr::new(ExprKind::Number(value), Span::EMPTY);
    expr.ty = Some(Type::Fixed);
    expr
}

/// Create a variable reference expression (type must be inferred by context)
pub fn var(name: &str) -> Expr {
    Expr::new(ExprKind::Variable(String::from(name)), Span::EMPTY)
}

// ============================================================================
// Comparison operators (return Expr, auto-typed to Bool)
// ============================================================================

/// Less than: left < right
pub fn less(left: Expr, right: Expr) -> Expr {
    let mut expr = Expr::new(ExprKind::Less(Box::new(left), Box::new(right)), Span::EMPTY);
    expr.ty = Some(Type::Bool);
    expr
}

/// Greater than: left > right
pub fn greater(left: Expr, right: Expr) -> Expr {
    let mut expr = Expr::new(
        ExprKind::Greater(Box::new(left), Box::new(right)),
        Span::EMPTY,
    );
    expr.ty = Some(Type::Bool);
    expr
}

/// Less than or equal: left <= right
pub fn less_eq(left: Expr, right: Expr) -> Expr {
    let mut expr = Expr::new(
        ExprKind::LessEq(Box::new(left), Box::new(right)),
        Span::EMPTY,
    );
    expr.ty = Some(Type::Bool);
    expr
}

/// Greater than or equal: left >= right
pub fn greater_eq(left: Expr, right: Expr) -> Expr {
    let mut expr = Expr::new(
        ExprKind::GreaterEq(Box::new(left), Box::new(right)),
        Span::EMPTY,
    );
    expr.ty = Some(Type::Bool);
    expr
}

/// Equal: left == right
pub fn eq(left: Expr, right: Expr) -> Expr {
    let mut expr = Expr::new(ExprKind::Eq(Box::new(left), Box::new(right)), Span::EMPTY);
    expr.ty = Some(Type::Bool);
    expr
}

/// Not equal: left != right
pub fn not_eq(left: Expr, right: Expr) -> Expr {
    let mut expr = Expr::new(
        ExprKind::NotEq(Box::new(left), Box::new(right)),
        Span::EMPTY,
    );
    expr.ty = Some(Type::Bool);
    expr
}

// ============================================================================
// Arithmetic operators (return Expr, type parameter required)
// ============================================================================

/// Add: left + right
pub fn add(left: Expr, right: Expr, ty: Type) -> Expr {
    let mut expr = Expr::new(ExprKind::Add(Box::new(left), Box::new(right)), Span::EMPTY);
    expr.ty = Some(ty);
    expr
}

/// Subtract: left - right
pub fn sub(left: Expr, right: Expr, ty: Type) -> Expr {
    let mut expr = Expr::new(ExprKind::Sub(Box::new(left), Box::new(right)), Span::EMPTY);
    expr.ty = Some(ty);
    expr
}

/// Multiply: left * right
pub fn mul(left: Expr, right: Expr, ty: Type) -> Expr {
    let mut expr = Expr::new(ExprKind::Mul(Box::new(left), Box::new(right)), Span::EMPTY);
    expr.ty = Some(ty);
    expr
}

/// Divide: left / right
pub fn div(left: Expr, right: Expr, ty: Type) -> Expr {
    let mut expr = Expr::new(ExprKind::Div(Box::new(left), Box::new(right)), Span::EMPTY);
    expr.ty = Some(ty);
    expr
}

/// Modulo: left % right
pub fn modulo(left: Expr, right: Expr, ty: Type) -> Expr {
    let mut expr = Expr::new(ExprKind::Mod(Box::new(left), Box::new(right)), Span::EMPTY);
    expr.ty = Some(ty);
    expr
}

/// Power: left ^ right
pub fn pow(left: Expr, right: Expr, ty: Type) -> Expr {
    let mut expr = Expr::new(ExprKind::Pow(Box::new(left), Box::new(right)), Span::EMPTY);
    expr.ty = Some(ty);
    expr
}

// ============================================================================
// Logical operators (return Expr, type parameter required)
// ============================================================================

/// Logical AND: left && right
pub fn and(left: Expr, right: Expr, ty: Type) -> Expr {
    let mut expr = Expr::new(ExprKind::And(Box::new(left), Box::new(right)), Span::EMPTY);
    expr.ty = Some(ty);
    expr
}

/// Logical OR: left || right
pub fn or(left: Expr, right: Expr, ty: Type) -> Expr {
    let mut expr = Expr::new(ExprKind::Or(Box::new(left), Box::new(right)), Span::EMPTY);
    expr.ty = Some(ty);
    expr
}

// ============================================================================
// Other expressions (return Expr, type parameter required)
// ============================================================================

/// Ternary: condition ? true_expr : false_expr
pub fn ternary(condition: Expr, true_expr: Expr, false_expr: Expr, ty: Type) -> Expr {
    let mut expr = Expr::new(
        ExprKind::Ternary {
            condition: Box::new(condition),
            true_expr: Box::new(true_expr),
            false_expr: Box::new(false_expr),
        },
        Span::EMPTY,
    );
    expr.ty = Some(ty);
    expr
}

/// Swizzle: expr.components
pub fn swizzle(base_expr: Expr, components: &str, ty: Type) -> Expr {
    let mut expr = Expr::new(
        ExprKind::Swizzle {
            expr: Box::new(base_expr),
            components: String::from(components),
        },
        Span::EMPTY,
    );
    expr.ty = Some(ty);
    expr
}

/// Assignment: target = value
pub fn assign(target: &str, value: Expr, ty: Type) -> Expr {
    let mut expr = Expr::new(
        ExprKind::Assign {
            target: String::from(target),
            value: Box::new(value),
        },
        Span::EMPTY,
    );
    expr.ty = Some(ty);
    expr
}

/// Function call: name(args)
pub fn call(name: &str, args: Vec<Expr>, ty: Type) -> Expr {
    let mut expr = Expr::new(
        ExprKind::Call {
            name: String::from(name),
            args,
        },
        Span::EMPTY,
    );
    expr.ty = Some(ty);
    expr
}

/// Vec2 constructor: vec2(args)
pub fn vec2_ctor(args: Vec<Expr>, ty: Type) -> Expr {
    let mut expr = Expr::new(ExprKind::Vec2Constructor(args), Span::EMPTY);
    expr.ty = Some(ty);
    expr
}

/// Vec3 constructor: vec3(args)
pub fn vec3_ctor(args: Vec<Expr>, ty: Type) -> Expr {
    let mut expr = Expr::new(ExprKind::Vec3Constructor(args), Span::EMPTY);
    expr.ty = Some(ty);
    expr
}

/// Vec4 constructor: vec4(args)
pub fn vec4_ctor(args: Vec<Expr>, ty: Type) -> Expr {
    let mut expr = Expr::new(ExprKind::Vec4Constructor(args), Span::EMPTY);
    expr.ty = Some(ty);
    expr
}
