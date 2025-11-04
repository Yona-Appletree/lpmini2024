/// Error types for LightPlayer Script compilation and runtime
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

/// Source code span for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const EMPTY: Span = Span { start: 0, end: 0 };

    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

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
        }
    }
}

/// Type system representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Bool,
    Fixed,
    Int32,
    Vec2,
    Vec3,
    Vec4,
    Void,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "bool"),
            Type::Fixed => write!(f, "float"),
            Type::Int32 => write!(f, "int"),
            Type::Vec2 => write!(f, "vec2"),
            Type::Vec3 => write!(f, "vec3"),
            Type::Vec4 => write!(f, "vec4"),
            Type::Void => write!(f, "void"),
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
        }
    }
}

/// Runtime errors (VM execution)
#[derive(Debug)]
pub enum RuntimeError {
    StackUnderflow {
        required: usize,
        actual: usize,
    },
    StackOverflow {
        sp: usize,
    },
    LocalTypeMismatch {
        local_idx: usize,
        local_name: String,
        expected: &'static str,
        found: &'static str,
    },
    LocalOutOfBounds {
        local_idx: usize,
        max: usize,
    },
    DivisionByZero,
    InvalidTextureCoords {
        u: i32,
        v: i32,
        texture_idx: usize,
    },
    InvalidArrayIndex {
        index: i32,
        array_size: usize,
    },
    ProgramCounterOutOfBounds {
        pc: usize,
        max: usize,
    },
    TypeMismatch,
    UnsupportedOpCode,
    InstructionLimitExceeded,
    CallStackOverflow {
        depth: usize,
    },
}

impl RuntimeError {
    /// Add execution context (PC, opcode name) to the error
    pub fn with_context(self, pc: usize, opcode: &'static str) -> RuntimeErrorWithContext {
        RuntimeErrorWithContext {
            error: self,
            pc,
            opcode,
        }
    }
}

/// Runtime error with execution context
#[derive(Debug)]
pub struct RuntimeErrorWithContext {
    pub error: RuntimeError,
    pub pc: usize,
    pub opcode: &'static str,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::StackUnderflow { required, actual } => {
                write!(
                    f,
                    "Stack underflow: need {} items, have {}",
                    required, actual
                )
            }
            RuntimeError::StackOverflow { sp } => {
                write!(f, "Stack overflow at sp={}", sp)
            }
            RuntimeError::LocalTypeMismatch {
                local_idx,
                local_name,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Local type mismatch for '{}' (index {}): expected {}, found {}",
                    local_name, local_idx, expected, found
                )
            }
            RuntimeError::LocalOutOfBounds { local_idx, max } => {
                write!(f, "Local index {} out of bounds (max {})", local_idx, max)
            }
            RuntimeError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            RuntimeError::InvalidTextureCoords { u, v, texture_idx } => {
                write!(
                    f,
                    "Invalid texture coordinates ({}, {}) for texture {}",
                    u, v, texture_idx
                )
            }
            RuntimeError::InvalidArrayIndex { index, array_size } => {
                write!(
                    f,
                    "Array index {} out of bounds (size {})",
                    index, array_size
                )
            }
            RuntimeError::ProgramCounterOutOfBounds { pc, max } => {
                write!(f, "Program counter {} out of bounds (max {})", pc, max)
            }
            RuntimeError::TypeMismatch => {
                write!(f, "Type mismatch in operation")
            }
            RuntimeError::UnsupportedOpCode => {
                write!(f, "Unsupported opcode encountered")
            }
            RuntimeError::InstructionLimitExceeded => {
                write!(f, "Instruction limit exceeded (possible infinite loop)")
            }
            RuntimeError::CallStackOverflow { depth } => {
                write!(f, "Call stack overflow at depth {}", depth)
            }
        }
    }
}

impl fmt::Display for RuntimeErrorWithContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Runtime error at PC {} ({}): {}",
            self.pc, self.opcode, self.error
        )
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

impl From<CodegenError> for CompileError {
    fn from(e: CodegenError) -> Self {
        CompileError::Codegen(e)
    }
}
