use alloc::boxed::Box;

/// Bitwise operator parsing (&, |, ^, ~, <<, >>)
use crate::compiler::ast::{Expr, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    // Bitwise OR: |
    pub(crate) fn bitwise_or(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let mut expr = self.bitwise_xor()?;

        while matches!(self.current().kind, TokenKind::Pipe) {
            let start = expr.span.start;
            self.advance();
            let right = self.bitwise_xor()?;
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::BitwiseOr(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                Span::new(start, end),
            );
        }

        self.exit_recursion();
        Ok(expr)
    }

    // Bitwise XOR: ^
    pub(crate) fn bitwise_xor(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let mut expr = self.bitwise_and()?;

        while matches!(self.current().kind, TokenKind::Caret) {
            let start = expr.span.start;
            self.advance();
            let right = self.bitwise_and()?;
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::BitwiseXor(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                Span::new(start, end),
            );
        }

        self.exit_recursion();
        Ok(expr)
    }

    // Bitwise AND: &
    pub(crate) fn bitwise_and(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let mut expr = self.equality()?;

        while matches!(self.current().kind, TokenKind::Ampersand) {
            let start = expr.span.start;
            self.advance();
            let right = self.equality()?;
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::BitwiseAnd(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                Span::new(start, end),
            );
        }

        self.exit_recursion();
        Ok(expr)
    }

    // Shift: << >>
    pub(crate) fn shift(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let mut expr = self.additive()?;

        loop {
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::LShift => {
                    self.advance();
                    let right = self.additive()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::LeftShift(LpBox::try_new(expr)?, LpBox::try_new(right)?),
                        Span::new(start, end),
                    );
                }
                TokenKind::RShift => {
                    self.advance();
                    let right = self.additive()?;
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::RightShift(LpBox::try_new(expr)?, LpBox::try_new(right)?),
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
