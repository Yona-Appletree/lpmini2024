/// Variable declaration parsing
use crate::compiler::ast::{StmtId, StmtKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;
use alloc::string::String;

impl Parser {
    pub(crate) fn parse_var_decl(&mut self) -> Result<StmtId, ParseError> {
        let stmt_id = self.parse_var_decl_no_semicolon()?;
        self.consume_semicolon();
        Ok(stmt_id)
    }

    pub(crate) fn parse_var_decl_no_semicolon(&mut self) -> Result<StmtId, ParseError> {
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

        let result = self
            .pool
            .alloc_stmt(StmtKind::VarDecl { ty, name, init }, Span::new(start, end))
            .map_err(|e| self.pool_error_to_parse_error(e));

        self.exit_recursion();
        result
    }
}
