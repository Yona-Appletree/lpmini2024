/// Block statement parsing
use crate::compiler::ast::{StmtId, StmtKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;
use alloc::vec::Vec;


impl Parser {
    pub(in crate) fn parse_block(&mut self) -> Result<StmtId, ParseError> {
        self.enter_recursion()?;
        let start = self.current().span.start;
        self.advance(); // consume '{'

        let mut stmts = Vec::new();
        while !matches!(self.current().kind, TokenKind::RBrace | TokenKind::Eof) {
            stmts.push(self.parse_stmt()?);
        }

        let end = self.current().span.end;
        self.expect(TokenKind::RBrace);

        let result = self
            .pool
            .alloc_stmt(StmtKind::Block(stmts), Span::new(start, end))
            .map_err(|e| self.pool_error_to_parse_error(e));

        self.exit_recursion();
        result
    }
}
