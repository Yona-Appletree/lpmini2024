/// Binary operator parsing (+, -, *, /, %, ^)
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use alloc::boxed::Box;

impl Parser {
    // Additive: + -
    pub(in crate::lpscript) fn additive(&mut self) -> Expr {
        let mut expr = self.multiplicative();

        loop {
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::Plus => {
                    self.advance();
                    let right = self.multiplicative();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Add(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::Minus => {
                    self.advance();
                    let right = self.multiplicative();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Sub(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                _ => break,
            }
        }

        expr
    }

    // Multiplicative: * / %
    pub(in crate::lpscript) fn multiplicative(&mut self) -> Expr {
        let mut expr = self.exponential();

        loop {
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::Star => {
                    self.advance();
                    let right = self.exponential();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Mul(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::Slash => {
                    self.advance();
                    let right = self.exponential();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Div(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::Percent => {
                    self.advance();
                    let right = self.exponential();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Mod(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                _ => break,
            }
        }

        expr
    }

    // Exponential: ^ (right-associative)
    pub(in crate::lpscript) fn exponential(&mut self) -> Expr {
        let mut expr = self.postfix();

        if matches!(self.current().kind, TokenKind::Caret) {
            let start = expr.span.start;
            self.advance();
            let right = self.exponential(); // Right-associative
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::Pow(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }

        expr
    }
}

