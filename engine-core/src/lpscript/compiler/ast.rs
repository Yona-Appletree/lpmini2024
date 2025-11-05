/// Abstract Syntax Tree for expressions and statements
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::shared::{Span, Type};

/// Index into the expression pool (Copy, no lifetimes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExprId(pub u32);

/// Index into the statement pool (Copy, no lifetimes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StmtId(pub u32);

/// Storage pool for all AST nodes
/// This avoids dynamic Box allocation and enables hard limits on AST size
#[derive(Debug)]
pub struct AstPool {
    pub exprs: Vec<Expr>,
    pub stmts: Vec<Stmt>,
    max_exprs: usize,
    max_stmts: usize,
}

impl AstPool {
    /// Create a new AST pool with default capacity
    pub fn new() -> Self {
        Self::with_capacity(10000, 5000)
    }

    /// Create a new AST pool with specified maximum capacities
    pub fn with_capacity(max_exprs: usize, max_stmts: usize) -> Self {
        AstPool {
            exprs: Vec::with_capacity(256),
            stmts: Vec::with_capacity(128),
            max_exprs,
            max_stmts,
        }
    }

    /// Allocate a new expression node, returns its index
    pub fn alloc_expr(&mut self, kind: ExprKind, span: Span) -> Result<ExprId, AstPoolError> {
        if self.exprs.len() >= self.max_exprs {
            return Err(AstPoolError::ExprLimitExceeded {
                max: self.max_exprs,
            });
        }
        let id = ExprId(self.exprs.len() as u32);
        self.exprs.push(Expr {
            kind,
            span,
            ty: None,
        });
        Ok(id)
    }

    /// Allocate a new statement node, returns its index
    pub fn alloc_stmt(&mut self, kind: StmtKind, span: Span) -> Result<StmtId, AstPoolError> {
        if self.stmts.len() >= self.max_stmts {
            return Err(AstPoolError::StmtLimitExceeded {
                max: self.max_stmts,
            });
        }
        let id = StmtId(self.stmts.len() as u32);
        self.stmts.push(Stmt { kind, span });
        Ok(id)
    }

    /// Get an expression by ID
    pub fn expr(&self, id: ExprId) -> &Expr {
        &self.exprs[id.0 as usize]
    }

    /// Get a mutable expression by ID
    pub fn expr_mut(&mut self, id: ExprId) -> &mut Expr {
        &mut self.exprs[id.0 as usize]
    }

    /// Get a statement by ID
    pub fn stmt(&self, id: StmtId) -> &Stmt {
        &self.stmts[id.0 as usize]
    }

    /// Get a mutable statement by ID
    pub fn stmt_mut(&mut self, id: StmtId) -> &mut Stmt {
        &mut self.stmts[id.0 as usize]
    }

    /// Get statistics about pool usage
    pub fn stats(&self) -> AstPoolStats {
        AstPoolStats {
            expr_count: self.exprs.len(),
            stmt_count: self.stmts.len(),
            max_exprs: self.max_exprs,
            max_stmts: self.max_stmts,
        }
    }
}

impl Default for AstPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about AST pool usage
#[derive(Debug, Clone, Copy)]
pub struct AstPoolStats {
    pub expr_count: usize,
    pub stmt_count: usize,
    pub max_exprs: usize,
    pub max_stmts: usize,
}

/// Errors that can occur during AST pool allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstPoolError {
    ExprLimitExceeded { max: usize },
    StmtLimitExceeded { max: usize },
}

impl core::fmt::Display for AstPoolError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            AstPoolError::ExprLimitExceeded { max } => {
                write!(f, "Expression node limit exceeded (max: {})", max)
            }
            AstPoolError::StmtLimitExceeded { max } => {
                write!(f, "Statement node limit exceeded (max: {})", max)
            }
        }
    }
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub ty: Type,
}

/// Function definition
#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<StmtId>,
    pub span: Span,
}

/// A complete program (for script mode)
#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<FunctionDef>,
    pub stmts: Vec<StmtId>,
    pub span: Span,
}

/// Statement with metadata
#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn new(kind: StmtKind, span: Span) -> Self {
        Stmt { kind, span }
    }
}

/// Statement kinds
#[derive(Debug, Clone)]
pub enum StmtKind {
    /// Variable declaration: `float x = expr;`
    VarDecl {
        ty: Type,
        name: String,
        init: Option<ExprId>,
    },

    /// Return statement: `return expr;`
    Return(ExprId),

    /// Expression statement: `expr;`
    Expr(ExprId),

    /// Block: `{ stmt1; stmt2; ... }`
    Block(Vec<StmtId>),

    /// If statement: `if (cond) then_stmt else else_stmt`
    If {
        condition: ExprId,
        then_stmt: StmtId,
        else_stmt: Option<StmtId>,
    },

    /// While loop: `while (cond) body`
    While {
        condition: ExprId,
        body: StmtId,
    },

    /// For loop: `for (init; condition; increment) body`
    For {
        init: Option<StmtId>,
        condition: Option<ExprId>,
        increment: Option<ExprId>,
        body: StmtId,
    },
}

/// Expression with metadata (span and optional type)
#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub ty: Option<Type>,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Expr {
            kind,
            span,
            ty: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_type(mut self, ty: Type) -> Self {
        self.ty = Some(ty);
        self
    }
}

/// Expression kinds
#[derive(Debug, Clone)]
pub enum ExprKind {
    // Literals
    Number(f32),
    IntNumber(i32),
    Variable(String),

    // Binary operations
    Add(ExprId, ExprId),
    Sub(ExprId, ExprId),
    Mul(ExprId, ExprId),
    Div(ExprId, ExprId),
    Mod(ExprId, ExprId),

    // Bitwise operations (Int32 only)
    BitwiseAnd(ExprId, ExprId),
    BitwiseOr(ExprId, ExprId),
    BitwiseXor(ExprId, ExprId),
    BitwiseNot(ExprId),
    LeftShift(ExprId, ExprId),
    RightShift(ExprId, ExprId),

    // Comparisons
    Less(ExprId, ExprId),
    Greater(ExprId, ExprId),
    LessEq(ExprId, ExprId),
    GreaterEq(ExprId, ExprId),
    Eq(ExprId, ExprId),
    NotEq(ExprId, ExprId),

    // Logical
    And(ExprId, ExprId),
    Or(ExprId, ExprId),
    Not(ExprId),

    // Unary
    Neg(ExprId),

    // Increment/Decrement (require l-values)
    PreIncrement(String),
    PreDecrement(String),
    PostIncrement(String),
    PostDecrement(String),

    // Ternary
    Ternary {
        condition: ExprId,
        true_expr: ExprId,
        false_expr: ExprId,
    },

    // Assignment expression (returns the assigned value)
    // In C/GLSL, assignments are expressions: x = y = 5
    Assign { target: String, value: ExprId },

    // Function call
    Call { name: String, args: Vec<ExprId> },

    // Vector constructors (GLSL-style: can take mixed vec/scalar args)
    Vec2Constructor(Vec<ExprId>),
    Vec3Constructor(Vec<ExprId>),
    Vec4Constructor(Vec<ExprId>),

    // Swizzle (component access/reordering)
    Swizzle {
        expr: ExprId,
        components: String, // e.g. "xy", "yx", "rgba", "x", etc.
    },
}
