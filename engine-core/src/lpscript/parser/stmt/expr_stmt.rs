/// Expression statement parsing
use super::Parser;
use crate::lpscript::ast::{Stmt, StmtKind};

impl Parser {
    pub(in crate::lpscript::parser) fn parse_expr_stmt(&mut self) -> Stmt {
        let expr = self.ternary();
        let span = expr.span;
        self.consume_semicolon();
        Stmt::new(StmtKind::Expr(expr), span)
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_expression_statement() {
        let mut lexer = Lexer::new("sin(time);");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(program.stmts[0].kind, crate::lpscript::ast::StmtKind::Expr(_)));
    }
}

