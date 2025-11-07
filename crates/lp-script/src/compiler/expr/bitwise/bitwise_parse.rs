/// Bitwise operator parsing (&, |, ^, ~, <<, >>)
use crate::compiler::ast::{ExprId, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    // Bitwise OR: |
    pub(crate) fn bitwise_or(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let mut expr_id = self.bitwise_xor()?;

        while matches!(self.current().kind, TokenKind::Pipe) {
            let start = self.pool.expr(expr_id).span.start;
            self.advance();
            let right_id = self.bitwise_xor()?;
            let end = self.pool.expr(right_id).span.end;
            expr_id = self
                .pool
                .alloc_expr(
                    ExprKind::BitwiseOr(expr_id, right_id),
                    Span::new(start, end),
                )
                .map_err(|e| self.pool_error_to_parse_error(e))?;
        }

        self.exit_recursion();
        Ok(expr_id)
    }

    // Bitwise XOR: ^
    pub(crate) fn bitwise_xor(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let mut expr_id = self.bitwise_and()?;

        while matches!(self.current().kind, TokenKind::Caret) {
            let start = self.pool.expr(expr_id).span.start;
            self.advance();
            let right_id = self.bitwise_and()?;
            let end = self.pool.expr(right_id).span.end;
            expr_id = self
                .pool
                .alloc_expr(
                    ExprKind::BitwiseXor(expr_id, right_id),
                    Span::new(start, end),
                )
                .map_err(|e| self.pool_error_to_parse_error(e))?;
        }

        self.exit_recursion();
        Ok(expr_id)
    }

    // Bitwise AND: &
    pub(crate) fn bitwise_and(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let mut expr_id = self.equality()?;

        while matches!(self.current().kind, TokenKind::Ampersand) {
            let start = self.pool.expr(expr_id).span.start;
            self.advance();
            let right_id = self.equality()?;
            let end = self.pool.expr(right_id).span.end;
            expr_id = self
                .pool
                .alloc_expr(
                    ExprKind::BitwiseAnd(expr_id, right_id),
                    Span::new(start, end),
                )
                .map_err(|e| self.pool_error_to_parse_error(e))?;
        }

        self.exit_recursion();
        Ok(expr_id)
    }

    // Shift: << >>
    pub(crate) fn shift(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let mut expr_id = self.additive()?;

        loop {
            let start = self.pool.expr(expr_id).span.start;
            match &self.current().kind {
                TokenKind::LShift => {
                    self.advance();
                    let right_id = self.additive()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(
                            ExprKind::LeftShift(expr_id, right_id),
                            Span::new(start, end),
                        )
                        .map_err(|e| self.pool_error_to_parse_error(e))?;
                }
                TokenKind::RShift => {
                    self.advance();
                    let right_id = self.additive()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(
                            ExprKind::RightShift(expr_id, right_id),
                            Span::new(start, end),
                        )
                        .map_err(|e| self.pool_error_to_parse_error(e))?;
                }
                _ => break,
            }
        }

        self.exit_recursion();
        Ok(expr_id)
    }
}
