/// Variable declaration parsing
use super::Parser;
use crate::lpscript::ast::{Stmt, StmtKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use alloc::string::String;

impl Parser {
    pub(in crate::lpscript::parser) fn parse_var_decl(&mut self) -> Stmt {
        let stmt = self.parse_var_decl_no_semicolon();
        self.consume_semicolon();
        stmt
    }

    pub(in crate::lpscript::parser) fn parse_var_decl_no_semicolon(&mut self) -> Stmt {
        let start = self.current().span.start;

        // Parse type
        let ty = self.parse_type();

        // Parse name
        let name = if let TokenKind::Ident(n) = &self.current().kind {
            let name = n.clone();
            self.advance();
            name
        } else {
            String::from("error")
        };

        // Parse optional initializer
        let init = if matches!(self.current().kind, TokenKind::Eq) {
            self.advance(); // consume '='
            Some(self.ternary())
        } else {
            None
        };

        let end = self.current().span.end;

        Stmt::new(StmtKind::VarDecl { ty, name, init }, Span::new(start, end))
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_variable_declaration() {
        let mut lexer = Lexer::new("float x = 5.0;");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();

        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(
            program.stmts[0].kind,
            crate::lpscript::ast::StmtKind::VarDecl { .. }
        ));
    }
}
