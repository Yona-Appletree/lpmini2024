use alloc::boxed::Box;

/// While loop parsing
use crate::lp_script::compiler::ast::{Stmt, StmtKind};
use crate::lp_script::compiler::error::ParseError;
use crate::lp_script::compiler::lexer::TokenKind;
use crate::lp_script::compiler::parser::Parser;
use crate::lp_script::shared::Span;

impl Parser {
    pub(crate) fn parse_while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.enter_recursion()?;
        let start = self.current().span.start;
        self.advance(); // consume 'while'

        self.expect(TokenKind::LParen);
        let condition = self.ternary()?;
        self.expect(TokenKind::RParen);

        let body = self.parse_stmt()?;
        let end = body.span.end;

        let result = Ok(Stmt::new(
            StmtKind::While {
                condition,
                body: Box::new(body),
            },
            Span::new(start, end),
        ));

        self.exit_recursion();
        result
    }
}
