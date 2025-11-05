/// Variable parsing
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;

impl Parser {
    // Parse identifier (variable or function call)
    pub(in crate::lpscript) fn parse_ident(&mut self) -> Expr {
        let token = self.current().clone();

        if let TokenKind::Ident(name) = &token.kind {
            let name = name.clone();
            self.advance();

            if matches!(self.current().kind, TokenKind::LParen) {
                // Function call - handled in call.rs
                self.pos -= 1; // Put token back
                self.parse_function_call()
            } else {
                // Variable
                Expr::new(ExprKind::Variable(name), token.span)
            }
        } else {
            // Error fallback
            Expr::new(ExprKind::Number(0.0), token.span)
        }
    }
}

