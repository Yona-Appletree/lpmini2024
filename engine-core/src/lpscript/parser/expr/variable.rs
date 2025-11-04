/// Variable parsing
use super::Parser;
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::lexer::TokenKind;

impl Parser {
    // Parse identifier (variable or function call)
    pub(super) fn parse_ident(&mut self) -> Expr {
        let token = self.current().clone();

        if let TokenKind::Ident(name) = &token.kind {
            let name = name.clone();
            let start = token.span.start;
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

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_variable() {
        let mut lexer = Lexer::new("xNorm");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        assert!(
            matches!(expr.kind, crate::lpscript::ast::ExprKind::Variable(ref name) if name == "xNorm")
        );
    }
}
