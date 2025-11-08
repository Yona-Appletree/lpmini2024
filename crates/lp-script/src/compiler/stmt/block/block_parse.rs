/// Block statement parsing
use crate::compiler::ast::{Stmt, StmtKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;
use alloc::vec::Vec;

impl Parser {
    pub(crate) fn parse_block(&mut self) -> Result<Stmt, ParseError> {
        self.enter_recursion()?;
        let start = self.current().span.start;
        self.advance(); // consume '{'

        let mut stmts = Vec::new();
        while !matches!(self.current().kind, TokenKind::RBrace | TokenKind::Eof) {
            stmts.push(self.parse_stmt()?);
        }

        let end = self.current().span.end;
        self.expect(TokenKind::RBrace);

        let result = Ok(Stmt::new(StmtKind::Block(stmts), Span::new(start, end)));

        self.exit_recursion();
        result
    }
}
