/// Statement parsing methods
use super::Parser;
use crate::lpscript::ast::{Stmt, StmtKind};
use crate::lpscript::error::{Span, Type};
use crate::lpscript::lexer::TokenKind;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

impl Parser {
    /// Parse a statement
    pub(super) fn parse_stmt(&mut self) -> Stmt {
        let start = self.current().span.start;
        
        let stmt = match &self.current().kind {
            TokenKind::If => self.parse_if_stmt(),
            TokenKind::While => self.parse_while_stmt(),
            TokenKind::For => self.parse_for_stmt(),
            TokenKind::Return => self.parse_return_stmt(),
            TokenKind::LBrace => self.parse_block(),
            TokenKind::Float | TokenKind::Int => self.parse_var_decl(),
            TokenKind::Ident(name) => {
                // Could be assignment or expression statement
                let name = name.clone();
                self.advance();
                
                if matches!(self.current().kind, TokenKind::Eq) {
                    // Assignment
                    self.advance(); // consume '='
                    let value = self.ternary();
                    self.consume_semicolon();
                    let end = self.current().span.end;
                    Stmt::new(
                        StmtKind::Assignment { name, value },
                        Span::new(start, end),
                    )
                } else {
                    // Put token back and parse as expression statement
                    self.pos -= 1;
                    self.parse_expr_stmt()
                }
            }
            _ => self.parse_expr_stmt(),
        };
        
        stmt
    }
    
    fn parse_return_stmt(&mut self) -> Stmt {
        let start = self.current().span.start;
        self.advance(); // consume 'return'
        
        let expr = self.ternary();
        self.consume_semicolon();
        let end = self.current().span.end;
        
        Stmt::new(StmtKind::Return(expr), Span::new(start, end))
    }
    
    fn parse_var_decl(&mut self) -> Stmt {
        let stmt = self.parse_var_decl_no_semicolon();
        self.consume_semicolon();
        stmt
    }
    
    pub(super) fn parse_var_decl_no_semicolon(&mut self) -> Stmt {
        let start = self.current().span.start;
        
        // Parse type
        let ty = self.parse_type();
        
        // Parse name
        let name = if let TokenKind::Ident(n) = &self.current().kind {
            let name = n.clone();
            self.advance();
            name
        } else {
            String::from("error")
        };
        
        // Parse optional initializer
        let init = if matches!(self.current().kind, TokenKind::Eq) {
            self.advance(); // consume '='
            Some(self.ternary())
        } else {
            None
        };
        
        let end = self.current().span.end;
        
        Stmt::new(
            StmtKind::VarDecl { ty, name, init },
            Span::new(start, end),
        )
    }
    
    pub(super) fn parse_block(&mut self) -> Stmt {
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
    
    fn parse_expr_stmt(&mut self) -> Stmt {
        let expr = self.ternary();
        let span = expr.span;
        self.consume_semicolon();
        Stmt::new(StmtKind::Expr(expr), span)
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::ast::StmtKind;
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_variable_declaration() {
        let mut lexer = Lexer::new("float x = 5.0;");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(program.stmts[0].kind, StmtKind::VarDecl { .. }));
    }

    #[test]
    fn test_parse_assignment_statement() {
        let mut lexer = Lexer::new("float x = 1.0; x = 2.0;");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 2);
        assert!(matches!(program.stmts[0].kind, StmtKind::VarDecl { .. }));
        assert!(matches!(program.stmts[1].kind, StmtKind::Assignment { .. }));
    }

    #[test]
    fn test_parse_return_statement() {
        let mut lexer = Lexer::new("return 5.0;");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(program.stmts[0].kind, StmtKind::Return(_)));
    }

    #[test]
    fn test_parse_expression_statement() {
        let mut lexer = Lexer::new("sin(time);");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(program.stmts[0].kind, StmtKind::Expr(_)));
    }

    #[test]
    fn test_parse_block_structure() {
        let mut lexer = Lexer::new("{ float x = 1.0; }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(program.stmts[0].kind, StmtKind::Block(_)));
    }

    #[test]
    fn test_parse_multiple_statements() {
        let mut lexer = Lexer::new("float a = 1.0; float b = 2.0; return a + b;");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 3);
    }
}

