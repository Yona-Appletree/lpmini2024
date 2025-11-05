/// Assignment statement parsing
use crate::lpscript::compiler::ast::{Expr, ExprKind, Stmt, StmtKind};
use crate::lpscript::compiler::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::shared::Span;
use alloc::boxed::Box;

impl Parser {
    pub(in crate::lpscript) fn parse_assignment_stmt(
        &mut self,
        name: alloc::string::String,
        start: usize,
    ) -> Stmt {
        // Already consumed the identifier, check for compound assignment or simple assignment
        let op_token = self.current().kind.clone();

        match op_token {
            // Simple assignment
            TokenKind::Eq => {
                self.advance(); // consume '='
                let value = self.parse_assignment_expr();
                self.consume_semicolon();
                let end = self.current().span.end;
                Stmt::new(StmtKind::Assignment { name, value }, Span::new(start, end))
            }

            // Compound assignments - desugar to regular assignment with binary operation
            // x += 5 becomes x = x + 5
            TokenKind::PlusEq => {
                self.advance(); // consume '+='
                let rhs = self.ternary(); // Use ternary, not assignment_expr (prevent infinite recursion)
                let var_ref = Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                let value = Expr::new(
                    ExprKind::Add(Box::new(var_ref), Box::new(rhs)),
                    Span::new(start, self.current().span.end),
                );
                self.consume_semicolon();
                let end = self.current().span.end;
                Stmt::new(StmtKind::Assignment { name, value }, Span::new(start, end))
            }

            TokenKind::MinusEq => {
                self.advance();
                let rhs = self.ternary();
                let var_ref = Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                let value = Expr::new(
                    ExprKind::Sub(Box::new(var_ref), Box::new(rhs)),
                    Span::new(start, self.current().span.end),
                );
                self.consume_semicolon();
                let end = self.current().span.end;
                Stmt::new(StmtKind::Assignment { name, value }, Span::new(start, end))
            }

            TokenKind::StarEq => {
                self.advance();
                let rhs = self.ternary();
                let var_ref = Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                let value = Expr::new(
                    ExprKind::Mul(Box::new(var_ref), Box::new(rhs)),
                    Span::new(start, self.current().span.end),
                );
                self.consume_semicolon();
                let end = self.current().span.end;
                Stmt::new(StmtKind::Assignment { name, value }, Span::new(start, end))
            }

            TokenKind::SlashEq => {
                self.advance();
                let rhs = self.ternary();
                let var_ref = Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                let value = Expr::new(
                    ExprKind::Div(Box::new(var_ref), Box::new(rhs)),
                    Span::new(start, self.current().span.end),
                );
                self.consume_semicolon();
                let end = self.current().span.end;
                Stmt::new(StmtKind::Assignment { name, value }, Span::new(start, end))
            }

            TokenKind::PercentEq => {
                self.advance();
                let rhs = self.ternary();
                let var_ref = Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                let value = Expr::new(
                    ExprKind::Mod(Box::new(var_ref), Box::new(rhs)),
                    Span::new(start, self.current().span.end),
                );
                self.consume_semicolon();
                let end = self.current().span.end;
                Stmt::new(StmtKind::Assignment { name, value }, Span::new(start, end))
            }

            TokenKind::AmpersandEq => {
                self.advance();
                let rhs = self.ternary();
                let var_ref = Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                let value = Expr::new(
                    ExprKind::BitwiseAnd(Box::new(var_ref), Box::new(rhs)),
                    Span::new(start, self.current().span.end),
                );
                self.consume_semicolon();
                let end = self.current().span.end;
                Stmt::new(StmtKind::Assignment { name, value }, Span::new(start, end))
            }

            TokenKind::PipeEq => {
                self.advance();
                let rhs = self.ternary();
                let var_ref = Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                let value = Expr::new(
                    ExprKind::BitwiseOr(Box::new(var_ref), Box::new(rhs)),
                    Span::new(start, self.current().span.end),
                );
                self.consume_semicolon();
                let end = self.current().span.end;
                Stmt::new(StmtKind::Assignment { name, value }, Span::new(start, end))
            }

            TokenKind::CaretEq => {
                self.advance();
                let rhs = self.ternary();
                let var_ref = Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                let value = Expr::new(
                    ExprKind::BitwiseXor(Box::new(var_ref), Box::new(rhs)),
                    Span::new(start, self.current().span.end),
                );
                self.consume_semicolon();
                let end = self.current().span.end;
                Stmt::new(StmtKind::Assignment { name, value }, Span::new(start, end))
            }

            TokenKind::LShiftEq => {
                self.advance();
                let rhs = self.ternary();
                let var_ref = Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                let value = Expr::new(
                    ExprKind::LeftShift(Box::new(var_ref), Box::new(rhs)),
                    Span::new(start, self.current().span.end),
                );
                self.consume_semicolon();
                let end = self.current().span.end;
                Stmt::new(StmtKind::Assignment { name, value }, Span::new(start, end))
            }

            TokenKind::RShiftEq => {
                self.advance();
                let rhs = self.ternary();
                let var_ref = Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                let value = Expr::new(
                    ExprKind::RightShift(Box::new(var_ref), Box::new(rhs)),
                    Span::new(start, self.current().span.end),
                );
                self.consume_semicolon();
                let end = self.current().span.end;
                Stmt::new(StmtKind::Assignment { name, value }, Span::new(start, end))
            }

            // If none of the above, fall back to simple assignment (for error recovery)
            _ => {
                self.advance();
                let value = self.parse_assignment_expr();
                self.consume_semicolon();
                let end = self.current().span.end;
                Stmt::new(StmtKind::Assignment { name, value }, Span::new(start, end))
            }
        }
    }
}
