use alloc::boxed::Box;

/// Ternary expression parsing
use crate::lp_script::compiler::ast::{Expr, ExprKind};
use crate::lp_script::compiler::error::ParseError;
use crate::lp_script::compiler::lexer::TokenKind;
use crate::lp_script::compiler::parser::Parser;
use crate::lp_script::shared::Span;

impl Parser {
    // Ternary: condition ? true_expr : false_expr
    pub(crate) fn ternary(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let mut expr = self.logical_or()?;

        if matches!(self.current().kind, TokenKind::Question) {
            let start = expr.span.start;
            self.advance(); // consume '?'
            let true_expr = self.ternary()?;

            if matches!(self.current().kind, TokenKind::Colon) {
                self.advance(); // consume ':'
                let false_expr = self.ternary()?;
                let end = false_expr.span.end;

                expr = Expr::new(
                    ExprKind::Ternary {
                        condition: Box::new(expr),
                        true_expr: Box::new(true_expr),
                        false_expr: Box::new(false_expr),
                    },
                    Span::new(start, end),
                );
            }
        }

        self.exit_recursion();
        Ok(expr)
    }
}
