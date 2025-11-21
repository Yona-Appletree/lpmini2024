use alloc::vec::Vec;

/// Block statement parsing
use crate::lp_script::compiler::ast::{Stmt, StmtKind};
use crate::lp_script::compiler::error::ParseError;
use crate::lp_script::compiler::lexer::TokenKind;
use crate::lp_script::compiler::parser::Parser;
use crate::lp_script::shared::Span;

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
