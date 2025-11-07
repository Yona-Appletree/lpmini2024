/// Function call parsing
use crate::compiler::ast::{ExprId, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;
use alloc::vec::Vec;


impl Parser {
    // Parse function call
    pub(in crate) fn parse_function_call(&mut self) -> Result<ExprId, ParseError> {
        let token = self.current().clone();

        if let TokenKind::Ident(name) = &token.kind {
            let name = name.clone();
            let start = token.span.start;
            self.advance();

            if matches!(self.current().kind, TokenKind::LParen) {
                // Function call
                self.advance(); // consume '('
                let args = self.parse_args()?;
                let end = if matches!(self.current().kind, TokenKind::RParen) {
                    let span = self.current().span;
                    self.advance(); // consume ')'
                    span.end
                } else {
                    self.current().span.end
                };

                let kind = ExprKind::Call { name, args };

                self.pool
                    .alloc_expr(kind, Span::new(start, end))
                    .map_err(|e| self.pool_error_to_parse_error(e))
            } else {
                // Not a function call, return variable
                self.pool
                    .alloc_expr(ExprKind::Variable(name), token.span)
                    .map_err(|e| self.pool_error_to_parse_error(e))
            }
        } else {
            // Error fallback
            self.pool
                .alloc_expr(ExprKind::Number(0.0), token.span)
                .map_err(|e| self.pool_error_to_parse_error(e))
        }
    }

    pub(in crate) fn parse_args(&mut self) -> Result<Vec<ExprId>, ParseError> {
        let mut args = Vec::new();

        if matches!(self.current().kind, TokenKind::RParen) {
            return Ok(args);
        }

        loop {
            args.push(self.ternary()?);
            if matches!(self.current().kind, TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        Ok(args)
    }
}
