/// Vector constructor parsing
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;

impl Parser {
    // Parse vector constructor
    pub(in crate::lpscript) fn parse_vec_constructor(&mut self) -> Expr {
        let token = self.current().clone();
        let vec_kind = token.kind.clone();
        let start = token.span.start;
        self.advance();

        // Must be followed by '(' for constructor
        self.expect(TokenKind::LParen);
        let args = self.parse_args();
        let end = if matches!(self.current().kind, TokenKind::RParen) {
            let span = self.current().span;
            self.advance(); // consume ')'
            span.end
        } else {
            self.current().span.end
        };

        let kind = match vec_kind {
            TokenKind::Vec2 => ExprKind::Vec2Constructor(args),
            TokenKind::Vec3 => ExprKind::Vec3Constructor(args),
            TokenKind::Vec4 => ExprKind::Vec4Constructor(args),
            _ => unreachable!(),
        };

        Expr::new(kind, Span::new(start, end))
    }
}

