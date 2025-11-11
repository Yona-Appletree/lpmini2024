/// Test utilities for AST-level optimizations on the recursive `LpBox` AST.
extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use lp_alloc::init_test_allocator;

use crate::compiler::ast::Expr;
use crate::compiler::expr::expr_test_util::expr_eq_ignore_spans;
use crate::compiler::{codegen, lexer, parser, typechecker};
use crate::fixed::{Fixed, ToFixed, Vec2, Vec3, Vec4};
use crate::shared::Type;
use crate::vm::{FunctionDef, LpsProgram, LpsVm, VmLimits};

type ExprBuilder = Box<dyn FnOnce(&mut crate::compiler::test_ast::AstBuilder) -> Expr>;

/// Type alias for optimization pass functions. A pass mutates the expression in place
/// and returns `true` when it performed any change.
pub type OptPassFn = fn(&mut Expr) -> bool;

/// Builder for testing AST optimization passes.
///
/// Supports two testing modes:
/// 1. **Structural** – ensure the pass rewrites the AST as expected
/// 2. **Semantic** – ensure the pass preserves runtime behaviour
pub struct AstOptTest {
    input: String,
    pass: Option<OptPassFn>,
    expected_ast_builder: Option<ExprBuilder>,
    check_semantics: bool,
    x: Fixed,
    y: Fixed,
    time: Fixed,
}

impl AstOptTest {
    pub fn new(input: &str) -> Self {
        AstOptTest {
            input: String::from(input),
            pass: None,
            expected_ast_builder: None,
            check_semantics: false,
            x: 0.5.to_fixed(),
            y: 0.5.to_fixed(),
            time: Fixed::ZERO,
        }
    }

    pub fn with_pass(mut self, pass: OptPassFn) -> Self {
        self.pass = Some(pass);
        self
    }

    pub fn expect_ast<F>(mut self, builder_fn: F) -> Self
    where
        F: FnOnce(&mut crate::compiler::test_ast::AstBuilder) -> Expr + 'static,
    {
        self.expected_ast_builder = Some(Box::new(builder_fn));
        self
    }

    pub fn expect_semantics_preserved(mut self) -> Self {
        self.check_semantics = true;
        self
    }

    pub fn with_time(mut self, time: f32) -> Self {
        self.time = time.to_fixed();
        self
    }

    pub fn with_vm_params(mut self, x: f32, y: f32, time: f32) -> Self {
        self.x = x.to_fixed();
        self.y = y.to_fixed();
        self.time = time.to_fixed();
        self
    }

    pub fn run(self) -> Result<(), String> {
        init_test_allocator();

        let AstOptTest {
            input,
            pass,
            mut expected_ast_builder,
            check_semantics,
            x,
            y,
            time,
        } = self;
        let mut errors = Vec::new();

        // Parse input
        let mut lexer = lexer::Lexer::new(&input);
        let tokens = lexer.tokenize();
        let mut parser = parser::Parser::new(tokens);
        let mut expr = match parser.parse() {
            Ok(expr) => expr,
            Err(e) => {
                errors.push(format!("Parse error: {}", e));
                return Err(errors.join("\n\n"));
            }
        };

        // Type check
        if let Err(e) = typechecker::TypeChecker::check(&mut expr) {
            errors.push(format!("Type check error: {}", e));
            return Err(errors.join("\n\n"));
        }

        let mut optimized_expr = expr.clone();

        if let Some(pass_fn) = pass {
            pass_fn(&mut optimized_expr);
        } else {
            errors.push("No optimization pass specified - call .with_pass()".to_string());
            return Err(errors.join("\n\n"));
        }

        // Structural assertion
        if let Some(builder_fn) = expected_ast_builder.take() {
            let mut expected_builder = crate::compiler::test_ast::AstBuilder::new();
            let expected_expr = builder_fn(&mut expected_builder);
            if !expr_eq_ignore_spans(&optimized_expr, &expected_expr) {
                errors.push(format!(
                    "AST mismatch after optimization:\nExpected: {:?}\nActual:   {:?}",
                    expected_expr, optimized_expr
                ));
            }
        }

        // Semantic preservation
        if check_semantics {
            match (
                evaluate_expr(&expr, &input, x, y, time),
                evaluate_expr(&optimized_expr, &input, x, y, time),
            ) {
                (Ok(expected), Ok(actual)) => {
                    if !expected.approx_eq(&actual) {
                        errors.push(format!(
                            "Semantic mismatch:\nExpected: {:?}\nActual:   {:?}",
                            expected, actual
                        ));
                    }
                }
                (Err(e), _) | (_, Err(e)) => errors.push(e),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n\n"))
        }
    }
}

#[derive(Debug)]
enum EvalResult {
    Scalar(Fixed),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
}

impl EvalResult {
    fn approx_eq(&self, other: &EvalResult) -> bool {
        const EPS: f32 = 0.0001;
        match (self, other) {
            (EvalResult::Scalar(a), EvalResult::Scalar(b)) => {
                (a.to_f32() - b.to_f32()).abs() <= EPS
            }
            (EvalResult::Vec2(a), EvalResult::Vec2(b)) => {
                (a.x.to_f32() - b.x.to_f32()).abs() <= EPS
                    && (a.y.to_f32() - b.y.to_f32()).abs() <= EPS
            }
            (EvalResult::Vec3(a), EvalResult::Vec3(b)) => {
                (a.x.to_f32() - b.x.to_f32()).abs() <= EPS
                    && (a.y.to_f32() - b.y.to_f32()).abs() <= EPS
                    && (a.z.to_f32() - b.z.to_f32()).abs() <= EPS
            }
            (EvalResult::Vec4(a), EvalResult::Vec4(b)) => {
                (a.x.to_f32() - b.x.to_f32()).abs() <= EPS
                    && (a.y.to_f32() - b.y.to_f32()).abs() <= EPS
                    && (a.z.to_f32() - b.z.to_f32()).abs() <= EPS
                    && (a.w.to_f32() - b.w.to_f32()).abs() <= EPS
            }
            _ => false,
        }
    }
}

fn evaluate_expr(
    expr: &Expr,
    source: &str,
    x: Fixed,
    y: Fixed,
    time: Fixed,
) -> Result<EvalResult, String> {
    let return_type = expr.ty.clone().unwrap_or(Type::Fixed);
    let opcodes = codegen::CodeGenerator::generate(expr);
    let program = LpsProgram::new(source.into())
        .with_functions(vec![
            FunctionDef::new("main".into(), return_type.clone()).with_opcodes(opcodes)
        ])
        .with_source(source.into());
    let mut vm = LpsVm::new(&program, VmLimits::default())
        .map_err(|e| format!("Failed to create VM: {:?}", e))?;

    match return_type {
        Type::Fixed | Type::Bool | Type::Int32 => vm
            .run_scalar(x, y, time)
            .map(EvalResult::Scalar)
            .map_err(|e| format!("Runtime error: {:?}", e)),
        Type::Vec2 => vm
            .run_vec2(x, y, time)
            .map(EvalResult::Vec2)
            .map_err(|e| format!("Runtime error: {:?}", e)),
        Type::Vec3 => vm
            .run_vec3(x, y, time)
            .map(EvalResult::Vec3)
            .map_err(|e| format!("Runtime error: {:?}", e)),
        Type::Vec4 => vm
            .run_vec4(x, y, time)
            .map(EvalResult::Vec4)
            .map_err(|e| format!("Runtime error: {:?}", e)),
        Type::Void => Err(String::from("Cannot evaluate expression with void type")),
    }
}
