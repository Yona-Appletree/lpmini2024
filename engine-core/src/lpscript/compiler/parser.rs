/// Parser: converts tokens to AST with spans
extern crate alloc;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{Expr, Program, Stmt};
use crate::lpscript::compiler::lexer::{Token, TokenKind};
use crate::lpscript::error::{Span, Type};

// Function parsing is now in compiler::func::func_parse
// Include it here to add impl methods to Parser
#[path = "func/func_parse.rs"]
mod func_parse;

pub struct Parser {
    pub(in crate::lpscript) tokens: Vec<Token>,
    pub(in crate::lpscript) pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub(in crate::lpscript) fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub(in crate::lpscript) fn advance(&mut self) {
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

    pub(in crate::lpscript) fn expect(&mut self, expected: TokenKind) -> bool {
        if core::mem::discriminant(&self.current().kind) == core::mem::discriminant(&expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(in crate::lpscript) fn consume_semicolon(&mut self) -> bool {
        if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub(in crate::lpscript) fn parse_type(&mut self) -> Type {
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

    pub(in crate::lpscript) fn parse_stmt(&mut self) -> Stmt {
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
            TokenKind::Ident(name) => {
                let name = name.clone();
                let start = self.current().span.start;
                self.advance();

                if matches!(self.current().kind, TokenKind::Eq) {
                    // Assignment statement
                    self.parse_assignment_stmt(name, start)
                } else {
                    // Put back the token and parse as expression statement
                    self.pos -= 1;
                    self.parse_expr_stmt()
                }
            }
            _ => self.parse_expr_stmt(),
        }
    }

    pub(in crate::lpscript) fn parse(&mut self) -> Expr {
        self.parse_assignment_expr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpscript::compiler::ast::ExprKind;
    use crate::lpscript::compiler::lexer::Lexer;

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
