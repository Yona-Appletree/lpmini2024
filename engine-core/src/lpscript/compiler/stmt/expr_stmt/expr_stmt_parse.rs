/// Expression statement parsing
use crate::lpscript::ast::{Stmt, StmtKind};
use crate::lpscript::compiler::parser::Parser;

impl Parser {
    pub(in crate::lpscript) fn parse_expr_stmt(&mut self) -> Stmt {
        let expr = self.ternary();
        let span = expr.span;
        self.consume_semicolon();
        Stmt::new(StmtKind::Expr(expr), span)
    }
}
