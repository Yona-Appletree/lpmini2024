use alloc::boxed::Box;

/// Ternary expression parsing
use crate::compiler::ast::{Expr, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

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
                        condition: LpBox::try_new(expr)?,
                        true_expr: LpBox::try_new(true_expr)?,
                        false_expr: LpBox::try_new(false_expr)?,
                    },
                    Span::new(start, end),
                );
            }
        }

        self.exit_recursion();
        Ok(expr)
    }
}
