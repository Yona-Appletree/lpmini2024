/// Ternary expression parsing
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use alloc::boxed::Box;

impl Parser {
    // Ternary: condition ? true_expr : false_expr
    pub(in crate::lpscript) fn ternary(&mut self) -> Expr {
        let mut expr = self.logical_or();

        if matches!(self.current().kind, TokenKind::Question) {
            let start = expr.span.start;
            self.advance(); // consume '?'
            let true_expr = Box::new(self.ternary());

            if matches!(self.current().kind, TokenKind::Colon) {
                self.advance(); // consume ':'
                let false_expr = Box::new(self.ternary());
                let end = false_expr.span.end;

                expr = Expr::new(
                    ExprKind::Ternary {
                        condition: Box::new(expr),
                        true_expr,
                        false_expr,
                    },
                    Span::new(start, end),
                );
            }
        }

        expr
    }
}

