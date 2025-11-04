/// Function call parsing
use super::Parser;
use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use alloc::vec::Vec;

impl Parser {
    // Parse function call
    pub(super) fn parse_function_call(&mut self) -> Expr {
        let token = self.current().clone();

        if let TokenKind::Ident(name) = &token.kind {
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
                // Not a function call, return variable
                Expr::new(ExprKind::Variable(name), token.span)
            }
        } else {
            // Error fallback
            Expr::new(ExprKind::Number(0.0), token.span)
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
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_function_call() {
        let mut lexer = Lexer::new("sin(time)");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        if let crate::lpscript::ast::ExprKind::Call { name, args } = expr.kind {
            assert_eq!(name, "sin");
            assert_eq!(args.len(), 1);
        } else {
            panic!("Expected Call expression");
        }
    }
}
