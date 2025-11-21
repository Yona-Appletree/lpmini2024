/// Error types for LightPlayer Script compilation
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use lp_alloc::AllocLimitError;

use crate::lp_script::shared::{Span, Type};

/// Comprehensive compilation error
#[derive(Debug)]
pub enum CompileError {
    Lexer(LexerError),
    Parser(ParseError),
    TypeCheck(TypeError),
    Codegen(CodegenError),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::Lexer(e) => write!(f, "{}", e),
            CompileError::Parser(e) => write!(f, "{}", e),
            CompileError::TypeCheck(e) => write!(f, "{}", e),
            CompileError::Codegen(e) => write!(f, "{}", e),
        }
    }
}

/// Lexer errors
#[derive(Debug)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum LexerErrorKind {
    InvalidNumber(String),
    UnexpectedChar(char),
    UnterminatedString,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexer error at {}:{}: ", self.span.start, self.span.end)?;
        match &self.kind {
            LexerErrorKind::InvalidNumber(s) => write!(f, "invalid number '{}'", s),
            LexerErrorKind::UnexpectedChar(c) => write!(f, "unexpected character '{}'", c),
            LexerErrorKind::UnterminatedString => write!(f, "unterminated string"),
        }
    }
}

/// Parser errors
#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken { expected: String, found: String },
    UnexpectedEof,
    InvalidExpression,
    RecursionLimitExceeded { max: usize },
    ExprLimitExceeded { max: usize },
    StmtLimitExceeded { max: usize },
    AllocationFailed(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at {}:{}: ", self.span.start, self.span.end)?;
        match &self.kind {
            ParseErrorKind::UnexpectedToken { expected, found } => {
                write!(f, "expected {}, found {}", expected, found)
            }
            ParseErrorKind::UnexpectedEof => write!(f, "unexpected end of input"),
            ParseErrorKind::InvalidExpression => write!(f, "invalid expression"),
            ParseErrorKind::RecursionLimitExceeded { max } => {
                write!(f, "parser recursion limit exceeded (max: {})", max)
            }
            ParseErrorKind::ExprLimitExceeded { max } => {
                write!(f, "expression node limit exceeded (max: {})", max)
            }
            ParseErrorKind::StmtLimitExceeded { max } => {
                write!(f, "statement node limit exceeded (max: {})", max)
            }
            ParseErrorKind::AllocationFailed(msg) => write!(f, "allocation failed: {}", msg),
        }
    }
}

impl From<AllocLimitError> for ParseError {
    fn from(e: AllocLimitError) -> Self {
        use alloc::format;
        ParseError {
            kind: ParseErrorKind::AllocationFailed(format!("{}", e)),
            span: Span::new(0, 0),
        }
    }
}

/// Type checking errors
#[derive(Debug)]
pub struct TypeError {
    pub kind: TypeErrorKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum TypeErrorKind {
    Mismatch { expected: Type, found: Type },
    UndefinedVariable(String),
    UndefinedFunction(String),
    InvalidArgumentCount { expected: usize, found: usize },
    InvalidOperation { op: String, types: Vec<Type> },
    InvalidSwizzle(String),
    MissingReturn(String),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type error at {}:{}: ", self.span.start, self.span.end)?;
        match &self.kind {
            TypeErrorKind::Mismatch { expected, found } => {
                write!(f, "type mismatch: expected {}, found {}", expected, found)
            }
            TypeErrorKind::UndefinedVariable(name) => {
                write!(f, "undefined variable '{}'", name)
            }
            TypeErrorKind::UndefinedFunction(name) => {
                write!(f, "undefined function '{}'", name)
            }
            TypeErrorKind::InvalidArgumentCount { expected, found } => {
                write!(f, "expected {} arguments, found {}", expected, found)
            }
            TypeErrorKind::InvalidOperation { op, types } => {
                write!(f, "invalid operation '{}' for types {:?}", op, types)
            }
            TypeErrorKind::InvalidSwizzle(msg) => {
                write!(f, "invalid swizzle: {}", msg)
            }
            TypeErrorKind::MissingReturn(name) => {
                write!(
                    f,
                    "function '{}' is missing a return statement on all code paths",
                    name
                )
            }
        }
    }
}

/// Code generation errors
#[derive(Debug)]
pub struct CodegenError {
    pub kind: CodegenErrorKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum CodegenErrorKind {
    UnsupportedFeature(String),
    TooManyLocals,
    TooManyOpcodes,
    AllocationFailed(String),
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Codegen error at {}:{}: ",
            self.span.start, self.span.end
        )?;
        match &self.kind {
            CodegenErrorKind::UnsupportedFeature(feat) => {
                write!(f, "unsupported feature: {}", feat)
            }
            CodegenErrorKind::TooManyLocals => write!(f, "too many local variables"),
            CodegenErrorKind::TooManyOpcodes => write!(f, "program too large"),
            CodegenErrorKind::AllocationFailed(msg) => write!(f, "allocation failed: {}", msg),
        }
    }
}

impl From<LexerError> for CompileError {
    fn from(e: LexerError) -> Self {
        CompileError::Lexer(e)
    }
}

impl From<ParseError> for CompileError {
    fn from(e: ParseError) -> Self {
        CompileError::Parser(e)
    }
}

impl From<TypeError> for CompileError {
    fn from(e: TypeError) -> Self {
        CompileError::TypeCheck(e)
    }
}

impl From<AllocLimitError> for CompileError {
    fn from(e: AllocLimitError) -> Self {
        use alloc::format;
        CompileError::Codegen(CodegenError {
            kind: CodegenErrorKind::AllocationFailed(format!("{}", e)),
            span: Span::new(0, 0),
        })
    }
}

impl From<CodegenError> for CompileError {
    fn from(e: CodegenError) -> Self {
        CompileError::Codegen(e)
    }
}
