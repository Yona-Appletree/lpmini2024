/// Statement parsing
use super::Parser;
use crate::lpscript::ast::Stmt;
use crate::lpscript::lexer::TokenKind;

mod assign;
mod block;
mod expr_stmt;
mod for_loop;
mod if_stmt;
mod return_stmt;
mod var_decl;
mod while_loop;

impl Parser {
    /// Parse a statement - exposed to submodules
    pub(in crate::lpscript::parser) fn parse_stmt(&mut self) -> Stmt {
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
                    self.parse_assignment_stmt(name, start)
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
}
