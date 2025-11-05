/// Type checker for LightPlayer Script
///
/// Performs type inference and validation on the AST.
extern crate alloc;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{Expr, ExprKind, Program, Stmt, StmtKind};
use crate::lpscript::error::{Span, Type, TypeError, TypeErrorKind};

// Import function-related types from compiler::func
pub(crate) use crate::lpscript::compiler::func::FunctionTable;
// Import symbol table from compiler::symbol_table
pub(crate) use crate::lpscript::compiler::symbol_table::SymbolTable;

pub struct TypeChecker;

impl TypeChecker {
    /// Type check an expression, returning a typed AST
    pub fn check(mut expr: Expr) -> Result<Expr, TypeError> {
        let mut symbols = SymbolTable::new();
        let func_table = FunctionTable::new(); // Empty for expression mode
        Self::infer_type(&mut expr, &mut symbols, &func_table)?;
        Ok(expr)
    }

    /// Type check a program (script mode)
    pub fn check_program(mut program: Program) -> Result<Program, TypeError> {
        let mut func_table = FunctionTable::new();

        // First pass: Register all function signatures
        for func in &program.functions {
            let param_types: Vec<Type> = func.params.iter().map(|p| p.ty.clone()).collect();
            func_table
                .declare(func.name.clone(), param_types, func.return_type.clone())
                .map_err(|msg| TypeError {
                    kind: TypeErrorKind::UndefinedFunction(msg),
                    span: func.span,
                })?;
        }

        // Second pass: Type check each function body
        for func in &mut program.functions {
            TypeChecker::check_function(func, &func_table)?;
        }

        // Third pass: Type check top-level statements
        let mut symbols = SymbolTable::new();
        for stmt in &mut program.stmts {
            Self::check_stmt(stmt, &mut symbols, &func_table)?;
        }

        Ok(program)
    }

    /// Type check a statement
    pub(crate) fn check_stmt(
        stmt: &mut Stmt,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        match &mut stmt.kind {
            StmtKind::VarDecl { ty, name, init } => {
                // Type check initializer if present
                if let Some(init_expr) = init {
                    Self::infer_type(init_expr, symbols, func_table)?;

                    // Check that initializer type matches declared type
                    let init_ty = init_expr.ty.as_ref().unwrap();
                    if init_ty != ty {
                        // Allow int -> fixed promotion
                        if *ty == Type::Fixed && *init_ty == Type::Int32 {
                            init_expr.ty = Some(Type::Fixed);
                        } else {
                            return Err(TypeError {
                                kind: TypeErrorKind::Mismatch {
                                    expected: ty.clone(),
                                    found: init_ty.clone(),
                                },
                                span: init_expr.span,
                            });
                        }
                    }
                }

                // Declare variable in current scope
                symbols
                    .declare(name.clone(), ty.clone())
                    .map_err(|msg| TypeError {
                        kind: TypeErrorKind::UndefinedVariable(msg),
                        span: stmt.span,
                    })?;
            }

            StmtKind::Assignment { name, value } => {
                // Type check the value
                Self::infer_type(value, symbols, func_table)?;

                // Check that variable exists
                let var_ty = symbols.lookup(name).ok_or_else(|| TypeError {
                    kind: TypeErrorKind::UndefinedVariable(name.clone()),
                    span: stmt.span,
                })?;

                // Check type matches
                let value_ty = value.ty.as_ref().unwrap();
                if &var_ty != value_ty {
                    return Err(TypeError {
                        kind: TypeErrorKind::Mismatch {
                            expected: var_ty,
                            found: value_ty.clone(),
                        },
                        span: value.span,
                    });
                }
            }

            StmtKind::Return(expr) => {
                Self::infer_type(expr, symbols, func_table)?;
            }

            StmtKind::Expr(expr) => {
                Self::infer_type(expr, symbols, func_table)?;
            }

            StmtKind::Block(stmts) => {
                symbols.push_scope();
                for stmt in stmts {
                    Self::check_stmt(stmt, symbols, func_table)?;
                }
                symbols.pop_scope();
            }

            StmtKind::If {
                condition,
                then_stmt,
                else_stmt,
            } => {
                Self::infer_type(condition, symbols, func_table)?;
                Self::check_stmt(then_stmt, symbols, func_table)?;
                if let Some(else_s) = else_stmt {
                    Self::check_stmt(else_s, symbols, func_table)?;
                }
            }

            StmtKind::While { condition, body } => {
                Self::infer_type(condition, symbols, func_table)?;
                Self::check_stmt(body, symbols, func_table)?;
            }

            StmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                symbols.push_scope();

                if let Some(init_stmt) = init {
                    Self::check_stmt(init_stmt, symbols, func_table)?;
                }

                if let Some(cond_expr) = condition {
                    Self::infer_type(cond_expr, symbols, func_table)?;
                }

                if let Some(inc_expr) = increment {
                    Self::infer_type(inc_expr, symbols, func_table)?;
                }

                Self::check_stmt(body, symbols, func_table)?;

                symbols.pop_scope();
            }
        }

