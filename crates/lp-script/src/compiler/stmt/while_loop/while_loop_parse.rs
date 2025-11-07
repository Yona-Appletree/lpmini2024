/// While loop parsing
use crate::compiler::ast::{StmtId, StmtKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    pub(crate) fn parse_while_stmt(&mut self) -> Result<StmtId, ParseError> {
        self.enter_recursion()?;
        let start = self.current().span.start;
        self.advance(); // consume 'while'

        self.expect(TokenKind::LParen);
        let condition = self.ternary()?;
        self.expect(TokenKind::RParen);

        let body = self.parse_stmt()?;
        let end = self.pool.stmt(body).span.end;

        let result = self
            .pool
            .alloc_stmt(StmtKind::While { condition, body }, Span::new(start, end))
            .map_err(|e| self.pool_error_to_parse_error(e));

        self.exit_recursion();
        result
    }
}
