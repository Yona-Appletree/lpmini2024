/// Assignment expression parsing
use crate::compiler::ast::{ExprId, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    // Assignment expression: lvalue = rvalue (right-associative)
    // Supports simple assignment (=) and compound assignments (+=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=)
    pub(crate) fn parse_assignment_expr(&mut self) -> Result<ExprId, ParseError> {
        self.enter_recursion()?;
        let expr_id = self.ternary()?;

        // Check if this is an assignment (simple or compound)
        let op_token = self.current().kind.clone();

        let result = match op_token {
            // Simple assignment
            TokenKind::Eq => {
                let expr = self.pool.expr(expr_id);
                if let ExprKind::Variable(name) = &expr.kind {
                    let name = name.clone();
                    let start = expr.span.start;
                    self.advance(); // consume '='
                    let value_id = self.parse_assignment_expr()?; // right-associative
                    let end = self.pool.expr(value_id).span.end;

                    self.pool
                        .alloc_expr(
                            ExprKind::Assign {
                                target: name,
                                value: value_id,
                            },
                            Span::new(start, end),
                        )
                        .map_err(|e| self.pool_error_to_parse_error(e))
                } else {
                    Ok(expr_id)
                }
            }

            // Compound assignments - desugar to simple assignment with binary operation
            // e.g., x += 5 becomes x = x + 5
            TokenKind::PlusEq => self.parse_compound_assignment(expr_id, ExprKind::Add),
            TokenKind::MinusEq => self.parse_compound_assignment(expr_id, ExprKind::Sub),
            TokenKind::StarEq => self.parse_compound_assignment(expr_id, ExprKind::Mul),
            TokenKind::SlashEq => self.parse_compound_assignment(expr_id, ExprKind::Div),
            TokenKind::PercentEq => self.parse_compound_assignment(expr_id, ExprKind::Mod),
            TokenKind::AmpersandEq => self.parse_compound_assignment(expr_id, ExprKind::BitwiseAnd),
            TokenKind::PipeEq => self.parse_compound_assignment(expr_id, ExprKind::BitwiseOr),
            TokenKind::CaretEq => self.parse_compound_assignment(expr_id, ExprKind::BitwiseXor),
            TokenKind::LShiftEq => self.parse_compound_assignment(expr_id, ExprKind::LeftShift),
            TokenKind::RShiftEq => self.parse_compound_assignment(expr_id, ExprKind::RightShift),

            _ => Ok(expr_id),
        };

        self.exit_recursion();
        result
    }

    /// Helper for compound assignment operators
    fn parse_compound_assignment<F>(
        &mut self,
        expr_id: ExprId,
        make_op: F,
    ) -> Result<ExprId, ParseError>
    where
        F: FnOnce(ExprId, ExprId) -> ExprKind,
    {
        let expr = self.pool.expr(expr_id);
        if let ExprKind::Variable(name) = &expr.kind {
            let name = name.clone();
            let start = expr.span.start;
            self.advance(); // consume compound operator
            let rhs_id = self.ternary()?; // Use ternary to avoid infinite recursion

            // Create a reference to the variable for the left side of the operation
            let var_ref_id = self
                .pool
                .alloc_expr(ExprKind::Variable(name.clone()), Span::new(start, start))
                .map_err(|e| self.pool_error_to_parse_error(e))?;

            // Create the binary operation
            let end = self.pool.expr(rhs_id).span.end;
            let op_id = self
                .pool
                .alloc_expr(make_op(var_ref_id, rhs_id), Span::new(start, end))
                .map_err(|e| self.pool_error_to_parse_error(e))?;

            // Create the assignment
            self.pool
                .alloc_expr(
                    ExprKind::Assign {
                        target: name,
                        value: op_id,
                    },
                    Span::new(start, end),
                )
                .map_err(|e| self.pool_error_to_parse_error(e))
        } else {
            Ok(expr_id)
        }
    }
}
