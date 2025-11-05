/// Literal parsing (numbers, parenthesized expressions)
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;

impl Parser {
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
                let expr = self.ternary();
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

