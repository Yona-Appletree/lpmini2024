/// Swizzle (postfix) operator parsing
use crate::compiler::ast::{ExprId, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    // Postfix: swizzle (.xyzw, .rgba, .stpq), postfix increment/decrement (++, --)
    pub(crate) fn postfix(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let mut expr_id = self.primary()?;

        loop {
            match &self.current().kind {
                TokenKind::Dot => {
                    let start = self.pool.expr(expr_id).span.start;
                    self.advance(); // consume '.'

                    // Read the swizzle components
                    if let TokenKind::Ident(components) = &self.current().kind {
                        let components = components.clone();
                        let end = self.current().span.end;
                        self.advance();

                        expr_id = self
                            .pool
                            .alloc_expr(
                                ExprKind::Swizzle {
                                    expr: expr_id,
                                    components,
                                },
                                Span::new(start, end),
                            )
                            .map_err(|e| self.pool_error_to_parse_error(e))?;
                    } else {
                        // Invalid swizzle, just break
                        break;
                    }
                }
                TokenKind::PlusPlus => {
                    // Postfix increment: var++
                    // Only works on variables (l-values)
                    let expr = self.pool.expr(expr_id);
                    if let ExprKind::Variable(name) = &expr.kind {
                        let name = name.clone();
                        let start = expr.span.start;
                        let end = self.current().span.end;
                        self.advance();
                        expr_id = self
                            .pool
                            .alloc_expr(ExprKind::PostIncrement(name), Span::new(start, end))
                            .map_err(|e| self.pool_error_to_parse_error(e))?;
                    } else {
                        // Not an l-value, break (will be caught by type checker)
                        break;
                    }
                }
                TokenKind::MinusMinus => {
                    // Postfix decrement: var--
                    // Only works on variables (l-values)
                    let expr = self.pool.expr(expr_id);
                    if let ExprKind::Variable(name) = &expr.kind {
                        let name = name.clone();
                        let start = expr.span.start;
                        let end = self.current().span.end;
                        self.advance();
                        expr_id = self
                            .pool
                            .alloc_expr(ExprKind::PostDecrement(name), Span::new(start, end))
                            .map_err(|e| self.pool_error_to_parse_error(e))?;
                    } else {
                        // Not an l-value, break (will be caught by type checker)
                        break;
                    }
                }
                _ => break,
            }
        }

        self.exit_recursion();
        Ok(expr_id)
    }
}
