/// Type checker for LightPlayer Script
///
/// Performs type inference and validation on the AST.
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::lpscript::ast::{Expr, ExprKind, Program, Stmt, StmtKind};
use crate::lpscript::error::{Span, Type, TypeError, TypeErrorKind};

// Import function-related types from compiler::func
pub(crate) use crate::lpscript::compiler::func::{FunctionSignature, FunctionTable};

/// Symbol table for tracking variables in scope
#[derive(Debug, Clone)]
pub(crate) struct SymbolTable {
    scopes: Vec<BTreeMap<String, Type>>,
}

impl SymbolTable {
    pub(crate) fn new() -> Self {
        SymbolTable {
            scopes: vec![BTreeMap::new()],
        }
    }

    pub(crate) fn push_scope(&mut self) {
        self.scopes.push(BTreeMap::new());
    }

    pub(crate) fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub(crate) fn declare(&mut self, name: String, ty: Type) -> Result<(), String> {
        // Check if already declared in current scope
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                return Err(format!(
                    "Variable '{}' already declared in this scope",
                    name
                ));
            }
            scope.insert(name, ty);
        }
        Ok(())
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<Type> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }
}

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
                Self::infer_type(base_expr, symbols, func_table)?;
                let base_type = base_expr.ty.as_ref().unwrap();

                // Validate swizzle is on a vector type
                let source_size = match base_type {
                    Type::Vec2 => 2,
                    Type::Vec3 => 3,
                    Type::Vec4 => 4,
                    _ => {
                        return Err(TypeError {
                            kind: TypeErrorKind::InvalidSwizzle(format!(
                                "Cannot swizzle non-vector type {:?}",
                                base_type
                            )),
                            span: expr.span,
                        });
                    }
                };

                // Validate components and determine result type
                let result_type = Self::validate_swizzle(components, source_size, expr.span)?;
                expr.ty = Some(result_type);
            }
        }

        Ok(())
    }

    /// Get the number of components in a type (for vector constructor validation)
    fn component_count(ty: &Type) -> usize {
        match ty {
            Type::Bool | Type::Fixed | Type::Int32 => 1,
            Type::Vec2 => 2,
            Type::Vec3 => 3,
            Type::Vec4 => 4,
            Type::Void => 0,
        }
    }

    /// Check vector constructor arguments and ensure total components match expected
    fn check_vector_constructor(
        args: &mut [Expr],
        expected_components: usize,
        name: &str,
        span: Span,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        // Type check all arguments
        for arg in args.iter_mut() {
            Self::infer_type(arg, symbols, func_table)?;
        }

        // Count total components provided
        let total: usize = args
            .iter()
            .map(|arg| Self::component_count(arg.ty.as_ref().unwrap()))
            .sum();

        if total != expected_components {
            let types: Vec<Type> = args.iter().map(|arg| arg.ty.clone().unwrap()).collect();

            return Err(TypeError {
                kind: TypeErrorKind::InvalidOperation {
                    op: format!(
                        "{} constructor expects {} components, got {}",
                        name, expected_components, total
                    ),
                    types,
                },
                span,
            });
        }

        Ok(())
    }

    /// Validate swizzle components and return result type
    fn validate_swizzle(
        components: &str,
        source_size: usize,
        span: Span,
    ) -> Result<Type, TypeError> {
        if components.is_empty() {
            return Err(TypeError {
                kind: TypeErrorKind::InvalidSwizzle("Empty swizzle".to_string()),
                span,
            });
        }

        // Check which naming scheme is used (can't mix)
        let is_xyzw = components.chars().all(|c| "xyzw".contains(c));
        let is_rgba = components.chars().all(|c| "rgba".contains(c));
        let is_stpq = components.chars().all(|c| "stpq".contains(c));

        if !is_xyzw && !is_rgba && !is_stpq {
            return Err(TypeError {
                kind: TypeErrorKind::InvalidSwizzle(format!(
                    "Invalid or mixed swizzle naming: '{}'. Use xyzw, rgba, or stpq.",
                    components
                )),
                span,
            });
        }

        // Validate each component is within bounds
        for ch in components.chars() {
            let component_index = if is_xyzw {
                match ch {
                    'x' => 0,
                    'y' => 1,
                    'z' => 2,
                    'w' => 3,
                    _ => unreachable!(),
                }
            } else if is_rgba {
                match ch {
                    'r' => 0,
                    'g' => 1,
                    'b' => 2,
                    'a' => 3,
                    _ => unreachable!(),
                }
            } else {
                // is_stpq
                match ch {
                    's' => 0,
                    't' => 1,
                    'p' => 2,
                    'q' => 3,
                    _ => unreachable!(),
                }
            };

            if component_index >= source_size {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidSwizzle(format!(
                        "Component '{}' out of bounds for vec{}",
                        ch, source_size
                    )),
                    span,
                });
            }
        }

        // Determine result type based on component count
        match components.len() {
            1 => Ok(Type::Fixed), // Single component returns scalar
            2 => Ok(Type::Vec2),
            3 => Ok(Type::Vec3),
            4 => Ok(Type::Vec4),
            _ => Err(TypeError {
                kind: TypeErrorKind::InvalidSwizzle(format!(
                    "Too many swizzle components: {}",
                    components.len()
                )),
                span,
            }),
        }
    }

    pub(crate) fn function_return_type(name: &str, args: &[Expr]) -> Result<Type, TypeError> {
        match name {
            // Math functions: Fixed -> Fixed
            "sin" | "cos" | "tan" | "abs" | "floor" | "ceil" | "sqrt" | "sign" | "frac"
            | "fract" | "saturate" => {
                if args.len() != 1 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 1,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                Ok(Type::Fixed)
            }

            // atan: can take 1 or 2 args (atan(y) or atan(y, x))
            "atan" => {
                if args.is_empty() || args.len() > 2 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 1,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                Ok(Type::Fixed)
            }

            // Vector length: vec -> float
            "length" => {
                if args.len() != 1 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 1,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                let arg_ty = args[0].ty.as_ref().unwrap();
                match arg_ty {
                    Type::Vec2 | Type::Vec3 | Type::Vec4 => Ok(Type::Fixed),
                    _ => Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: "length".to_string(),
                            types: vec![arg_ty.clone()],
                        },
                        span: args[0].span,
                    }),
                }
            }

            // Normalize: vec -> vec (same type)
            "normalize" => {
                if args.len() != 1 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 1,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                let arg_ty = args[0].ty.as_ref().unwrap();
                match arg_ty {
                    Type::Vec2 | Type::Vec3 | Type::Vec4 => Ok(arg_ty.clone()),
                    _ => Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: "normalize".to_string(),
                            types: vec![arg_ty.clone()],
                        },
                        span: args[0].span,
                    }),
                }
            }

            // Dot product: vec x vec -> float
            "dot" => {
                if args.len() != 2 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 2,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                // Both args must be same vector type
                let left_ty = args[0].ty.as_ref().unwrap();
                let right_ty = args[1].ty.as_ref().unwrap();
                if left_ty != right_ty {
                    return Err(TypeError {
                        kind: TypeErrorKind::Mismatch {
                            expected: left_ty.clone(),
                            found: right_ty.clone(),
                        },
                        span: args[1].span,
                    });
                }
                match left_ty {
                    Type::Vec2 | Type::Vec3 | Type::Vec4 => Ok(Type::Fixed),
                    _ => Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: "dot".to_string(),
                            types: vec![left_ty.clone()],
                        },
                        span: args[0].span,
                    }),
                }
            }

            // Distance: vec x vec -> float
            "distance" => {
                if args.len() != 2 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 2,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                let left_ty = args[0].ty.as_ref().unwrap();
                let right_ty = args[1].ty.as_ref().unwrap();
                if left_ty != right_ty {
                    return Err(TypeError {
                        kind: TypeErrorKind::Mismatch {
                            expected: left_ty.clone(),
                            found: right_ty.clone(),
                        },
                        span: args[1].span,
                    });
                }
                match left_ty {
                    Type::Vec2 | Type::Vec3 | Type::Vec4 => Ok(Type::Fixed),
                    _ => Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: "distance".to_string(),
                            types: vec![left_ty.clone()],
                        },
                        span: args[0].span,
                    }),
                }
            }

            // Cross product: vec3 x vec3 -> vec3
            "cross" => {
                if args.len() != 2 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 2,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                // Both must be vec3
                let left_ty = args[0].ty.as_ref().unwrap();
                let right_ty = args[1].ty.as_ref().unwrap();
                if left_ty != &Type::Vec3 || right_ty != &Type::Vec3 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: "cross requires vec3 arguments".to_string(),
                            types: vec![left_ty.clone(), right_ty.clone()],
                        },
                        span: args[0].span,
                    });
                }
                Ok(Type::Vec3)
            }

            // Binary math functions
            "min" | "max" | "pow" | "step" | "mod" => {
                if args.len() != 2 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 2,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                Ok(Type::Fixed)
            }

            // Ternary functions
            "clamp" | "lerp" | "mix" | "smoothstep" => {
                if args.len() != 3 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 3,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                Ok(Type::Fixed)
            }

            // Perlin noise: perlin3(vec3) or perlin3(vec3, octaves)
            "perlin3" => {
                if args.is_empty() || args.len() > 2 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 1,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }

                // First arg must be vec3
                let first_ty = args[0].ty.as_ref().unwrap();
                if first_ty != &Type::Vec3 {
                    return Err(TypeError {
                        kind: TypeErrorKind::Mismatch {
                            expected: Type::Vec3,
                            found: first_ty.clone(),
                        },
                        span: args[0].span,
                    });
                }

                // Second arg (if present) should be int (octaves), but we're lenient
                Ok(Type::Fixed)
            }

            _ => Err(TypeError {
                kind: TypeErrorKind::UndefinedFunction(name.to_string()),
                span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    fn parse_and_check(input: &str) -> Result<Expr, TypeError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        TypeChecker::check(ast)
    }

    #[test]
    fn test_simple_number() {
        let expr = parse_and_check("42.0").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));
    }

    #[test]
    fn test_int_number() {
        let expr = parse_and_check("42").unwrap();
        assert_eq!(expr.ty, Some(Type::Int32));
    }

    #[test]
    fn test_variable() {
        let expr = parse_and_check("xNorm").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));
    }

    #[test]
    fn test_undefined_variable() {
        let result = parse_and_check("undefined");
        assert!(matches!(
            result,
            Err(TypeError {
                kind: TypeErrorKind::UndefinedVariable(_),
                ..
            })
        ));
    }

    #[test]
    fn test_arithmetic() {
        let expr = parse_and_check("1.0 + 2.0").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));
    }

    #[test]
    fn test_function_call() {
        let expr = parse_and_check("sin(time)").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));
    }

    #[test]
    fn test_undefined_function() {
        let result = parse_and_check("unknown(1.0)");
        assert!(matches!(
            result,
            Err(TypeError {
                kind: TypeErrorKind::UndefinedFunction(_),
                ..
            })
        ));
    }

    #[test]
    fn test_vec2_constructor() {
        let expr = parse_and_check("vec2(1.0, 2.0)").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));
    }

    #[test]
    fn test_vec3_constructor() {
        let expr = parse_and_check("vec3(1.0, 2.0, 3.0)").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec3));
    }

    #[test]
    fn test_ternary_type_mismatch() {
        // true and false branches must match
        let result = parse_and_check("xNorm > 0.5 ? vec2(1.0, 0.0) : 1.0");
        assert!(matches!(
            result,
            Err(TypeError {
                kind: TypeErrorKind::Mismatch { .. },
                ..
            })
        ));
    }

    #[test]
    fn test_swizzle_single_component() {
        let expr = parse_and_check("vec2(1.0, 2.0).x").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));

        let expr = parse_and_check("vec3(1.0, 2.0, 3.0).z").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));
    }

    #[test]
    fn test_swizzle_two_components() {
        let expr = parse_and_check("vec2(1.0, 2.0).xy").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));

        let expr = parse_and_check("vec2(1.0, 2.0).yx").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));

        let expr = parse_and_check("vec3(1.0, 2.0, 3.0).xz").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));
    }

    #[test]
    fn test_swizzle_duplicate() {
        let expr = parse_and_check("vec2(1.0, 2.0).xx").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));

        let expr = parse_and_check("vec3(1.0, 2.0, 3.0).zzz").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec3));
    }

    #[test]
    fn test_swizzle_rgba() {
        let expr = parse_and_check("vec4(1.0, 2.0, 3.0, 4.0).rgba").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec4));

        let expr = parse_and_check("vec4(1.0, 2.0, 3.0, 4.0).rgb").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec3));

        let expr = parse_and_check("vec2(1.0, 2.0).rg").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));
    }

    #[test]
    fn test_swizzle_stpq() {
        let expr = parse_and_check("vec2(1.0, 2.0).st").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));

        let expr = parse_and_check("vec4(1.0, 2.0, 3.0, 4.0).stpq").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec4));
    }

    #[test]
    fn test_swizzle_out_of_bounds() {
        let result = parse_and_check("vec2(1.0, 2.0).z");
        assert!(matches!(
            result,
            Err(TypeError {
                kind: TypeErrorKind::InvalidSwizzle(_),
                ..
            })
        ));

        let result = parse_and_check("vec3(1.0, 2.0, 3.0).w");
        assert!(matches!(
            result,
            Err(TypeError {
                kind: TypeErrorKind::InvalidSwizzle(_),
                ..
            })
        ));
    }

    #[test]
    fn test_swizzle_mixed_naming() {
        // Can't mix xyzw with rgba
        let result = parse_and_check("vec2(1.0, 2.0).xr");
        assert!(matches!(
            result,
            Err(TypeError {
                kind: TypeErrorKind::InvalidSwizzle(_),
                ..
            })
        ));
    }

    #[test]
    fn test_swizzle_on_scalar() {
        let result = parse_and_check("xNorm.x");
        assert!(matches!(
            result,
            Err(TypeError {
                kind: TypeErrorKind::InvalidSwizzle(_),
                ..
            })
        ));
    }

    #[test]
    fn test_swizzle_chaining() {
        let expr = parse_and_check("vec3(1.0, 2.0, 3.0).xy.yx").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));
    }

    #[test]
    fn test_uv_variable() {
        let expr = parse_and_check("uv").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));
    }

    #[test]
    fn test_coord_variable() {
        let expr = parse_and_check("coord").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));
    }

    #[test]
    fn test_uv_swizzle() {
        let expr = parse_and_check("uv.x").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));

        let expr = parse_and_check("uv.y").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));

        let expr = parse_and_check("uv.yx").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));
    }

    #[test]
    fn test_coord_swizzle() {
        let expr = parse_and_check("coord.xy").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));
    }

    #[test]
    fn test_uv_in_expression() {
        let expr = parse_and_check("uv.x * 2.0 + uv.y").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));
    }

    #[test]
    fn test_vec3_from_vec2_and_float() {
        // GLSL-style: vec3(vec2, float)
        let expr = parse_and_check("vec3(uv, time)").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec3));
    }

    #[test]
    fn test_vec4_from_vec3_and_float() {
        // GLSL-style: vec4(vec3, float)
        let expr = parse_and_check("vec4(vec3(1.0, 2.0, 3.0), 4.0)").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec4));
    }

    #[test]
    fn test_vec4_from_two_vec2s() {
        // GLSL-style: vec4(vec2, vec2)
        let expr = parse_and_check("vec4(uv, uv)").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec4));
    }

    #[test]
    fn test_vec3_from_float_and_vec2() {
        // GLSL-style: vec3(float, vec2)
        let expr = parse_and_check("vec3(time, uv)").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec3));
    }

    #[test]
    fn test_vec_constructor_wrong_component_count() {
        // Too many components
        let result = parse_and_check("vec2(vec3(1.0, 2.0, 3.0))");
        assert!(matches!(
            result,
            Err(TypeError {
                kind: TypeErrorKind::InvalidOperation { .. },
                ..
            })
        ));

        // Too few components
        let result = parse_and_check("vec3(1.0)");
        assert!(matches!(
            result,
            Err(TypeError {
                kind: TypeErrorKind::InvalidOperation { .. },
                ..
            })
        ));
    }
}
