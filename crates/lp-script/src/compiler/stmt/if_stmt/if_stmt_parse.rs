/// If statement parsing
use crate::compiler::ast::{StmtId, StmtKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    pub(crate) fn parse_if_stmt(&mut self) -> Result<StmtId, ParseError> {
        self.enter_recursion()?;
        let start = self.current().span.start;
        self.advance(); // consume 'if'

        self.expect(TokenKind::LParen);
        let condition = self.ternary()?;
        self.expect(TokenKind::RParen);

        let then_stmt = self.parse_stmt()?;

        let else_stmt = if matches!(self.current().kind, TokenKind::Else) {
            self.advance(); // consume 'else'
            Some(self.parse_stmt()?)
        } else {
            None
        };

        let end = else_stmt
            .map(|s| self.pool.stmt(s).span.end)
            .unwrap_or_else(|| self.pool.stmt(then_stmt).span.end);

        let result = self
            .pool
            .alloc_stmt(
                StmtKind::If {
                    condition,
                    then_stmt,
                    else_stmt,
                },
                Span::new(start, end),
            )
            .map_err(|e| self.pool_error_to_parse_error(e));

        self.exit_recursion();
        result
    }
}
