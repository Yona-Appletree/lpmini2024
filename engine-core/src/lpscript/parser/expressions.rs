/// Expression parsing methods
use super::Parser;
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use alloc::boxed::Box;
use alloc::vec::Vec;

impl Parser {
    // Assignment expression: lvalue = rvalue (right-associative)
    // This is needed for for-loop increments like: i = i + 1
    pub(super) fn parse_assignment_expr(&mut self) -> Expr {
        let expr = self.ternary();

        // Check if this is an assignment
        if matches!(self.current().kind, TokenKind::Eq) {
            if let ExprKind::Variable(name) = &expr.kind {
                let start = expr.span.start;
                self.advance(); // consume '='
                let value = Box::new(self.parse_assignment_expr()); // right-associative
                let end = value.span.end;

                // Return assignment expression (which evaluates to the assigned value)
                return Expr::new(
                    ExprKind::Assign {
                        target: name.clone(),
                        value,
                    },
                    Span::new(start, end),
                );
            }
        }

        expr
    }

    // Ternary: condition ? true_expr : false_expr
    pub(super) fn ternary(&mut self) -> Expr {
        let mut expr = self.logical_or();

        if matches!(self.current().kind, TokenKind::Question) {
            let start = expr.span.start;
            self.advance(); // consume '?'
            let true_expr = Box::new(self.ternary());

            if matches!(self.current().kind, TokenKind::Colon) {
                self.advance(); // consume ':'
                let false_expr = Box::new(self.ternary());
                let end = false_expr.span.end;

                expr = Expr::new(
                    ExprKind::Ternary {
                        condition: Box::new(expr),
                        true_expr,
                        false_expr,
                    },
                    Span::new(start, end),
                );
            }
        }

        expr
    }

