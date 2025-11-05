/// Literal parsing (numbers, parenthesized expressions)
use crate::lpscript::compiler::ast::{Expr, ExprKind};
use crate::lpscript::compiler::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::error::Span;
use alloc::boxed::Box;


impl Parser {
    // Unary: - ! (between exponential and postfix in precedence)
    pub(in crate::lpscript) fn unary(&mut self) -> Expr {
        let token = self.current().clone();

        match &token.kind {
            TokenKind::Minus => {
                self.advance();
                let operand = self.unary(); // Right-associative (can stack: --x)
                let end = operand.span.end;

                // Optimization: if operand is a literal, fold the negation
                match operand.kind {
                    ExprKind::Number(n) => {
                        Expr::new(ExprKind::Number(-n), Span::new(token.span.start, end))
                    }
                    ExprKind::IntNumber(n) => {
                        Expr::new(ExprKind::IntNumber(-n), Span::new(token.span.start, end))
                    }
                    _ => Expr::new(
                        ExprKind::Neg(Box::new(operand)),
                        Span::new(token.span.start, end),
                    ),
                }
            }
            TokenKind::Bang => {
                self.advance();
                let operand = self.unary();
                let end = operand.span.end;
                Expr::new(
                    ExprKind::Not(Box::new(operand)),
                    Span::new(token.span.start, end),
                )
            }
            _ => self.postfix(),
        }
    }

    // Primary: number, variable, function call, constructor, or parenthesized expression
    pub(in crate::lpscript) fn primary(&mut self) -> Expr {
        let token = self.current().clone();

        match &token.kind {
            TokenKind::FloatLiteral(n) => {
                self.advance();
                Expr::new(ExprKind::Number(*n), token.span)
            }
            TokenKind::IntLiteral(n) => {
                self.advance();
                Expr::new(ExprKind::IntNumber(*n), token.span)
            }
            TokenKind::LParen => {
                self.advance(); // consume '('
                let expr = self.parse_assignment_expr();
                if matches!(self.current().kind, TokenKind::RParen) {
                    self.advance(); // consume ')'
                }
                expr
            }
            TokenKind::Vec2 | TokenKind::Vec3 | TokenKind::Vec4 => self.parse_vec_constructor(),
            TokenKind::Ident(_) => self.parse_ident(),
            _ => {
                // Error fallback
                Expr::new(ExprKind::Number(0.0), token.span)
            }
        }
    }
}
