/// Variable parsing
use crate::compiler::ast::{ExprId, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;


impl Parser {
    // Parse identifier (variable or function call)
    pub(in crate) fn parse_ident(&mut self) -> Result<ExprId, ParseError> {
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
                self.pool
                    .alloc_expr(ExprKind::Variable(name), token.span)
                    .map_err(|e| self.pool_error_to_parse_error(e))
            }
        } else {
            // Error fallback
            self.pool
                .alloc_expr(ExprKind::Number(0.0), token.span)
                .map_err(|e| self.pool_error_to_parse_error(e))
        }
    }
}
