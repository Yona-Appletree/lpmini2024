/// Ternary expression parsing
use crate::compiler::ast::{ExprId, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    // Ternary: condition ? true_expr : false_expr
    pub(crate) fn ternary(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let mut expr_id = self.logical_or()?;

        if matches!(self.current().kind, TokenKind::Question) {
            let start = self.pool.expr(expr_id).span.start;
            self.advance(); // consume '?'
            let true_expr_id = self.ternary()?;

            if matches!(self.current().kind, TokenKind::Colon) {
                self.advance(); // consume ':'
                let false_expr_id = self.ternary()?;
                let end = self.pool.expr(false_expr_id).span.end;

                expr_id = self
                    .pool
                    .alloc_expr(
                        ExprKind::Ternary {
                            condition: expr_id,
                            true_expr: true_expr_id,
                            false_expr: false_expr_id,
                        },
                        Span::new(start, end),
                    )
                    .map_err(|e| self.pool_error_to_parse_error(e))?;
            }
        }

        self.exit_recursion();
        Ok(expr_id)
    }
}
