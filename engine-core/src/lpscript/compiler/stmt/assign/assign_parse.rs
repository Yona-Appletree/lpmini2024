/// Assignment statement parsing
use crate::lpscript::ast::{Stmt, StmtKind};
use crate::lpscript::error::Span;
use crate::lpscript::compiler::parser::Parser;

impl Parser {
    pub(in crate::lpscript) fn parse_assignment_stmt(&mut self, name: alloc::string::String, start: usize) -> Stmt {
        // Already consumed the identifier, now consume '='
        self.advance(); // consume '='
        let value = self.parse_assignment_expr();
        self.consume_semicolon();
        let end = self.current().span.end;
        Stmt::new(
            StmtKind::Assignment { name, value },
            Span::new(start, end),
        )
    }
}

