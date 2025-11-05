/// Comparison operator parsing (<, >, <=, >=, ==, !=)
use crate::lpscript::compiler::ast::{Expr, ExprKind};
use crate::lpscript::compiler::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::error::Span;
use alloc::boxed::Box;


impl Parser {
    // Comparison: <, >, <=, >=, ==, !=
    pub(in crate::lpscript) fn comparison(&mut self) -> Expr {
        let mut expr = self.additive();

        loop {
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::Less => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Less(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::Greater => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Greater(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::LessEq => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::LessEq(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::GreaterEq => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::GreaterEq(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::EqEq => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Eq(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::NotEq => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::NotEq(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                _ => break,
            }
        }

        expr
    }
}
