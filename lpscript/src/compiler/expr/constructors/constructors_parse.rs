/// Vector constructor parsing
use crate::compiler::ast::{ExprId, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;


impl Parser {
    // Parse vector constructor
    pub(in crate) fn parse_vec_constructor(&mut self) -> Result<ExprId, ParseError> {
        let token = self.current().clone();
        let vec_kind = token.kind.clone();
        let start = token.span.start;
        self.advance();

        // Must be followed by '(' for constructor
        self.expect(TokenKind::LParen);
        let args = self.parse_args()?;
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

        self.pool
            .alloc_expr(kind, Span::new(start, end))
            .map_err(|e| self.pool_error_to_parse_error(e))
    }
}