        Ok(())
    }

    pub(crate) fn infer_type(
        expr: &mut Expr,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        match &mut expr.kind {
            // Literals
            ExprKind::Number(_) => {
                expr.ty = Some(Type::Fixed);
            }

            ExprKind::IntNumber(_) => {
                expr.ty = Some(Type::Int32);
            }

            ExprKind::Variable(name) => {
                // Check built-ins first, then symbol table
                let var_type = match name.as_str() {
                    // Vec2 built-ins (GLSL-style)
                    "uv" => Type::Vec2,    // normalized coordinates (0..1)
                    "coord" => Type::Vec2, // pixel coordinates

                    // Scalar built-ins
                    "time" | "t" => Type::Fixed,
                    "timeNorm" => Type::Fixed,
                    "centerAngle" | "angle" => Type::Fixed,
                    "centerDist" | "dist" => Type::Fixed,

                    // Legacy scalar built-ins (deprecated, kept for compatibility)
                    "x" | "xNorm" | "y" | "yNorm" => Type::Fixed,

                    // Not a built-in, check symbol table
                    _ => symbols.lookup(name).ok_or_else(|| TypeError {
                        kind: TypeErrorKind::UndefinedVariable(name.clone()),
                        span: expr.span,
                    })?,
                };

                expr.ty = Some(var_type);
            }

            // Binary arithmetic operations
            ExprKind::Add(left, right)
            | ExprKind::Sub(left, right)
            | ExprKind::Mul(left, right)
            | ExprKind::Div(left, right)
            | ExprKind::Mod(left, right)
            | ExprKind::Pow(left, right) => {
                Self::infer_type(left, symbols, func_table)?;
                Self::infer_type(right, symbols, func_table)?;

                let left_ty = left.ty.clone().unwrap();
                let right_ty = right.ty.clone().unwrap();

                // Check for vector-scalar operations
                let result_ty = match (&left_ty, &right_ty) {
                    // Both same type
                    (l, r) if l == r => l.clone(),

                    // Int -> Fixed promotion
                    (Type::Int32, Type::Fixed) => {
                        left.ty = Some(Type::Fixed);
                        Type::Fixed
                    }
                    (Type::Fixed, Type::Int32) => {
                        right.ty = Some(Type::Fixed);
                        Type::Fixed
                    }

                    // Vector * Scalar (returns vector)
                    (Type::Vec2, Type::Fixed | Type::Int32) => Type::Vec2,
                    (Type::Vec3, Type::Fixed | Type::Int32) => Type::Vec3,
                    (Type::Vec4, Type::Fixed | Type::Int32) => Type::Vec4,

                    // Scalar * Vector (returns vector)
                    (Type::Fixed | Type::Int32, Type::Vec2) => Type::Vec2,
                    (Type::Fixed | Type::Int32, Type::Vec3) => Type::Vec3,
                    (Type::Fixed | Type::Int32, Type::Vec4) => Type::Vec4,

                    // Mismatch
                    _ => {
                        return Err(TypeError {
                            kind: TypeErrorKind::Mismatch {
                                expected: left_ty.clone(),
                                found: right_ty.clone(),
                            },
                            span: expr.span,
                        });
                    }
                };

                expr.ty = Some(result_ty);
            }

            // Comparisons return Fixed (0 or 1) - handled in compiler/expr/compare module
            ExprKind::Less(left, right)
            | ExprKind::Greater(left, right)
            | ExprKind::LessEq(left, right)
            | ExprKind::GreaterEq(left, right)
            | ExprKind::Eq(left, right)
            | ExprKind::NotEq(left, right) => {
                let ty = Self::check_comparison(left, right, symbols, func_table)?;
                expr.ty = Some(ty);
            }

            // Logical operations
            ExprKind::And(left, right) | ExprKind::Or(left, right) => {
                Self::infer_type(left, symbols, func_table)?;
                Self::infer_type(right, symbols, func_table)?;

                expr.ty = Some(Type::Bool);
            }

            ExprKind::Not(operand) => {
                Self::infer_type(operand, symbols, func_table)?;
                expr.ty = Some(Type::Bool);
            }

            // Unary negation
            ExprKind::Neg(operand) => {
                Self::infer_type(operand, symbols, func_table)?;
                // Type of negation is same as operand
                expr.ty = operand.ty.clone();
            }

            // Ternary
            ExprKind::Ternary {
                condition,
                true_expr,
                false_expr,
            } => {
                Self::infer_type(condition, symbols, func_table)?;
                Self::infer_type(true_expr, symbols, func_table)?;
                Self::infer_type(false_expr, symbols, func_table)?;

                // Result type is the type of true_expr (must match false_expr)
                let true_ty = true_expr.ty.as_ref().unwrap();
                let false_ty = false_expr.ty.as_ref().unwrap();

                if true_ty != false_ty {
                    return Err(TypeError {
                        kind: TypeErrorKind::Mismatch {
                            expected: true_ty.clone(),
                            found: false_ty.clone(),
                        },
                        span: expr.span,
                    });
                }

                expr.ty = Some(true_ty.clone());
            }

            // Assignment expression
            ExprKind::Assign { target, value } => {
                // Check that variable exists
                let var_ty = symbols.lookup(target).ok_or_else(|| TypeError {
                    kind: TypeErrorKind::UndefinedVariable(target.clone()),
                    span: expr.span,
                })?;

                // Type check the value
                Self::infer_type(value, symbols, func_table)?;
                let value_ty = value.ty.as_ref().unwrap();

                // Check type matches
                if &var_ty != value_ty {
                    return Err(TypeError {
                        kind: TypeErrorKind::Mismatch {
                            expected: var_ty.clone(),
                            found: value_ty.clone(),
                        },
                        span: value.span,
                    });
                }

                // Assignment expression returns the assigned value
                expr.ty = Some(var_ty);
            }

            // Function calls
            ExprKind::Call { name, args } => {
                // Type check arguments
                for arg in args.iter_mut() {
                    Self::infer_type(arg, symbols, func_table)?;
                }

                // Check if it's a user-defined function first
                if let Some(sig) = func_table.lookup(name) {
                    // Validate argument count
                    if args.len() != sig.params.len() {
                        return Err(TypeError {
                            kind: TypeErrorKind::InvalidArgumentCount {
                                expected: sig.params.len(),
                                found: args.len(),
                            },
                            span: expr.span,
                        });
                    }

                    // Validate argument types
                    for (arg, expected_ty) in args.iter().zip(sig.params.iter()) {
                        let arg_ty = arg.ty.as_ref().unwrap();
                        if arg_ty != expected_ty {
                            return Err(TypeError {
                                kind: TypeErrorKind::Mismatch {
                                    expected: expected_ty.clone(),
                                    found: arg_ty.clone(),
                                },
                                span: arg.span,
                            });
                        }
                    }

                    expr.ty = Some(sig.return_type.clone());
                } else {
                    // Built-in function
                    let return_ty = Self::function_return_type(name, args)?;
                    expr.ty = Some(return_ty);
                }
            }

            // Vector constructors
            // In GLSL, these can take mixed vec/scalar args: vec3(vec2, float) is valid
            ExprKind::Vec2Constructor(args) => {
                Self::check_vector_constructor(args, 2, "vec2", expr.span, symbols, func_table)?;
                expr.ty = Some(Type::Vec2);
            }

            ExprKind::Vec3Constructor(args) => {
                Self::check_vector_constructor(args, 3, "vec3", expr.span, symbols, func_table)?;
                expr.ty = Some(Type::Vec3);
            }

            ExprKind::Vec4Constructor(args) => {
                Self::check_vector_constructor(args, 4, "vec4", expr.span, symbols, func_table)?;
                expr.ty = Some(Type::Vec4);
            }

            ExprKind::Swizzle {
                expr: base_expr,
                components,
            } => {
                let result_type =
                    Self::check_swizzle(base_expr, components, symbols, func_table, expr.span)?;
                expr.ty = Some(result_type);
            }
        }

        Ok(())
    }
}
