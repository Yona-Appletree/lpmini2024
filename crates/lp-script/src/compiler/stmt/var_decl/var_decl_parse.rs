use alloc::string::String;

/// Variable declaration parsing
use crate::compiler::ast::{Stmt, StmtKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    pub(crate) fn parse_var_decl(&mut self) -> Result<Stmt, ParseError> {
        let stmt = self.parse_var_decl_no_semicolon()?;
        self.consume_semicolon();
        Ok(stmt)
    }

    pub(crate) fn parse_var_decl_no_semicolon(&mut self) -> Result<Stmt, ParseError> {
        self.enter_recursion()?;
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
            Some(self.parse_assignment_expr()?)
        } else {
            None
        };

        let end = self.current().span.end;

        let result = Ok(Stmt::new(
            StmtKind::VarDecl { ty, name, init },
            Span::new(start, end),
        ));

        self.exit_recursion();
        result
    }
}
