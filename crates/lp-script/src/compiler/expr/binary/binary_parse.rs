/// Binary operator parsing (+, -, *, /, %, ^)
use crate::compiler::ast::{Expr, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;
use lp_pool::LpBox;

impl Parser {
    // Additive: + -
    pub(crate) fn additive(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let mut expr = self.multiplicative()?;

        loop {
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::Plus => {
                    self.advance();
                    let right = self.multiplicative()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Add(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                        Span::new(start, end),
                    );
                }
                TokenKind::Minus => {
                    self.advance();
                    let right = self.multiplicative()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Sub(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                        Span::new(start, end),
                    );
                }
                _ => break,
            }
        }

        self.exit_recursion();
        Ok(expr)
    }

    // Multiplicative: * / %
    pub(crate) fn multiplicative(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let mut expr = self.exponential()?;

        loop {
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::Star => {
                    self.advance();
                    let right = self.exponential()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Mul(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                        Span::new(start, end),
                    );
                }
                TokenKind::Slash => {
                    self.advance();
                    let right = self.exponential()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Div(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                        Span::new(start, end),
                    );
                }
                TokenKind::Percent => {
                    self.advance();
                    let right = self.exponential()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Mod(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                        Span::new(start, end),
                    );
                }
                _ => break,
            }
        }

        self.exit_recursion();
        Ok(expr)
    }

    // Exponential: ^ removed (use pow() function instead)
    // This now just delegates to unary, will be re-added as bitwise XOR in Phase 2
    pub(crate) fn exponential(&mut self) -> Result<Expr, ParseError> {
        self.unary()
    }
}
