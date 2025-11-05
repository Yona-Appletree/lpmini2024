// Suppress warnings for unused test helper functions
#![allow(dead_code)]

/// Helper functions for building expected statement AST in tests
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::ast::{Expr, ExprKind, Program, Stmt, StmtKind};
use crate::lpscript::error::{Span, Type};

// Re-export expression helpers for building statement contents
pub use crate::lpscript::compiler::test_ast::*;

/// Create a typed variable reference (for use in statements after type checking)
pub fn typed_var(name: &str, ty: Type) -> Expr {
    let mut expr = Expr::new(ExprKind::Variable(String::from(name)), Span::EMPTY);
    expr.ty = Some(ty);
    expr
}

/// Create a variable declaration statement
pub fn var_decl(ty: Type, name: &str, init: Option<Expr>) -> Stmt {
    Stmt::new(
        StmtKind::VarDecl {
            ty,
            name: String::from(name),
            init,
        },
        Span::EMPTY,
    )
}

/// Create an assignment statement
pub fn assign_stmt(name: &str, value: Expr) -> Stmt {
    Stmt::new(
        StmtKind::Assignment {
            name: String::from(name),
            value,
        },
        Span::EMPTY,
    )
}

/// Create a return statement
pub fn return_stmt(expr: Expr) -> Stmt {
    Stmt::new(StmtKind::Return(expr), Span::EMPTY)
}

/// Create an expression statement
pub fn expr_stmt(expr: Expr) -> Stmt {
    Stmt::new(StmtKind::Expr(expr), Span::EMPTY)
}

/// Create a block statement
pub fn block(stmts: Vec<Stmt>) -> Stmt {
    Stmt::new(StmtKind::Block(stmts), Span::EMPTY)
}

/// Create an if statement
pub fn if_stmt(condition: Expr, then_stmt: Stmt, else_stmt: Option<Stmt>) -> Stmt {
    Stmt::new(
        StmtKind::If {
            condition,
            then_stmt: Box::new(then_stmt),
            else_stmt: else_stmt.map(Box::new),
        },
        Span::EMPTY,
    )
}

/// Create a while statement
pub fn while_stmt(condition: Expr, body: Stmt) -> Stmt {
    Stmt::new(
        StmtKind::While {
            condition,
            body: Box::new(body),
        },
        Span::EMPTY,
    )
}

/// Create a for statement
pub fn for_stmt(
    init: Option<Stmt>,
    condition: Option<Expr>,
    increment: Option<Expr>,
    body: Stmt,
) -> Stmt {
    Stmt::new(
        StmtKind::For {
            init: init.map(Box::new),
            condition,
            increment,
            body: Box::new(body),
        },
        Span::EMPTY,
    )
}

/// Create a program with statements
pub fn program(stmts: Vec<Stmt>) -> Program {
    Program {
        functions: Vec::new(),
        stmts,
        span: Span::EMPTY,
    }
}

