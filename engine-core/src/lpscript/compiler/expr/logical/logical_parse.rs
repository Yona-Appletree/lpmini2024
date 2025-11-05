/// Logical operator parsing (&&, ||)
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use alloc::boxed::Box;

impl Parser {
    // Logical OR: ||
    pub(in crate::lpscript) fn logical_or(&mut self) -> Expr {
        let mut expr = self.logical_and();

        while matches!(self.current().kind, TokenKind::Or) {
            let start = expr.span.start;
            self.advance();
            let right = self.logical_and();
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::Or(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }

        expr
    }

    // Logical AND: &&
    pub(in crate::lpscript) fn logical_and(&mut self) -> Expr {
        let mut expr = self.comparison();

        while matches!(self.current().kind, TokenKind::And) {
            let start = expr.span.start;
            self.advance();
            let right = self.comparison();
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::And(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }

        expr
    }
}

