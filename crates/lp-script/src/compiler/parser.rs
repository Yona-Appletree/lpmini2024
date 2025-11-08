/// Parser: converts tokens to AST with spans
extern crate alloc;
use alloc::vec::Vec;

use crate::compiler::ast::{Expr, Program, Stmt};
use crate::compiler::error::{ParseError, ParseErrorKind};
use crate::compiler::lexer::{Token, TokenKind};
use crate::shared::{Span, Type};

// Function parsing is now in compiler::func::func_parse
// Include it here to add impl methods to Parser
#[path = "func/func_parse.rs"]
mod func_parse;

pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) pos: usize,
    recursion_depth: usize,
    max_recursion: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self::with_limits(tokens, 128)
    }

    pub fn with_limits(tokens: Vec<Token>, max_recursion: usize) -> Self {
        Parser {
            tokens,
            pos: 0,
            recursion_depth: 0,
            max_recursion,
        }
    }

    /// Track entering a recursive parse function
    pub(crate) fn enter_recursion(&mut self) -> Result<(), ParseError> {
        self.recursion_depth += 1;
        if self.recursion_depth > self.max_recursion {
            return Err(ParseError {
                kind: ParseErrorKind::RecursionLimitExceeded {
                    max: self.max_recursion,
                },
                span: self.current().span,
            });
        }
        Ok(())
    }

    /// Track exiting a recursive parse function
    pub(crate) fn exit_recursion(&mut self) {
        if self.recursion_depth > 0 {
            self.recursion_depth -= 1;
        }
    }

    pub(crate) fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub(crate) fn advance(&mut self) {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
    }

    /// Parse an expression (expression mode) - delegated to expr module
    // pub fn parse(&mut self) -> Result<Expr, ParseError> is implemented in expr/mod.rs

    /// Parse a full program (script mode)
    pub fn parse_program(mut self) -> Result<Program, ParseError> {
        let start = self.current().span.start;
        let mut functions = Vec::new();
        let mut stmts = Vec::new();

        // Parse function definitions first (must come before statements)
        while self.is_function_definition() {
            functions.push(self.parse_function_def()?);
        }

        // Parse top-level statements
        while !matches!(self.current().kind, TokenKind::Eof) {
            stmts.push(self.parse_stmt()?);
        }

        let end = if !stmts.is_empty() {
            stmts.last().unwrap().span.end
        } else if !functions.is_empty() {
            functions.last().unwrap().span.end
        } else {
            start
        };

        let program = Program {
            functions,
            stmts,
            span: Span::new(start, end),
        };

        Ok(program)
    }

    pub(crate) fn expect(&mut self, expected: TokenKind) -> bool {
        if core::mem::discriminant(&self.current().kind) == core::mem::discriminant(&expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(crate) fn consume_semicolon(&mut self) -> bool {
        if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(crate) fn parse_type(&mut self) -> Type {
        let ty = match &self.current().kind {
            TokenKind::Float => Type::Fixed,
            TokenKind::Int => Type::Int32,
            TokenKind::Vec2 => Type::Vec2,
            TokenKind::Vec3 => Type::Vec3,
            TokenKind::Vec4 => Type::Vec4,
            TokenKind::Void => Type::Void,
            _ => Type::Fixed, // Fallback
        };
        self.advance();
        ty
    }

    pub(crate) fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match &self.current().kind {
            TokenKind::Float
            | TokenKind::Int
            | TokenKind::Vec2
            | TokenKind::Vec3
            | TokenKind::Vec4 => self.parse_var_decl(),
            TokenKind::Return => self.parse_return_stmt(),
            TokenKind::If => self.parse_if_stmt(),
            TokenKind::While => self.parse_while_stmt(),
            TokenKind::For => self.parse_for_stmt(),
            TokenKind::LBrace => self.parse_block(),
            _ => self.parse_expr_stmt(),
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment_expr()
    }
}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use lp_pool::LpMemoryPool;

    use super::*;
    use crate::compiler::ast::ExprKind;
    use crate::compiler::lexer::Lexer;

    const POOL_SIZE: usize = 512 * 1024;

    #[test]
    fn test_parse_simple_expression() {
        let mut memory = vec![0u8; POOL_SIZE];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).expect("pool memory null");
        let pool = unsafe { LpMemoryPool::new(memory_ptr, POOL_SIZE).expect("pool init failed") };

        pool.run(|| -> Result<(), ParseError> {
            let mut lexer = Lexer::new("1.0 + 2.0");
            let tokens = lexer.tokenize();
            let mut parser = Parser::new(tokens);
            let expr = parser.parse()?;

            assert!(matches!(expr.kind, ExprKind::Add(_, _)));
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_parse_program_with_statements() {
        let mut memory = vec![0u8; POOL_SIZE];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).expect("pool memory null");
        let pool = unsafe { LpMemoryPool::new(memory_ptr, POOL_SIZE).expect("pool init failed") };

        pool.run(|| -> Result<(), ParseError> {
            let mut lexer = Lexer::new("float x = 5.0; return x;");
            let tokens = lexer.tokenize();
            let parser = Parser::new(tokens);
            let program = parser.parse_program()?;

            assert_eq!(program.stmts.len(), 2);
            assert!(program.functions.is_empty());
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_parse_program_with_function() {
        let mut memory = vec![0u8; POOL_SIZE];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).expect("pool memory null");
        let pool = unsafe { LpMemoryPool::new(memory_ptr, POOL_SIZE).expect("pool init failed") };

        pool.run(|| -> Result<(), ParseError> {
            let mut lexer =
                Lexer::new("float add(float a, float b) { return a + b; } return add(1.0, 2.0);");
            let tokens = lexer.tokenize();
            let parser = Parser::new(tokens);
            let program = parser.parse_program()?;

            assert_eq!(program.functions.len(), 1);
            assert_eq!(program.stmts.len(), 1);
            Ok(())
        })
        .unwrap();
    }
}
