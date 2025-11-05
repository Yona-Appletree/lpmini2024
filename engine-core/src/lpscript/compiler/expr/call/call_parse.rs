/// Function call parsing
use crate::lpscript::compiler::ast::{Expr, ExprKind};
use crate::lpscript::compiler::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::shared::Span;
use alloc::vec::Vec;


impl Parser {
    // Parse function call
    pub(in crate::lpscript) fn parse_function_call(&mut self) -> Expr {
        let token = self.current().clone();

        if let TokenKind::Ident(name) = &token.kind {
            let name = name.clone();
            let start = token.span.start;
            self.advance();

            if matches!(self.current().kind, TokenKind::LParen) {
                // Function call
                self.advance(); // consume '('
                let args = self.parse_args();
                let end = if matches!(self.current().kind, TokenKind::RParen) {
                    let span = self.current().span;
                    self.advance(); // consume ')'
                    span.end
                } else {
                    self.current().span.end
                };

                let kind = ExprKind::Call { name, args };

                Expr::new(kind, Span::new(start, end))
            } else {
                // Not a function call, return variable
                Expr::new(ExprKind::Variable(name), token.span)
            }
        } else {
            // Error fallback
            Expr::new(ExprKind::Number(0.0), token.span)
        }
    }

    pub(in crate::lpscript) fn parse_args(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();

        if matches!(self.current().kind, TokenKind::RParen) {
            return args;
        }

        loop {
            args.push(self.ternary());
            if matches!(self.current().kind, TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        args
    }
}
