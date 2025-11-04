/// Swizzle (postfix) operator parsing
use super::Parser;
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use alloc::boxed::Box;

impl Parser {
    // Postfix: swizzle (.xyzw, .rgba, .stpq)
    pub(super) fn postfix(&mut self) -> Expr {
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
}

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_swizzle() {
        let mut lexer = Lexer::new("uv.x");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(matches!(
            expr.kind,
            crate::lpscript::ast::ExprKind::Swizzle { .. }
        ));
    }
}
