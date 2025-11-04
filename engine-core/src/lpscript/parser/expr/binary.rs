/// Binary operator parsing (+, -, *, /, %, ^)
use super::Parser;
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use alloc::boxed::Box;

impl Parser {
    // Additive: + -
    pub(super) fn additive(&mut self) -> Expr {
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
    pub(super) fn multiplicative(&mut self) -> Expr {
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
    pub(super) fn exponential(&mut self) -> Expr {
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

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_addition() {
        let mut lexer = Lexer::new("1.0 + 2.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::Add(_, _)
        ));
    }

    #[test]
    fn test_parse_subtraction() {
        let mut lexer = Lexer::new("5.0 - 2.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::Sub(_, _)
        ));
    }

    #[test]
    fn test_parse_multiplication() {
        let mut lexer = Lexer::new("3.0 * 4.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::Mul(_, _)
        ));
    }

    #[test]
    fn test_parse_division() {
        let mut lexer = Lexer::new("10.0 / 2.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::Div(_, _)
        ));
    }

    #[test]
    fn test_parse_operator_precedence() {
        let mut lexer = Lexer::new("1.0 + 2.0 * 3.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        // Should be Add(1.0, Mul(2.0, 3.0))
        if let crate::lpscript::ast::ExprKind::Add(left, right) = expr.kind {
            assert!(matches!(
                left.kind,
                crate::lpscript::ast::ExprKind::Number(_)
            ));
            assert!(matches!(
                right.kind,
                crate::lpscript::ast::ExprKind::Mul(_, _)
            ));
        } else {
            panic!("Expected Add expression");
        }
    }

    #[test]
    fn test_parse_parenthesized_expression() {
        let mut lexer = Lexer::new("(1.0 + 2.0) * 3.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        // Should be Mul(Add(1.0, 2.0), 3.0)
        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::Mul(_, _)
        ));
    }
}
