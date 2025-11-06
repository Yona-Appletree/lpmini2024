/// Comparison operator parsing (<, >, <=, >=, ==, !=)
use crate::lpscript::compiler::ast::{ExprId, ExprKind};
use crate::lpscript::compiler::error::ParseError;
use crate::lpscript::compiler::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::shared::Span;

impl Parser {
    // Equality: == !=
    pub(in crate::lpscript) fn equality(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let mut expr_id = self.relational()?;

        loop {
            let start = self.pool.expr(expr_id).span.start;
            match &self.current().kind {
                TokenKind::EqEq => {
                    self.advance();
                    let right_id = self.relational()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(ExprKind::Eq(expr_id, right_id), Span::new(start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e))?;
                }
                TokenKind::NotEq => {
                    self.advance();
                    let right_id = self.relational()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(ExprKind::NotEq(expr_id, right_id), Span::new(start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e))?;
                }
                _ => break,
            }
        }

        self.exit_recursion();
        Ok(expr_id)
    }

    // Relational: < > <= >=
    pub(in crate::lpscript) fn relational(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let mut expr_id = self.shift()?;

        loop {
            let start = self.pool.expr(expr_id).span.start;
            match &self.current().kind {
                TokenKind::Less => {
                    self.advance();
                    let right_id = self.shift()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(ExprKind::Less(expr_id, right_id), Span::new(start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e))?;
                }
                TokenKind::Greater => {
                    self.advance();
                    let right_id = self.shift()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(ExprKind::Greater(expr_id, right_id), Span::new(start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e))?;
                }
                TokenKind::LessEq => {
                    self.advance();
                    let right_id = self.shift()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(ExprKind::LessEq(expr_id, right_id), Span::new(start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e))?;
                }
                TokenKind::GreaterEq => {
                    self.advance();
                    let right_id = self.shift()?;
                    let end = self.pool.expr(right_id).span.end;
                    expr_id = self
                        .pool
                        .alloc_expr(
                            ExprKind::GreaterEq(expr_id, right_id),
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
