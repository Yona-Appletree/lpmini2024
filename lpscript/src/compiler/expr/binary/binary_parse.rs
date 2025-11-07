/// Binary operator parsing (+, -, *, /, %, ^)
use crate::compiler::ast::{ExprId, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;


impl Parser {
    // Additive: + -
    pub(in crate) fn additive(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let mut expr_id = self.multiplicative()?;

        loop {
            let start = self.pool.expr(expr_id).span.start;
            match &self.current().kind {
                TokenKind::Plus => {
                    self.advance();
                    let right_id = self.multiplicative()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(ExprKind::Add(expr_id, right_id), Span::new(start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e))?;
                }
                TokenKind::Minus => {
                    self.advance();
                    let right_id = self.multiplicative()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(ExprKind::Sub(expr_id, right_id), Span::new(start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e))?;
                }
                _ => break,
            }
        }

        self.exit_recursion();
        Ok(expr_id)
    }

    // Multiplicative: * / %
    pub(in crate) fn multiplicative(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let mut expr_id = self.exponential()?;

        loop {
            let start = self.pool.expr(expr_id).span.start;
            match &self.current().kind {
                TokenKind::Star => {
                    self.advance();
                    let right_id = self.exponential()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(ExprKind::Mul(expr_id, right_id), Span::new(start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e))?;
                }
                TokenKind::Slash => {
                    self.advance();
                    let right_id = self.exponential()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(ExprKind::Div(expr_id, right_id), Span::new(start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e))?;
                }
                TokenKind::Percent => {
                    self.advance();
                    let right_id = self.exponential()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(ExprKind::Mod(expr_id, right_id), Span::new(start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e))?;
                }
                _ => break,
            }
        }

        self.exit_recursion();
        Ok(expr_id)
    }

    // Exponential: ^ removed (use pow() function instead)
    // This now just delegates to unary, will be re-added as bitwise XOR in Phase 2
    pub(in crate) fn exponential(&mut self) -> Result<ExprId, ParseError> {
        self.unary()
    }
}
