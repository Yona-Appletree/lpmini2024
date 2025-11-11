/// Abstract Syntax Tree for expressions and statements
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

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

/// Statement with types
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
        then_stmt: Box<Stmt>,
        else_stmt: Option<Box<Stmt>>,
    },

    /// While loop: `while (cond) body`
    While { condition: Expr, body: Box<Stmt> },

    /// For loop: `for (init; condition; increment) body`
    For {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Box<Stmt>,
    },
}

/// Expression with types (span and optional type)
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
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),

    // Bitwise operations (Int32 only)
    BitwiseAnd(Box<Expr>, Box<Expr>),
    BitwiseOr(Box<Expr>, Box<Expr>),
    BitwiseXor(Box<Expr>, Box<Expr>),
    BitwiseNot(Box<Expr>),
    LeftShift(Box<Expr>, Box<Expr>),
    RightShift(Box<Expr>, Box<Expr>),

    // Comparisons
    Less(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    LessEq(Box<Expr>, Box<Expr>),
    GreaterEq(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    NotEq(Box<Expr>, Box<Expr>),

    // Logical
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),

    // Unary
    Neg(Box<Expr>),

    // Increment/Decrement (require l-values)
    PreIncrement(String),
    PreDecrement(String),
    PostIncrement(String),
    PostDecrement(String),

    // Ternary
    Ternary {
        condition: Box<Expr>,
        true_expr: Box<Expr>,
        false_expr: Box<Expr>,
    },

    // Assignment expression (returns the assigned value)
    // In C/GLSL, assignments are expressions: x = y = 5
    Assign {
        target: String,
        value: Box<Expr>,
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
        expr: Box<Expr>,
        components: String, // e.g. "xy", "yx", "rgba", "x", etc.
    },
}
