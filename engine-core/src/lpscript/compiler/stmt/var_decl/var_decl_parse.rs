/// Variable declaration parsing
use crate::lpscript::ast::{Stmt, StmtKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use alloc::string::String;

impl Parser {
    pub(in crate::lpscript) fn parse_var_decl(&mut self) -> Stmt {
        let stmt = self.parse_var_decl_no_semicolon();
        self.consume_semicolon();
        stmt
    }

    pub(in crate::lpscript) fn parse_var_decl_no_semicolon(&mut self) -> Stmt {
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
            Some(self.parse_assignment_expr())
        } else {
            None
        };

        let end = self.current().span.end;

        Stmt::new(StmtKind::VarDecl { ty, name, init }, Span::new(start, end))
    }
}

