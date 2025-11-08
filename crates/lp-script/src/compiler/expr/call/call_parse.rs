use alloc::vec::Vec;

/// Function call parsing
use crate::compiler::ast::{Expr, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    // Parse function call
    pub(crate) fn parse_function_call(&mut self) -> Result<Expr, ParseError> {
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
                Ok(Expr::new(kind, Span::new(start, end)))
            } else {
                // Not a function call, return variable
                Ok(Expr::new(ExprKind::Variable(name), token.span))
            }
        } else {
            // Error fallback
            Ok(Expr::new(ExprKind::Number(0.0), token.span))
        }
    }

    pub(crate) fn parse_args(&mut self) -> Result<Vec<Expr>, ParseError> {
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
