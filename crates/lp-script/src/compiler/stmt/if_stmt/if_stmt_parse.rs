use alloc::boxed::Box;

/// If statement parsing
use crate::compiler::ast::{Stmt, StmtKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    pub(crate) fn parse_if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.enter_recursion()?;
        let start = self.current().span.start;
        self.advance(); // consume 'if'

        self.expect(TokenKind::LParen);
        let condition = self.ternary()?;
        self.expect(TokenKind::RParen);

        let then_stmt = self.parse_stmt()?;

        let else_stmt = if matches!(self.current().kind, TokenKind::Else) {
            self.advance(); // consume 'else'
            let stmt = self.parse_stmt()?;
            Some(Box::new(stmt))
        } else {
            None
        };

        let end = else_stmt
            .as_ref()
            .map(|s| s.span.end)
            .unwrap_or(then_stmt.span.end);

        let result = Ok(Stmt::new(
            StmtKind::If {
                condition,
                then_stmt: Box::new(then_stmt),
                else_stmt,
            },
            Span::new(start, end),
        ));

        self.exit_recursion();
        result
    }
}
