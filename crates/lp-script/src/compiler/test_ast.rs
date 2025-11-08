#![cfg(test)]
#![allow(dead_code)]

/// Helper functions for building expected AST expressions in tests using the
/// new recursive `LpBox`-backed AST.
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::compiler::ast::{Expr, ExprKind, Stmt, StmtKind};
use crate::shared::{Span, Type};
use lp_pool::LpBox;

/// Zero-sized builder that provides ergonomic helpers for constructing AST
/// nodes. Each method returns a fully-formed `Expr`/`Stmt` with spans set to
/// `Span::EMPTY` and, where appropriate, inferred types pre-populated.
#[derive(Default, Debug, Clone, Copy)]
pub struct AstBuilder;

impl AstBuilder {
    pub fn new() -> Self {
        AstBuilder
    }

    // ---------------------------------------------------------------------
    // Leaf expressions
    // ---------------------------------------------------------------------
    pub fn int32(&mut self, value: i32) -> Expr {
        self.expr_with_type(ExprKind::IntNumber(value), Some(Type::Int32))
    }

    pub fn num(&mut self, value: f32) -> Expr {
        self.expr_with_type(ExprKind::Number(value), Some(Type::Fixed))
    }

    pub fn var(&mut self, name: &str) -> Expr {
        self.expr_with_type(ExprKind::Variable(String::from(name)), None)
    }

    // ---------------------------------------------------------------------
    // Arithmetic helpers
    // ---------------------------------------------------------------------
    pub fn add(&mut self, left: Expr, right: Expr, ty: Type) -> Expr {
        self.binary(|l, r| ExprKind::Add(l, r), left, right, Some(ty))
    }

    pub fn sub(&mut self, left: Expr, right: Expr, ty: Type) -> Expr {
        self.binary(|l, r| ExprKind::Sub(l, r), left, right, Some(ty))
    }

    pub fn mul(&mut self, left: Expr, right: Expr, ty: Type) -> Expr {
        self.binary(|l, r| ExprKind::Mul(l, r), left, right, Some(ty))
    }

    pub fn div(&mut self, left: Expr, right: Expr, ty: Type) -> Expr {
        self.binary(|l, r| ExprKind::Div(l, r), left, right, Some(ty))
    }

    pub fn modulo(&mut self, left: Expr, right: Expr, ty: Type) -> Expr {
        self.binary(|l, r| ExprKind::Mod(l, r), left, right, Some(ty))
    }

    // ---------------------------------------------------------------------
    // Bitwise helpers
    // ---------------------------------------------------------------------
    pub fn bitwise_and(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(
            |l, r| ExprKind::BitwiseAnd(l, r),
            left,
            right,
            Some(Type::Int32),
        )
    }

    pub fn bitwise_or(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(
            |l, r| ExprKind::BitwiseOr(l, r),
            left,
            right,
            Some(Type::Int32),
        )
    }

    pub fn bitwise_xor(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(
            |l, r| ExprKind::BitwiseXor(l, r),
            left,
            right,
            Some(Type::Int32),
        )
    }

    pub fn bitwise_not(&mut self, value: Expr) -> Expr {
        self.unary(ExprKind::BitwiseNot, value, Some(Type::Int32))
    }

    pub fn left_shift(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(
            |l, r| ExprKind::LeftShift(l, r),
            left,
            right,
            Some(Type::Int32),
        )
    }

    pub fn right_shift(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(
            |l, r| ExprKind::RightShift(l, r),
            left,
            right,
            Some(Type::Int32),
        )
    }

    // ---------------------------------------------------------------------
    // Comparison helpers (return Bool)
    // ---------------------------------------------------------------------
    pub fn less(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(|l, r| ExprKind::Less(l, r), left, right, Some(Type::Bool))
    }

    pub fn greater(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(
            |l, r| ExprKind::Greater(l, r),
            left,
            right,
            Some(Type::Bool),
        )
    }

    pub fn less_eq(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(|l, r| ExprKind::LessEq(l, r), left, right, Some(Type::Bool))
    }

    pub fn greater_eq(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(
            |l, r| ExprKind::GreaterEq(l, r),
            left,
            right,
            Some(Type::Bool),
        )
    }

    pub fn eq(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(|l, r| ExprKind::Eq(l, r), left, right, Some(Type::Bool))
    }

    pub fn not_eq(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(|l, r| ExprKind::NotEq(l, r), left, right, Some(Type::Bool))
    }

    // ---------------------------------------------------------------------
    // Logical helpers
    // ---------------------------------------------------------------------
    pub fn and(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(|l, r| ExprKind::And(l, r), left, right, Some(Type::Bool))
    }

    pub fn or(&mut self, left: Expr, right: Expr) -> Expr {
        self.binary(|l, r| ExprKind::Or(l, r), left, right, Some(Type::Bool))
    }

    pub fn not(&mut self, value: Expr) -> Expr {
        self.unary(ExprKind::Not, value, Some(Type::Bool))
    }

    pub fn logical_and(&mut self, left: Expr, right: Expr) -> Expr {
        self.and(left, right)
    }

