/// Assignment expression parsing
use crate::lpscript::compiler::ast::{Expr, ExprKind};
use crate::lpscript::compiler::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::shared::Span;
use alloc::boxed::Box;

impl Parser {
    // Assignment expression: lvalue = rvalue (right-associative)
    // Supports simple assignment (=) and compound assignments (+=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=)
    pub(in crate::lpscript) fn parse_assignment_expr(&mut self) -> Expr {
        let expr = self.ternary();

        // Check if this is an assignment (simple or compound)
        let op_token = self.current().kind.clone();

        match op_token {
            // Simple assignment
            TokenKind::Eq => {
                if let ExprKind::Variable(name) = &expr.kind {
                    let start = expr.span.start;
                    self.advance(); // consume '='
                    let value = Box::new(self.parse_assignment_expr()); // right-associative
                    let end = value.span.end;

                    return Expr::new(
                        ExprKind::Assign {
                            target: name.clone(),
                            value,
                        },
                        Span::new(start, end),
                    );
                }
            }

            // Compound assignments - desugar to simple assignment with binary operation
            // e.g., x += 5 becomes x = x + 5
            TokenKind::PlusEq => {
                if let ExprKind::Variable(name) = &expr.kind {
                    let start = expr.span.start;
                    self.advance(); // consume '+='
                    let rhs = self.ternary(); // Use ternary to avoid infinite recursion
                    let var_ref =
                        Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                    let value = Box::new(Expr::new(
                        ExprKind::Add(Box::new(var_ref), Box::new(rhs)),
                        Span::new(start, self.current().span.end),
                    ));

                    return Expr::new(
                        ExprKind::Assign {
                            target: name.clone(),
                            value,
                        },
                        Span::new(start, self.current().span.end),
                    );
                }
            }

            TokenKind::MinusEq => {
                if let ExprKind::Variable(name) = &expr.kind {
                    let start = expr.span.start;
                    self.advance();
                    let rhs = self.ternary();
                    let var_ref =
                        Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                    let value = Box::new(Expr::new(
                        ExprKind::Sub(Box::new(var_ref), Box::new(rhs)),
                        Span::new(start, self.current().span.end),
                    ));

                    return Expr::new(
                        ExprKind::Assign {
                            target: name.clone(),
                            value,
                        },
                        Span::new(start, self.current().span.end),
                    );
                }
            }

            TokenKind::StarEq => {
                if let ExprKind::Variable(name) = &expr.kind {
                    let start = expr.span.start;
                    self.advance();
                    let rhs = self.ternary();
                    let var_ref =
                        Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                    let value = Box::new(Expr::new(
                        ExprKind::Mul(Box::new(var_ref), Box::new(rhs)),
                        Span::new(start, self.current().span.end),
                    ));

                    return Expr::new(
                        ExprKind::Assign {
                            target: name.clone(),
                            value,
                        },
                        Span::new(start, self.current().span.end),
                    );
                }
            }

            TokenKind::SlashEq => {
                if let ExprKind::Variable(name) = &expr.kind {
                    let start = expr.span.start;
                    self.advance();
                    let rhs = self.ternary();
                    let var_ref =
                        Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                    let value = Box::new(Expr::new(
                        ExprKind::Div(Box::new(var_ref), Box::new(rhs)),
                        Span::new(start, self.current().span.end),
                    ));

                    return Expr::new(
                        ExprKind::Assign {
                            target: name.clone(),
                            value,
                        },
                        Span::new(start, self.current().span.end),
                    );
                }
            }

            TokenKind::PercentEq => {
                if let ExprKind::Variable(name) = &expr.kind {
                    let start = expr.span.start;
                    self.advance();
                    let rhs = self.ternary();
                    let var_ref =
                        Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                    let value = Box::new(Expr::new(
                        ExprKind::Mod(Box::new(var_ref), Box::new(rhs)),
                        Span::new(start, self.current().span.end),
                    ));

                    return Expr::new(
                        ExprKind::Assign {
                            target: name.clone(),
                            value,
                        },
                        Span::new(start, self.current().span.end),
                    );
                }
            }

            TokenKind::AmpersandEq => {
                if let ExprKind::Variable(name) = &expr.kind {
                    let start = expr.span.start;
                    self.advance();
                    let rhs = self.ternary();
                    let var_ref =
                        Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                    let value = Box::new(Expr::new(
                        ExprKind::BitwiseAnd(Box::new(var_ref), Box::new(rhs)),
                        Span::new(start, self.current().span.end),
                    ));

                    return Expr::new(
                        ExprKind::Assign {
                            target: name.clone(),
                            value,
                        },
                        Span::new(start, self.current().span.end),
                    );
                }
            }

            TokenKind::PipeEq => {
                if let ExprKind::Variable(name) = &expr.kind {
                    let start = expr.span.start;
                    self.advance();
                    let rhs = self.ternary();
                    let var_ref =
                        Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                    let value = Box::new(Expr::new(
                        ExprKind::BitwiseOr(Box::new(var_ref), Box::new(rhs)),
                        Span::new(start, self.current().span.end),
                    ));

                    return Expr::new(
                        ExprKind::Assign {
                            target: name.clone(),
                            value,
                        },
                        Span::new(start, self.current().span.end),
                    );
                }
            }

            TokenKind::CaretEq => {
                if let ExprKind::Variable(name) = &expr.kind {
                    let start = expr.span.start;
                    self.advance();
                    let rhs = self.ternary();
                    let var_ref =
                        Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                    let value = Box::new(Expr::new(
                        ExprKind::BitwiseXor(Box::new(var_ref), Box::new(rhs)),
                        Span::new(start, self.current().span.end),
                    ));

                    return Expr::new(
                        ExprKind::Assign {
                            target: name.clone(),
                            value,
                        },
                        Span::new(start, self.current().span.end),
                    );
                }
            }

            TokenKind::LShiftEq => {
                if let ExprKind::Variable(name) = &expr.kind {
                    let start = expr.span.start;
                    self.advance();
                    let rhs = self.ternary();
                    let var_ref =
                        Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                    let value = Box::new(Expr::new(
                        ExprKind::LeftShift(Box::new(var_ref), Box::new(rhs)),
                        Span::new(start, self.current().span.end),
                    ));

                    return Expr::new(
                        ExprKind::Assign {
                            target: name.clone(),
                            value,
                        },
                        Span::new(start, self.current().span.end),
                    );
                }
            }

            TokenKind::RShiftEq => {
                if let ExprKind::Variable(name) = &expr.kind {
                    let start = expr.span.start;
                    self.advance();
                    let rhs = self.ternary();
                    let var_ref =
                        Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));
                    let value = Box::new(Expr::new(
                        ExprKind::RightShift(Box::new(var_ref), Box::new(rhs)),
                        Span::new(start, self.current().span.end),
                    ));

                    return Expr::new(
                        ExprKind::Assign {
                            target: name.clone(),
                            value,
                        },
                        Span::new(start, self.current().span.end),
                    );
                }
            }

            _ => {}
        }

        expr
    }
}
