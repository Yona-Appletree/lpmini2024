/// Abstract Syntax Tree for expressions and statements
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use lp_pool::LpBox;

use crate::shared::{Span, Type};

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
    pub body: Vec<Stmt>,
    pub span: Span,
}

/// A complete program (for script mode)
#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<FunctionDef>,
    pub stmts: Vec<Stmt>,
    #[allow(dead_code)] // Metadata field - may be used for error reporting
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
        init: Option<Expr>,
    },

    /// Return statement: `return expr;`
    Return(Expr),

    /// Expression statement: `expr;`
    Expr(Expr),

    /// Block: `{ stmt1; stmt2; ... }`
    Block(Vec<Stmt>),

    /// If statement: `if (cond) then_stmt else else_stmt`
    If {
        condition: Expr,
        then_stmt: LpBox<Stmt>,
        else_stmt: Option<LpBox<Stmt>>,
    },

    /// While loop: `while (cond) body`
    While { condition: Expr, body: LpBox<Stmt> },

    /// For loop: `for (init; condition; increment) body`
    For {
        init: Option<LpBox<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: LpBox<Stmt>,
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

    #[cfg(test)]
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
    Add(LpBox<Expr>, LpBox<Expr>),
    Sub(LpBox<Expr>, LpBox<Expr>),
    Mul(LpBox<Expr>, LpBox<Expr>),
    Div(LpBox<Expr>, LpBox<Expr>),
    Mod(LpBox<Expr>, LpBox<Expr>),

    // Bitwise operations (Int32 only)
    BitwiseAnd(LpBox<Expr>, LpBox<Expr>),
    BitwiseOr(LpBox<Expr>, LpBox<Expr>),
    BitwiseXor(LpBox<Expr>, LpBox<Expr>),
    BitwiseNot(LpBox<Expr>),
    LeftShift(LpBox<Expr>, LpBox<Expr>),
    RightShift(LpBox<Expr>, LpBox<Expr>),

    // Comparisons
    Less(LpBox<Expr>, LpBox<Expr>),
    Greater(LpBox<Expr>, LpBox<Expr>),
    LessEq(LpBox<Expr>, LpBox<Expr>),
    GreaterEq(LpBox<Expr>, LpBox<Expr>),
    Eq(LpBox<Expr>, LpBox<Expr>),
    NotEq(LpBox<Expr>, LpBox<Expr>),

    // Logical
    And(LpBox<Expr>, LpBox<Expr>),
    Or(LpBox<Expr>, LpBox<Expr>),
    Not(LpBox<Expr>),

    // Unary
    Neg(LpBox<Expr>),

    // Increment/Decrement (require l-values)
    PreIncrement(String),
    PreDecrement(String),
    PostIncrement(String),
    PostDecrement(String),

    // Ternary
    Ternary {
        condition: LpBox<Expr>,
        true_expr: LpBox<Expr>,
        false_expr: LpBox<Expr>,
    },

    // Assignment expression (returns the assigned value)
    // In C/GLSL, assignments are expressions: x = y = 5
    Assign {
        target: String,
        value: LpBox<Expr>,
    },

    // Function call
    Call {
        name: String,
        args: Vec<Expr>,
    },

    // Vector constructors (GLSL-style: can take mixed vec/scalar args)
    Vec2Constructor(Vec<Expr>),
    Vec3Constructor(Vec<Expr>),
    Vec4Constructor(Vec<Expr>),

    // Swizzle (component access/reordering)
    Swizzle {
        expr: LpBox<Expr>,
        components: String, // e.g. "xy", "yx", "rgba", "x", etc.
    },
}
