/// Assignment statement parsing
use super::Parser;
use crate::lpscript::ast::{Stmt, StmtKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;

impl Parser {
    pub(super) fn parse_assignment_stmt(&mut self, name: alloc::string::String, start: usize) -> Stmt {
        // Already consumed the identifier, now consume '='
        self.advance(); // consume '='
        let value = self.ternary();
        self.consume_semicolon();
        let end = self.current().span.end;
        Stmt::new(
            StmtKind::Assignment { name, value },
            Span::new(start, end),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_assignment_statement() {
        let mut lexer = Lexer::new("float x = 1.0; x = 2.0;");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 2);
        assert!(matches!(program.stmts[0].kind, crate::lpscript::ast::StmtKind::VarDecl { .. }));
        assert!(matches!(program.stmts[1].kind, crate::lpscript::ast::StmtKind::Assignment { .. }));
    }
}

