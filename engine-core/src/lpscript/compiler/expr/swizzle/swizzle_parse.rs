/// Swizzle (postfix) operator parsing
use crate::lpscript::compiler::ast::{Expr, ExprKind};
use crate::lpscript::compiler::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::shared::Span;
use alloc::boxed::Box;


impl Parser {
    // Postfix: swizzle (.xyzw, .rgba, .stpq)
    pub(in crate::lpscript) fn postfix(&mut self) -> Expr {
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
