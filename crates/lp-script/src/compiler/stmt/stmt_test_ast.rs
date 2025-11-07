// Suppress warnings for unused test helper functions
#![allow(dead_code)]

/// Helper functions for building expected statement AST in tests using AstPool
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::compiler::ast::{AstPool, ExprId, ExprKind, Program, StmtId, StmtKind};
use crate::shared::{Span, Type};

/// Builder for creating test statement ASTs with a pool
pub struct StmtBuilder {
    pool: AstPool,
}

impl StmtBuilder {
    /// Create a new builder with an empty pool
    pub fn new() -> Self {
        Self {
            pool: AstPool::new(),
        }
    }

    /// Consume the builder and return the pool
    pub fn into_pool(self) -> AstPool {
        self.pool
    }

    /// Get a reference to the pool
    pub fn pool(&self) -> &AstPool {
        &self.pool
    }

    /// Get a mutable reference to the pool
    pub fn pool_mut(&mut self) -> &mut AstPool {
        &mut self.pool
    }

    // ============================================================================
    // Expression helpers (delegate to pool)
    // ============================================================================

    /// Create an integer literal expression
    pub fn int32(&mut self, value: i32) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::IntNumber(value), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Int32);
        id
    }

    /// Create a float literal expression
    pub fn num(&mut self, value: f32) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Number(value), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Fixed);
        id
    }

    /// Create a typed variable reference
    pub fn typed_var(&mut self, name: &str, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Variable(String::from(name)), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Addition: left + right
    pub fn add(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Add(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Subtraction: left - right
    pub fn sub(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Sub(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Multiplication: left * right
    pub fn mul(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Mul(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Assignment: target = value
    pub fn assign(&mut self, target: &str, value: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(
                ExprKind::Assign {
                    target: String::from(target),
                    value,
                },
                Span::EMPTY,
            )
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Function call: name(args...)
    pub fn call(&mut self, name: &str, args: Vec<ExprId>, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(
                ExprKind::Call {
                    name: String::from(name),
                    args,
                },
                Span::EMPTY,
            )
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    // ============================================================================
    // Statement builders
    // ============================================================================

    /// Create a variable declaration statement
    pub fn var_decl(&mut self, ty: Type, name: &str, init: Option<ExprId>) -> StmtId {
        self.pool
            .alloc_stmt(
                StmtKind::VarDecl {
                    ty,
                    name: String::from(name),
                    init,
                },
                Span::EMPTY,
            )
            .unwrap()
    }

    /// Create a return statement
    pub fn return_stmt(&mut self, expr: ExprId) -> StmtId {
        self.pool
            .alloc_stmt(StmtKind::Return(expr), Span::EMPTY)
            .unwrap()
    }

    /// Create an expression statement
    pub fn expr_stmt(&mut self, expr: ExprId) -> StmtId {
        self.pool
            .alloc_stmt(StmtKind::Expr(expr), Span::EMPTY)
            .unwrap()
    }

    /// Create a block statement
    pub fn block(&mut self, stmts: Vec<StmtId>) -> StmtId {
        self.pool
            .alloc_stmt(StmtKind::Block(stmts), Span::EMPTY)
            .unwrap()
    }

    /// Create an if statement
    pub fn if_stmt(
        &mut self,
        condition: ExprId,
        then_stmt: StmtId,
        else_stmt: Option<StmtId>,
    ) -> StmtId {
        self.pool
            .alloc_stmt(
                StmtKind::If {
                    condition,
                    then_stmt,
                    else_stmt,
                },
                Span::EMPTY,
            )
            .unwrap()
    }

    /// Create a while statement
    pub fn while_stmt(&mut self, condition: ExprId, body: StmtId) -> StmtId {
        self.pool
            .alloc_stmt(StmtKind::While { condition, body }, Span::EMPTY)
            .unwrap()
    }

    /// Create a for statement
    pub fn for_stmt(
        &mut self,
        init: Option<StmtId>,
        condition: Option<ExprId>,
        increment: Option<ExprId>,
        body: StmtId,
    ) -> StmtId {
        self.pool
            .alloc_stmt(
                StmtKind::For {
                    init,
                    condition,
                    increment,
                    body,
                },
                Span::EMPTY,
            )
            .unwrap()
    }

    /// Create a program with statements (returns a reference to internal pool)
    pub fn program(&mut self, stmts: Vec<StmtId>) -> Program {
        Program {
            functions: Vec::new(),
            stmts,
            span: Span::EMPTY,
        }
    }
}

impl StmtBuilder {
    /// Build a program and return it with the pool (consumes the builder)
    pub fn build_program(self, stmts: Vec<StmtId>) -> (Program, AstPool) {
        let program = Program {
            functions: Vec::new(),
            stmts,
            span: Span::EMPTY,
        };
        (program, self.pool)
    }
}

impl Default for StmtBuilder {
    fn default() -> Self {
        Self::new()
    }
}
