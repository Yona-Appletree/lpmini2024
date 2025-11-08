/// Variable parsing
use crate::compiler::ast::{Expr, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;

impl Parser {
    // Parse identifier (variable or function call)
    pub(crate) fn parse_ident(&mut self) -> Result<Expr, ParseError> {
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
                Ok(Expr::new(ExprKind::Variable(name), token.span))
            }
        } else {
            // Error fallback
            Ok(Expr::new(ExprKind::Number(0.0), token.span))
        }
    }
}
