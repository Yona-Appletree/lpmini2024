/// Assignment expression parsing
use super::Parser;
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use alloc::boxed::Box;

impl Parser {
    // Assignment expression: lvalue = rvalue (right-associative)
    // This is needed for for-loop increments like: i = i + 1
    pub(in crate::lpscript::parser) fn parse_assignment_expr(&mut self) -> Expr {
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
}

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    #[ignore] // TODO: Fix assignment expression parser recursion
    fn test_parse_assignment_expression() {
        let mut lexer = Lexer::new("x = 5.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::Assign { .. }
        ));
    }

    #[test]
    #[ignore] // TODO: Fix chained assignment recursion
    fn test_parse_chained_assignment() {
        let mut lexer = Lexer::new("z = y = x = 7.0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::Assign { .. }
        ));
    }
}
