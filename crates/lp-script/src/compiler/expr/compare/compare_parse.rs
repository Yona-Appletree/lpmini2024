use lp_pool::LpBox;

/// Comparison operator parsing (<, >, <=, >=, ==, !=)
use crate::compiler::ast::{Expr, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    // Equality: == !=
    pub(crate) fn equality(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let mut expr = self.relational()?;

        loop {
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::EqEq => {
                    self.advance();
                    let right = self.relational()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Eq(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                        Span::new(start, end),
                    );
                }
                TokenKind::NotEq => {
                    self.advance();
                    let right = self.relational()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::NotEq(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                        Span::new(start, end),
                    );
                }
                _ => break,
            }
        }

        self.exit_recursion();
        Ok(expr)
    }

    // Relational: < > <= >=
    pub(crate) fn relational(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let mut expr = self.shift()?;

        loop {
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::Less => {
                    self.advance();
                    let right = self.shift()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Less(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                        Span::new(start, end),
                    );
                }
                TokenKind::Greater => {
                    self.advance();
                    let right = self.shift()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Greater(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                        Span::new(start, end),
                    );
                }
                TokenKind::LessEq => {
                    self.advance();
                    let right = self.shift()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::LessEq(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                        Span::new(start, end),
                    );
                }
                TokenKind::GreaterEq => {
                    self.advance();
                    let right = self.shift()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::GreaterEq(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                        Span::new(start, end),
                    );
                }
                _ => break,
            }
        }

        self.exit_recursion();
        Ok(expr)
    }
}
