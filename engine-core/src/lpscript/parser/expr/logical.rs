/// Logical operator parsing (&&, ||)
use super::Parser;
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use alloc::boxed::Box;

impl Parser {
    // Logical OR: ||
    pub(super) fn logical_or(&mut self) -> Expr {
        let mut expr = self.logical_and();

        while matches!(self.current().kind, TokenKind::Or) {
            let start = expr.span.start;
            self.advance();
            let right = self.logical_and();
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::Or(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }

        expr
    }

    // Logical AND: &&
    pub(super) fn logical_and(&mut self) -> Expr {
        let mut expr = self.comparison();

        while matches!(self.current().kind, TokenKind::And) {
            let start = expr.span.start;
            self.advance();
            let right = self.comparison();
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::And(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }

        expr
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_logical_and() {
        let mut lexer = Lexer::new("x > 0.5 && y < 0.5");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::And(_, _)
        ));
    }

    #[test]
    fn test_parse_logical_or() {
        let mut lexer = Lexer::new("x < 0.25 || x > 0.75");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::Or(_, _)
        ));
    }
}
