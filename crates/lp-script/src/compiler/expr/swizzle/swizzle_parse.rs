use lp_pool::LpBox;

/// Swizzle (postfix) operator parsing
use crate::compiler::ast::{Expr, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    // Postfix: swizzle (.xyzw, .rgba, .stpq), postfix increment/decrement (++, --)
    pub(crate) fn postfix(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let mut expr = self.primary()?;

        loop {
            match &self.current().kind {
                TokenKind::Dot => {
                    let start = expr.span.start;
                    self.advance(); // consume '.'

                    // Read the swizzle components
                    if let TokenKind::Ident(components) = &self.current().kind {
                        let components = components.clone();
                        let end = self.current().span.end;
                        self.advance();

                        expr = Expr::new(
                            ExprKind::Swizzle {
                                expr: LpBox::try_new(expr)?,
                                components,
                            },
                            Span::new(start, end),
                        );
                    } else {
                        // Invalid swizzle, just break
                        break;
                    }
                }
                TokenKind::PlusPlus => {
                    // Postfix increment: var++
                    // Only works on variables (l-values)
                    if let ExprKind::Variable(name) = &expr.kind {
                        let name = name.clone();
                        let start = expr.span.start;
                        let end = self.current().span.end;
                        self.advance();
                        expr = Expr::new(ExprKind::PostIncrement(name), Span::new(start, end));
                    } else {
                        // Not an l-value, break (will be caught by type checker)
                        break;
                    }
                }
                TokenKind::MinusMinus => {
                    // Postfix decrement: var--
                    // Only works on variables (l-values)
                    if let ExprKind::Variable(name) = &expr.kind {
                        let name = name.clone();
                        let start = expr.span.start;
                        let end = self.current().span.end;
                        self.advance();
                        expr = Expr::new(ExprKind::PostDecrement(name), Span::new(start, end));
                    } else {
                        // Not an l-value, break (will be caught by type checker)
                        break;
                    }
                }
                _ => break,
            }
        }

        self.exit_recursion();
        Ok(expr)
    }
}
