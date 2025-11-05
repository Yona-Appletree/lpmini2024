/// While loop parsing
use crate::lpscript::compiler::ast::{Stmt, StmtKind};
use crate::lpscript::compiler::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::shared::Span;
use alloc::boxed::Box;


impl Parser {
    pub(in crate::lpscript) fn parse_while_stmt(&mut self) -> Stmt {
        let start = self.current().span.start;
        self.advance(); // consume 'while'

        self.expect(TokenKind::LParen);
        let condition = self.ternary();
        self.expect(TokenKind::RParen);

        let body = Box::new(self.parse_stmt());
        let end = body.span.end;

        Stmt::new(StmtKind::While { condition, body }, Span::new(start, end))
    }
}