    // Logical OR: ||
    fn logical_or(&mut self) -> Expr {
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
    fn logical_and(&mut self) -> Expr {
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

    // Comparison: <, >, <=, >=, ==, !=
    fn comparison(&mut self) -> Expr {
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

    // Additive: + -
    fn additive(&mut self) -> Expr {
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
    fn multiplicative(&mut self) -> Expr {
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
    fn exponential(&mut self) -> Expr {
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

    // Postfix: swizzle (.xyzw, .rgba, .stpq)
    fn postfix(&mut self) -> Expr {
        let mut expr = self.primary();

        // Handle swizzle operations
        while matches!(self.current().kind, TokenKind::Dot) {
            let start = expr.span.start;
            self.advance(); // consume '.'

            // Read the swizzle components
            if let TokenKind::Ident(components) = &self.current().kind {
                let components = components.clone();
                let end = self.current().span.end;
                self.advance();

                expr = Expr::new(
                    ExprKind::Swizzle {
                        expr: Box::new(expr),
                        components,
                    },
                    Span::new(start, end),
                );
            } else {
                // Invalid swizzle, just break
                break;
            }
        }

        expr
    }

    // Primary: number, variable, function call, or parenthesized expression
    fn primary(&mut self) -> Expr {
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
            TokenKind::Vec2 | TokenKind::Vec3 | TokenKind::Vec4 => {
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

            TokenKind::Ident(name) => {
                let name = name.clone();
                let start = token.span.start;
                self.advance();

                if matches!(self.current().kind, TokenKind::LParen) {
                    // Function call
                    self.advance(); // consume '('
                    let args = self.parse_args();
                    let end = if matches!(self.current().kind, TokenKind::RParen) {
                        let span = self.current().span;
                        self.advance(); // consume ')'
                        span.end
                    } else {
                        self.current().span.end
                    };

                    let kind = ExprKind::Call { name, args };

                    Expr::new(kind, Span::new(start, end))
                } else {
                    // Variable
                    Expr::new(ExprKind::Variable(name), token.span)
                }
            }
            TokenKind::LParen => {
                self.advance(); // consume '('
                let expr = self.ternary();
                if matches!(self.current().kind, TokenKind::RParen) {
                    self.advance(); // consume ')'
                }
                expr
            }
            _ => {
                // Error fallback
                Expr::new(ExprKind::Number(0.0), token.span)
            }
        }
    }

    pub(super) fn parse_args(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();

        if matches!(self.current().kind, TokenKind::RParen) {
            return args;
        }

        loop {
            args.push(self.ternary());
            if matches!(self.current().kind, TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        
        args
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::ast::ExprKind;
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_number_literal() {
        let mut lexer = Lexer::new("42.5");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Number(n) if (n - 42.5).abs() < 0.01));
    }

    #[test]
    fn test_parse_int_literal() {
        let mut lexer = Lexer::new("42");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::IntNumber(42)));
    }

    #[test]
    fn test_parse_variable() {
        let mut lexer = Lexer::new("xNorm");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Variable(ref name) if name == "xNorm"));
    }

    #[test]
    fn test_parse_addition() {
        let mut lexer = Lexer::new("1.0 + 2.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Add(_, _)));
    }

    #[test]
    fn test_parse_subtraction() {
        let mut lexer = Lexer::new("5.0 - 2.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Sub(_, _)));
    }

    #[test]
    fn test_parse_multiplication() {
        let mut lexer = Lexer::new("3.0 * 4.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Mul(_, _)));
    }

    #[test]
    fn test_parse_division() {
        let mut lexer = Lexer::new("10.0 / 2.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Div(_, _)));
    }

    #[test]
    fn test_parse_operator_precedence() {
        let mut lexer = Lexer::new("1.0 + 2.0 * 3.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        // Should be Add(1.0, Mul(2.0, 3.0))
        if let ExprKind::Add(left, right) = expr.kind {
            assert!(matches!(left.kind, ExprKind::Number(_)));
            assert!(matches!(right.kind, ExprKind::Mul(_, _)));
        } else {
            panic!("Expected Add expression");
        }
    }

    #[test]
    fn test_parse_comparison() {
        let mut lexer = Lexer::new("x > 5.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Greater(_, _)));
    }

    #[test]
    fn test_parse_logical_and() {
        let mut lexer = Lexer::new("x > 0.5 && y < 0.5");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::And(_, _)));
    }

    #[test]
    fn test_parse_logical_or() {
        let mut lexer = Lexer::new("x < 0.25 || x > 0.75");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Or(_, _)));
    }

    #[test]
    fn test_parse_ternary() {
        let mut lexer = Lexer::new("x > 0.5 ? 1.0 : 0.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Ternary { .. }));
    }

    #[test]
    fn test_parse_function_call() {
        let mut lexer = Lexer::new("sin(time)");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        if let ExprKind::Call { name, args } = expr.kind {
            assert_eq!(name, "sin");
            assert_eq!(args.len(), 1);
        } else {
            panic!("Expected Call expression");
        }
    }

    #[test]
    fn test_parse_vec2_constructor() {
        let mut lexer = Lexer::new("vec2(1.0, 2.0)");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Vec2Constructor(_)));
    }

    #[test]
    fn test_parse_swizzle() {
        let mut lexer = Lexer::new("uv.x");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Swizzle { .. }));
    }

    #[test]
    fn test_parse_parenthesized_expression() {
        let mut lexer = Lexer::new("(1.0 + 2.0) * 3.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        // Should be Mul(Add(1.0, 2.0), 3.0)
        assert!(matches!(expr.kind, ExprKind::Mul(_, _)));
    }

    #[test]
    #[ignore] // TODO: Fix assignment expression parser recursion
    fn test_parse_assignment_expression() {
        let mut lexer = Lexer::new("x = 5.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Assign { .. }));
    }

    #[test]
    #[ignore] // TODO: Fix chained assignment recursion
    fn test_parse_chained_assignment() {
        let mut lexer = Lexer::new("z = y = x = 7.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        
        assert!(matches!(expr.kind, ExprKind::Assign { .. }));
    }
}
