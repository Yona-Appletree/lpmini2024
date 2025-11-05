/// Assignment expression parsing
use crate::lpscript::compiler::ast::{Expr, ExprKind};
use crate::lpscript::compiler::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::shared::Span;
use alloc::boxed::Box;


impl Parser {
    // Assignment expression: lvalue = rvalue (right-associative)
    // This is needed for for-loop increments like: i = i + 1
    pub(in crate::lpscript) fn parse_assignment_expr(&mut self) -> Expr {
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
