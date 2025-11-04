/// Vector constructor parsing
use super::Parser;
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;

impl Parser {
    // Parse vector constructor
    pub(super) fn parse_vec_constructor(&mut self) -> Expr {
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

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_vec2_constructor() {
        let mut lexer = Lexer::new("vec2(1.0, 2.0)");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::Vec2Constructor(_)
        ));
    }
}
