/// Return statement parsing
use super::Parser;
use crate::lpscript::ast::{Stmt, StmtKind};
use crate::lpscript::error::Span;

impl Parser {
    pub(in crate::lpscript::parser) fn parse_return_stmt(&mut self) -> Stmt {
        let start = self.current().span.start;
        self.advance(); // consume 'return'
        
        let expr = self.ternary();
        self.consume_semicolon();
        let end = self.current().span.end;
        
        Stmt::new(StmtKind::Return(expr), Span::new(start, end))
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_return_statement() {
        let mut lexer = Lexer::new("return 5.0;");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(program.stmts[0].kind, crate::lpscript::ast::StmtKind::Return(_)));
    }
}

