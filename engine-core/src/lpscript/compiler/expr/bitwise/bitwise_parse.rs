/// Bitwise operator parsing (&, |, ^, ~, <<, >>)
use crate::lpscript::compiler::ast::{Expr, ExprKind};
use crate::lpscript::compiler::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::shared::Span;
use alloc::boxed::Box;


impl Parser {
    // Bitwise OR: |
    pub(in crate::lpscript) fn bitwise_or(&mut self) -> Expr {
        let mut expr = self.bitwise_xor();

        while matches!(self.current().kind, TokenKind::Pipe) {
            let start = expr.span.start;
            self.advance();
            let right = self.bitwise_xor();
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::BitwiseOr(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }

        expr
    }

    // Bitwise XOR: ^
    pub(in crate::lpscript) fn bitwise_xor(&mut self) -> Expr {
        let mut expr = self.bitwise_and();

        while matches!(self.current().kind, TokenKind::Caret) {
            let start = expr.span.start;
            self.advance();
            let right = self.bitwise_and();
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::BitwiseXor(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }

        expr
    }

    // Bitwise AND: &
    pub(in crate::lpscript) fn bitwise_and(&mut self) -> Expr {
        let mut expr = self.equality();

        while matches!(self.current().kind, TokenKind::Ampersand) {
            let start = expr.span.start;
            self.advance();
            let right = self.equality();
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::BitwiseAnd(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }

        expr
    }

    // Equality: == !=
    pub(in crate::lpscript) fn equality(&mut self) -> Expr {
        let mut expr = self.relational();

        loop {
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::EqEq => {
                    self.advance();
                    let right = self.relational();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Eq(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::NotEq => {
                    self.advance();
                    let right = self.relational();
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

    // Relational: < > <= >=
    pub(in crate::lpscript) fn relational(&mut self) -> Expr {
        let mut expr = self.shift();

        loop {
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::Less => {
                    self.advance();
                    let right = self.shift();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Less(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::Greater => {
                    self.advance();
                    let right = self.shift();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Greater(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::LessEq => {
                    self.advance();
                    let right = self.shift();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::LessEq(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::GreaterEq => {
                    self.advance();
                    let right = self.shift();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::GreaterEq(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                _ => break,
            }
        }

        expr
    }

    // Shift: << >>
    pub(in crate::lpscript) fn shift(&mut self) -> Expr {
        let mut expr = self.additive();

        loop {
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::LShift => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::LeftShift(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::RShift => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::RightShift(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                _ => break,
            }
        }

        expr
    }
}

