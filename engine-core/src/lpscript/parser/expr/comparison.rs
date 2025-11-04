/// Comparison operator parsing (<, >, <=, >=, ==, !=)
use super::Parser;
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use alloc::boxed::Box;

impl Parser {
    // Comparison: <, >, <=, >=, ==, !=
    pub(super) fn comparison(&mut self) -> Expr {
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

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_comparison() {
        let mut lexer = Lexer::new("x > 5.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::Greater(_, _)
        ));
    }
}
