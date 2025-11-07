/// Logical operator parsing (&&, ||)
use crate::compiler::ast::{ExprId, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;


impl Parser {
    // Logical OR: ||
    pub(in crate) fn logical_or(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let mut expr_id = self.logical_and()?;

        while matches!(self.current().kind, TokenKind::Or) {
            let start = self.pool.expr(expr_id).span.start;
            self.advance();
            let right_id = self.logical_and()?;
            let end = self.pool.expr(right_id).span.end;
            expr_id = self
                .pool
                .alloc_expr(ExprKind::Or(expr_id, right_id), Span::new(start, end))
                .map_err(|e| self.pool_error_to_parse_error(e))?;
        }

        self.exit_recursion();
        Ok(expr_id)
    }

    // Logical AND: &&
    pub(in crate) fn logical_and(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let mut expr_id = self.bitwise_or()?;

        while matches!(self.current().kind, TokenKind::And) {
            let start = self.pool.expr(expr_id).span.start;
            self.advance();
            let right_id = self.bitwise_or()?;
            let end = self.pool.expr(right_id).span.end;
            expr_id = self
                .pool
                .alloc_expr(ExprKind::And(expr_id, right_id), Span::new(start, end))
                .map_err(|e| self.pool_error_to_parse_error(e))?;
        }

        self.exit_recursion();
        Ok(expr_id)
    }
}
