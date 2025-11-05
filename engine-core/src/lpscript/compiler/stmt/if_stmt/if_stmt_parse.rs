/// If statement parsing
use crate::lpscript::compiler::ast::{Stmt, StmtKind};
use crate::lpscript::compiler::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::error::Span;
use alloc::boxed::Box;


impl Parser {
    pub(in crate::lpscript) fn parse_if_stmt(&mut self) -> Stmt {
        let start = self.current().span.start;
        self.advance(); // consume 'if'

        self.expect(TokenKind::LParen);
        let condition = self.ternary();
        self.expect(TokenKind::RParen);

        let then_stmt = Box::new(self.parse_stmt());

        let else_stmt = if matches!(self.current().kind, TokenKind::Else) {
            self.advance(); // consume 'else'
            Some(Box::new(self.parse_stmt()))
        } else {
            None
        };

        let end = else_stmt
            .as_ref()
            .map(|s| s.span.end)
            .unwrap_or(then_stmt.span.end);

        Stmt::new(
            StmtKind::If {
                condition,
                then_stmt,
                else_stmt,
            },
            Span::new(start, end),
        )
    }
}
