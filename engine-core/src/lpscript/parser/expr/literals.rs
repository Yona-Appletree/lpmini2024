/// Literal parsing (numbers, parenthesized expressions)
use super::Parser;
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::lexer::TokenKind;

impl Parser {
    // Primary: number, variable, function call, constructor, or parenthesized expression
    pub(super) fn primary(&mut self) -> Expr {
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

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_number_literal() {
        let mut lexer = Lexer::new("42.5");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(
            matches!(expr.kind, crate::lpscript::ast::ExprKind::Number(n) if (n - 42.5).abs() < 0.01)
        );
    }

    #[test]
    fn test_parse_int_literal() {
        let mut lexer = Lexer::new("42");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::IntNumber(42)
        ));
    }
}
