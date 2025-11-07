use crate::compiler::ast::{FunctionDef, Parameter};
use crate::compiler::error::ParseError;
/// Function definition parsing methods
use crate::compiler::parser::Parser;
use crate::lexer::TokenKind;
use crate::shared::Span;
use alloc::string::String;
use alloc::vec::Vec;

impl Parser {
    /// Check if the current position is a function definition
    /// Function def: type name(params) { body }
    pub(in crate) fn is_function_definition(&mut self) -> bool {
        // Look ahead: type + identifier + (
        matches!(
            self.current().kind,
            TokenKind::Float
                | TokenKind::Int
                | TokenKind::Vec2
                | TokenKind::Vec3
                | TokenKind::Vec4
                | TokenKind::Void
        ) && {
            // Look ahead 2 tokens
            let saved_pos = self.pos;
            self.advance(); // Skip type
            let is_ident = matches!(self.current().kind, TokenKind::Ident(_));
            if is_ident {
                self.advance(); // Skip name
                let has_paren = matches!(self.current().kind, TokenKind::LParen);
                self.pos = saved_pos; // Restore position
                has_paren
            } else {
                self.pos = saved_pos;
                false
            }
        }
    }

    /// Parse a function definition
    pub(in crate) fn parse_function_def(&mut self) -> Result<FunctionDef, ParseError> {
        self.enter_recursion()?;
        let start = self.current().span.start;

        // Parse return type
        let return_type = self.parse_type();

        // Parse function name
        let name = if let TokenKind::Ident(n) = &self.current().kind {
            let name = n.clone();
            self.advance();
            name
        } else {
            String::from("error")
        };

        // Parse parameter list
        self.expect(TokenKind::LParen);
        let params = self.parse_parameters();
        self.expect(TokenKind::RParen);

        // Parse body
        self.expect(TokenKind::LBrace);
        let mut body = Vec::new();
        while !matches!(self.current().kind, TokenKind::RBrace | TokenKind::Eof) {
            body.push(self.parse_stmt()?);
        }
        let end = self.current().span.end;
        self.expect(TokenKind::RBrace);

        self.exit_recursion();
        Ok(FunctionDef {
            name,
            params,
            return_type,
            body,
            span: Span::new(start, end),
        })
    }

    /// Parse function parameters
    fn parse_parameters(&mut self) -> Vec<Parameter> {
        let mut params = Vec::new();

        // Empty parameter list
        if matches!(self.current().kind, TokenKind::RParen) {
            return params;
        }

        loop {
            // Parse parameter type
            let ty = self.parse_type();

            // Parse parameter name
            let name = if let TokenKind::Ident(n) = &self.current().kind {
                let name = n.clone();
                self.advance();
                name
            } else {
                String::from("error")
            };

            params.push(Parameter { name, ty });

            // Check for more parameters
            if matches!(self.current().kind, TokenKind::Comma) {
                self.advance(); // consume ','
            } else {
                break;
            }
        }

        params
    }
}
