/// Block statement parsing
use crate::lpscript::ast::{Stmt, StmtKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use alloc::vec::Vec;

impl Parser {
    pub(in crate::lpscript) fn parse_block(&mut self) -> Stmt {
        let start = self.current().span.start;
        self.advance(); // consume '{'
        
        let mut stmts = Vec::new();
        while !matches!(self.current().kind, TokenKind::RBrace | TokenKind::Eof) {
            stmts.push(self.parse_stmt());
        }
        
        let end = self.current().span.end;
        self.expect(TokenKind::RBrace);
        
        Stmt::new(StmtKind::Block(stmts), Span::new(start, end))
    }
}
