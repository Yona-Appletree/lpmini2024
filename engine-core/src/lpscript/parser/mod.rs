/// Parser: converts tokens to AST with spans
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use super::ast::{Expr, ExprKind, FunctionDef, Parameter, Program, Stmt, StmtKind};
use super::lexer::{Token, TokenKind};
use crate::lpscript::error::{Span, Type};

mod expr;
mod stmt;
mod functions;

pub struct Parser {
    pub(super) tokens: Vec<Token>,
    pub(super) pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub(super) fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub(super) fn advance(&mut self) {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
    }

    /// Parse an expression (expression mode) - delegated to expr module
    // pub fn parse(&mut self) -> Expr is implemented in expr/mod.rs

    /// Parse a full program (script mode)
    pub fn parse_program(&mut self) -> Program {
        let start = self.current().span.start;
        let mut functions = Vec::new();
        let mut stmts = Vec::new();

        // Parse function definitions first (must come before statements)
        while self.is_function_definition() {
            functions.push(self.parse_function_def());
        }

        // Parse top-level statements
        while !matches!(self.current().kind, TokenKind::Eof) {
            stmts.push(self.parse_stmt());
        }

        let end = if !stmts.is_empty() {
            stmts.last().unwrap().span.end
        } else if !functions.is_empty() {
            functions.last().unwrap().span.end
        } else {
            start
        };

        Program {
            functions,
            stmts,
            span: Span::new(start, end),
        }
    }

    pub(super) fn expect(&mut self, expected: TokenKind) -> bool {
        if core::mem::discriminant(&self.current().kind) == core::mem::discriminant(&expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(super) fn consume_semicolon(&mut self) -> bool {
        if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(super) fn parse_type(&mut self) -> Type {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpscript::lexer::Lexer;

    #[test]
    fn test_parse_simple_expression() {
        let mut lexer = Lexer::new("1.0 + 2.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(expr.kind, ExprKind::Add(_, _)));
    }

    #[test]
    fn test_parse_program_with_statements() {
        let mut lexer = Lexer::new("float x = 5.0; return x;");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();

        assert_eq!(program.stmts.len(), 2);
        assert!(program.functions.is_empty());
    }

    #[test]
    fn test_parse_program_with_function() {
        let mut lexer =
            Lexer::new("float add(float a, float b) { return a + b; } return add(1.0, 2.0);");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();

        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.stmts.len(), 1);
    }
}
