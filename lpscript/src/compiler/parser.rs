/// Parser: converts tokens to AST with spans
extern crate alloc;
use alloc::vec::Vec;

use crate::compiler::ast::{AstPool, ExprId, Program, StmtId};
use crate::compiler::error::{ParseError, ParseErrorKind};
use crate::compiler::lexer::{Token, TokenKind};
use crate::shared::{Span, Type};

// Function parsing is now in compiler::func::func_parse
// Include it here to add impl methods to Parser
#[path = "func/func_parse.rs"]
mod func_parse;

pub struct Parser {
    pub(in crate) tokens: Vec<Token>,
    pub(in crate) pos: usize,
    pub(in crate) pool: AstPool,
    recursion_depth: usize,
    max_recursion: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self::with_limits(tokens, 128, 20000, 10000)
    }

    pub fn with_limits(
        tokens: Vec<Token>,
        max_recursion: usize,
        max_exprs: usize,
        max_stmts: usize,
    ) -> Self {
        Parser {
            tokens,
            pos: 0,
            pool: AstPool::with_capacity(max_exprs, max_stmts),
            recursion_depth: 0,
            max_recursion,
        }
    }

    /// Track entering a recursive parse function
    pub(in crate) fn enter_recursion(&mut self) -> Result<(), ParseError> {
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
    pub(in crate) fn exit_recursion(&mut self) {
        if self.recursion_depth > 0 {
            self.recursion_depth -= 1;
        }
    }

    /// Helper to convert AstPoolError to ParseError
    pub(in crate) fn pool_error_to_parse_error(
        &self,
        err: crate::compiler::ast::AstPoolError,
    ) -> ParseError {
        use crate::compiler::ast::AstPoolError;
        let kind = match err {
            AstPoolError::ExprLimitExceeded { max } => ParseErrorKind::ExprLimitExceeded { max },
            AstPoolError::StmtLimitExceeded { max } => ParseErrorKind::StmtLimitExceeded { max },
        };
        ParseError {
            kind,
            span: self.current().span,
        }
    }

    pub(in crate) fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub(in crate) fn advance(&mut self) {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
    }

    /// Parse an expression (expression mode) - delegated to expr module
    // pub fn parse(&mut self) -> Result<ExprId, ParseError> is implemented in expr/mod.rs

    /// Parse a full program (script mode)
    /// Consumes the parser and returns the program along with the AST pool
    pub fn parse_program(mut self) -> Result<(Program, AstPool), ParseError> {
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
            self.pool.stmt(*stmts.last().unwrap()).span.end
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

        Ok((program, self.pool))
    }

    pub(in crate) fn expect(&mut self, expected: TokenKind) -> bool {
        if core::mem::discriminant(&self.current().kind) == core::mem::discriminant(&expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(in crate) fn consume_semicolon(&mut self) -> bool {
        if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(in crate) fn parse_type(&mut self) -> Type {
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

    pub(in crate) fn parse_stmt(&mut self) -> Result<StmtId, ParseError> {
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

    pub(in crate) fn parse(&mut self) -> Result<ExprId, ParseError> {
        self.parse_assignment_expr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::ast::ExprKind;
    use crate::compiler::lexer::Lexer;

    #[test]
    fn test_parse_simple_expression() {
        let mut lexer = Lexer::new("1.0 + 2.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr_id = parser.parse().unwrap();
        let expr = parser.pool.expr(expr_id);

        assert!(matches!(expr.kind, ExprKind::Add(_, _)));
    }

    #[test]
    fn test_parse_program_with_statements() {
        let mut lexer = Lexer::new("float x = 5.0; return x;");
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        let (program, _pool) = parser.parse_program().unwrap();

        assert_eq!(program.stmts.len(), 2);
        assert!(program.functions.is_empty());
    }

    #[test]
    fn test_parse_program_with_function() {
        let mut lexer =
            Lexer::new("float add(float a, float b) { return a + b; } return add(1.0, 2.0);");
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        let (program, _pool) = parser.parse_program().unwrap();

        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.stmts.len(), 1);
    }
}