    pub fn logical_or(&mut self, left: Expr, right: Expr) -> Expr {
        self.or(left, right)
    }

    // ---------------------------------------------------------------------
    // Unary helpers
    // ---------------------------------------------------------------------
    pub fn neg(&mut self, value: Expr, ty: Type) -> Expr {
        self.unary(ExprKind::Neg, value, Some(ty))
    }

    // ---------------------------------------------------------------------
    // Assignment helpers
    // ---------------------------------------------------------------------
    pub fn assign(&mut self, target: &str, value: Expr, ty: Type) -> Expr {
        let mut expr = Expr::new(
            ExprKind::Assign {
                target: String::from(target),
                value: self.box_expr(value),
            },
            Span::EMPTY,
        );
        expr.ty = Some(ty);
        expr
    }

    // ---------------------------------------------------------------------
    // Function call helpers
    // ---------------------------------------------------------------------
    pub fn call(&mut self, name: &str, args: Vec<Expr>, ty: Type) -> Expr {
        self.call_with_type(name, args, Some(ty))
    }

    pub fn call_untyped(&mut self, name: &str, args: Vec<Expr>) -> Expr {
        self.call_with_type(name, args, None)
    }

    fn call_with_type(&mut self, name: &str, args: Vec<Expr>, ty: Option<Type>) -> Expr {
        let mut expr = Expr::new(
            ExprKind::Call {
                name: String::from(name),
                args,
            },
            Span::EMPTY,
        );
        expr.ty = ty;
        expr
    }

    // ---------------------------------------------------------------------
    // Vector constructors
    // ---------------------------------------------------------------------
    pub fn vec2(&mut self, components: Vec<Expr>) -> Expr {
        self.expr_with_type(ExprKind::Vec2Constructor(components), Some(Type::Vec2))
    }

    pub fn vec3(&mut self, components: Vec<Expr>) -> Expr {
        self.expr_with_type(ExprKind::Vec3Constructor(components), Some(Type::Vec3))
    }

    pub fn vec4(&mut self, components: Vec<Expr>) -> Expr {
        self.expr_with_type(ExprKind::Vec4Constructor(components), Some(Type::Vec4))
    }

    // ---------------------------------------------------------------------
    // Swizzle helper
    // ---------------------------------------------------------------------
    pub fn swizzle(&mut self, expr: Expr, components: &str, ty: Option<Type>) -> Expr {
        let mut result = Expr::new(
            ExprKind::Swizzle {
                expr: self.box_expr(expr),
                components: components.into(),
            },
            Span::EMPTY,
        );
        result.ty = ty;
        result
    }

    pub fn ternary(&mut self, condition: Expr, then_expr: Expr, else_expr: Expr, ty: Type) -> Expr {
        let mut expr = Expr::new(
            ExprKind::Ternary {
                condition: self.box_expr(condition),
                true_expr: self.box_expr(then_expr),
                false_expr: self.box_expr(else_expr),
            },
            Span::EMPTY,
        );
        expr.ty = Some(ty);
        expr
    }

    // ---------------------------------------------------------------------
    // Statement helpers (subset used in tests)
    // ---------------------------------------------------------------------
    pub fn stmt_expr(&mut self, expr: Expr) -> Stmt {
        Stmt::new(StmtKind::Expr(expr), Span::EMPTY)
    }

    pub fn stmt_return(&mut self, expr: Expr) -> Stmt {
        Stmt::new(StmtKind::Return(expr), Span::EMPTY)
    }

    pub fn stmt_var(&mut self, name: &str, ty: Type, init: Option<Expr>) -> Stmt {
        Stmt::new(
            StmtKind::VarDecl {
                ty,
                name: String::from(name),
                init,
            },
            Span::EMPTY,
        )
    }

    pub fn stmt_block(&mut self, stmts: Vec<Stmt>) -> Stmt {
        Stmt::new(StmtKind::Block(stmts), Span::EMPTY)
    }

    // ---------------------------------------------------------------------
    // Internal helpers
    // ---------------------------------------------------------------------
    fn binary<F>(&mut self, make_kind: F, left: Expr, right: Expr, ty: Option<Type>) -> Expr
    where
        F: FnOnce(LpBox<Expr>, LpBox<Expr>) -> ExprKind,
    {
        let kind = make_kind(self.box_expr(left), self.box_expr(right));
        self.expr_with_type(kind, ty)
    }

    fn unary<F>(&mut self, make_kind: F, value: Expr, ty: Option<Type>) -> Expr
    where
        F: FnOnce(LpBox<Expr>) -> ExprKind,
    {
        let kind = make_kind(self.box_expr(value));
        self.expr_with_type(kind, ty)
    }

    fn expr_with_type(&mut self, kind: ExprKind, ty: Option<Type>) -> Expr {
        let mut expr = Expr::new(kind, Span::EMPTY);
        expr.ty = ty;
        expr
    }

    fn box_expr(&mut self, expr: Expr) -> LpBox<Expr> {
        LpBox::try_new(expr).expect("LpBox allocation failed in AstBuilder")
    }
}
