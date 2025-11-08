#![allow(dead_code)]

/// Helper functions for building expected statement AST in tests using the
/// recursive `LpBox` AST.
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use lp_pool::LpBox;

use crate::compiler::ast::{Expr, ExprKind, Program, Stmt, StmtKind};
use crate::compiler::test_ast::AstBuilder as ExprBuilder;
use crate::shared::{Span, Type};

/// Builder for creating statement/program ASTs for tests.
#[derive(Default, Debug, Clone, Copy)]
pub struct StmtBuilder {
    expr_builder: ExprBuilder,
}

impl StmtBuilder {
    pub fn new() -> Self {
        StmtBuilder {
            expr_builder: ExprBuilder::new(),
        }
    }

    // ------------------------------------------------------------------
    // Expression helpers (delegate to expression builder)
    // ------------------------------------------------------------------
    pub fn int32(&mut self, value: i32) -> Expr {
        self.expr_builder.int32(value)
    }

    pub fn num(&mut self, value: f32) -> Expr {
        self.expr_builder.num(value)
    }

    pub fn var(&mut self, name: &str) -> Expr {
        self.expr_builder.var(name)
    }

    pub fn typed_var(&mut self, name: &str, ty: Type) -> Expr {
        let mut expr = self.expr_builder.var(name);
        expr.ty = Some(ty);
        expr
    }

    pub fn add(&mut self, left: Expr, right: Expr, ty: Type) -> Expr {
        self.expr_builder.add(left, right, ty)
    }

    pub fn sub(&mut self, left: Expr, right: Expr, ty: Type) -> Expr {
        self.expr_builder.sub(left, right, ty)
    }

    pub fn mul(&mut self, left: Expr, right: Expr, ty: Type) -> Expr {
        self.expr_builder.mul(left, right, ty)
    }

    pub fn assign(&mut self, target: &str, value: Expr, ty: Type) -> Expr {
        self.expr_builder.assign(target, value, ty)
    }

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

    pub fn bitwise_and(&mut self, left: Expr, right: Expr) -> Expr {
        self.expr_builder.bitwise_and(left, right)
    }

    pub fn bitwise_or(&mut self, left: Expr, right: Expr) -> Expr {
        self.expr_builder.bitwise_or(left, right)
    }

    pub fn bitwise_xor(&mut self, left: Expr, right: Expr) -> Expr {
        self.expr_builder.bitwise_xor(left, right)
    }

    pub fn bitwise_not(&mut self, value: Expr) -> Expr {
        self.expr_builder.bitwise_not(value)
    }

    // ------------------------------------------------------------------
    // Statement helpers
    // ------------------------------------------------------------------
    pub fn stmt_expr(&mut self, expr: Expr) -> Stmt {
        Stmt::new(StmtKind::Expr(expr), Span::EMPTY)
    }

    pub fn expr_stmt(&mut self, expr: Expr) -> Stmt {
        self.stmt_expr(expr)
    }

    pub fn return_stmt(&mut self, expr: Expr) -> Stmt {
        Stmt::new(StmtKind::Return(expr), Span::EMPTY)
    }

    pub fn var_decl(&mut self, ty: Type, name: &str, init: Option<Expr>) -> Stmt {
        Stmt::new(
            StmtKind::VarDecl {
                ty,
                name: String::from(name),
                init,
            },
            Span::EMPTY,
        )
    }

    pub fn block(&mut self, stmts: Vec<Stmt>) -> Stmt {
        Stmt::new(StmtKind::Block(stmts), Span::EMPTY)
    }

    pub fn if_stmt(&mut self, condition: Expr, then_stmt: Stmt, else_stmt: Option<Stmt>) -> Stmt {
        Stmt::new(
            StmtKind::If {
                condition,
                then_stmt: self.box_stmt(then_stmt),
                else_stmt: else_stmt.map(|s| self.box_stmt(s)),
            },
            Span::EMPTY,
        )
    }

    pub fn while_stmt(&mut self, condition: Expr, body: Stmt) -> Stmt {
        Stmt::new(
            StmtKind::While {
                condition,
                body: self.box_stmt(body),
            },
            Span::EMPTY,
        )
    }

    pub fn for_stmt(
        &mut self,
        init: Option<Stmt>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Stmt,
    ) -> Stmt {
        Stmt::new(
            StmtKind::For {
                init: init.map(|s| self.box_stmt(s)),
                condition,
                increment,
                body: self.box_stmt(body),
            },
            Span::EMPTY,
        )
    }

    pub fn program(&mut self, stmts: Vec<Stmt>) -> Program {
        Program {
            functions: Vec::new(),
            stmts,
            span: Span::EMPTY,
        }
    }

    fn box_stmt(&mut self, stmt: Stmt) -> LpBox<Stmt> {
        LpBox::try_new(stmt).expect("LpBox allocation failed in StmtBuilder")
    }
}
