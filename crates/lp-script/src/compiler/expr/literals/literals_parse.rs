/// Literal parsing (numbers, parenthesized expressions)
use crate::compiler::ast::{ExprId, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    // Unary: - ! ~ ++ -- (between exponential and postfix in precedence)
    pub(crate) fn unary(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let token = self.current().clone();

        let result = match &token.kind {
            TokenKind::PlusPlus => {
                self.advance();
                // Prefix increment must be followed by a variable
                if let TokenKind::Ident(name) = &self.current().kind {
                    let name = name.clone();
                    let end = self.current().span.end;
                    self.advance();
                    self.pool
                        .alloc_expr(
                            ExprKind::PreIncrement(name),
                            Span::new(token.span.start, end),
                        )
                        .map_err(|e| self.pool_error_to_parse_error(e))
                } else {
                    // Error: prefix increment requires an l-value
                    // For now, create a dummy expression (will be caught by type checker)
                    self.pool
                        .alloc_expr(
                            ExprKind::Number(0.0),
                            Span::new(token.span.start, self.current().span.end),
                        )
                        .map_err(|e| self.pool_error_to_parse_error(e))
                }
            }
            TokenKind::MinusMinus => {
                self.advance();
                // Prefix decrement must be followed by a variable
                if let TokenKind::Ident(name) = &self.current().kind {
                    let name = name.clone();
                    let end = self.current().span.end;
                    self.advance();
                    self.pool
                        .alloc_expr(
                            ExprKind::PreDecrement(name),
                            Span::new(token.span.start, end),
                        )
                        .map_err(|e| self.pool_error_to_parse_error(e))
                } else {
                    // Error: prefix decrement requires an l-value
                    // For now, create a dummy expression (will be caught by type checker)
                    self.pool
                        .alloc_expr(
                            ExprKind::Number(0.0),
                            Span::new(token.span.start, self.current().span.end),
                        )
                        .map_err(|e| self.pool_error_to_parse_error(e))
                }
            }
            TokenKind::Minus => {
                self.advance();
                let operand_id = self.unary()?; // Right-associative (can stack: --x)
                let operand = self.pool.expr(operand_id);
                let end = operand.span.end;

                // Optimization: if operand is a literal, fold the negation
                match &operand.kind {
                    ExprKind::Number(n) => self
                        .pool
                        .alloc_expr(ExprKind::Number(-n), Span::new(token.span.start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e)),
                    ExprKind::IntNumber(n) => self
                        .pool
                        .alloc_expr(ExprKind::IntNumber(-n), Span::new(token.span.start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e)),
                    _ => self
                        .pool
                        .alloc_expr(ExprKind::Neg(operand_id), Span::new(token.span.start, end))
                        .map_err(|e| self.pool_error_to_parse_error(e)),
                }
            }
            TokenKind::Bang => {
                self.advance();
                let operand_id = self.unary()?;
                let end = self.pool.expr(operand_id).span.end;
                self.pool
                    .alloc_expr(ExprKind::Not(operand_id), Span::new(token.span.start, end))
                    .map_err(|e| self.pool_error_to_parse_error(e))
            }
            TokenKind::Tilde => {
                self.advance();
                let operand_id = self.unary()?;
                let end = self.pool.expr(operand_id).span.end;
                self.pool
                    .alloc_expr(
                        ExprKind::BitwiseNot(operand_id),
                        Span::new(token.span.start, end),
                    )
                    .map_err(|e| self.pool_error_to_parse_error(e))
            }
            _ => self.postfix(),
        };

        self.exit_recursion();
        result
    }

    // Primary: number, variable, function call, constructor, or parenthesized expression
    pub(crate) fn primary(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let token = self.current().clone();

        let result = match &token.kind {
            TokenKind::FloatLiteral(n) => {
                self.advance();
                self.pool
                    .alloc_expr(ExprKind::Number(*n), token.span)
                    .map_err(|e| self.pool_error_to_parse_error(e))
            }
            TokenKind::IntLiteral(n) => {
                self.advance();
                self.pool
                    .alloc_expr(ExprKind::IntNumber(*n), token.span)
                    .map_err(|e| self.pool_error_to_parse_error(e))
            }
            TokenKind::LParen => {
                self.advance(); // consume '('
                let expr_id = self.parse_assignment_expr()?;
                if matches!(self.current().kind, TokenKind::RParen) {
                    self.advance(); // consume ')'
                }
                Ok(expr_id)
            }
            TokenKind::Vec2 | TokenKind::Vec3 | TokenKind::Vec4 => self.parse_vec_constructor(),
            TokenKind::Ident(_) => self.parse_ident(),
            _ => {
                // Error fallback
                self.pool
                    .alloc_expr(ExprKind::Number(0.0), token.span)
                    .map_err(|e| self.pool_error_to_parse_error(e))
            }
        };

        self.exit_recursion();
        result
    }
}
