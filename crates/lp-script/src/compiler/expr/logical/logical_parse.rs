use alloc::boxed::Box;

/// Logical operator parsing (&&, ||)
use crate::compiler::ast::{Expr, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    // Logical OR: ||
    pub(crate) fn logical_or(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let mut expr = self.logical_and()?;

        while matches!(self.current().kind, TokenKind::Or) {
            let start = expr.span.start;
            self.advance();
            let right = self.logical_and()?;
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::Or(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }

        self.exit_recursion();
        Ok(expr)
    }

    // Logical AND: &&
    pub(crate) fn logical_and(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let mut expr = self.bitwise_or()?;

        while matches!(self.current().kind, TokenKind::And) {
            let start = expr.span.start;
            self.advance();
            let right = self.bitwise_or()?;
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::And(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }

        self.exit_recursion();
        Ok(expr)
    }
}
