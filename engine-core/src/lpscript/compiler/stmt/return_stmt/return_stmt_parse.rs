/// Return statement parsing
use crate::lpscript::compiler::ast::{Stmt, StmtKind};
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::error::Span;


impl Parser {
    pub(in crate::lpscript) fn parse_return_stmt(&mut self) -> Stmt {
        let start = self.current().span.start;
        self.advance(); // consume 'return'

        let expr = self.ternary();
        self.consume_semicolon();
        let end = self.current().span.end;

        Stmt::new(StmtKind::Return(expr), Span::new(start, end))
    }
}
