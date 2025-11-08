#![cfg(test)]
/// Test utilities for lp-script expressions - builder pattern for clean testing
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::ptr::NonNull;

use lp_pool::LpMemoryPool;

use crate::compiler::ast::Expr;
use crate::compiler::optimize::OptimizeOptions;
use crate::compiler::test_ast::AstBuilder;
use crate::compiler::{codegen, lexer, optimize, parser, typechecker};
use crate::fixed::{Fixed, ToFixed, Vec2, Vec3, Vec4};
use crate::shared::Type;
use crate::vm::lps_vm::LpsVm;
use crate::vm::vm_limits::VmLimits;
use crate::vm::{LpsOpCode, LpsProgram};

type ExprBuilder = Box<dyn FnOnce(&mut AstBuilder) -> Expr>;

/// Builder for testing expressions through the compilation pipeline
///
/// Note: In expression mode, variables like `x`, `y`, `time` are built-in variables
/// that derive their values from the VM's run() parameters. Use `.with_vm_params()`
/// to set these. For script mode tests, you would use `.local_*()` methods instead.
#[cfg(test)]
pub struct ExprTest {
    input: String,
    declared_locals: Vec<(String, Type)>, // For symbol table
    local_initial_values: Vec<(String, Vec<i32>)>, /* Initial values for locals (raw i32 representation) */
    expected_ast_builder: Option<ExprBuilder>,
    expected_opcodes: Option<Vec<LpsOpCode>>,
    expected_result: Option<TestResult>,
    expected_locals: Vec<(String, Fixed)>, // Expected local values after execution
    optimize_options: Option<OptimizeOptions>, // If set, apply optimizations to opcodes
    x: Fixed,
    y: Fixed,
    time: Fixed,
}

#[cfg(test)]
enum TestResult {
    Fixed(Fixed),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
}

#[cfg(test)]
impl ExprTest {
    /// Create a new test case with the given input expression
    pub fn new(input: &str) -> Self {
        ExprTest {
            input: String::from(input),
            declared_locals: Vec::new(),
            local_initial_values: Vec::new(),
            expected_ast_builder: None,
            expected_opcodes: None,
            expected_result: None,
            expected_locals: Vec::new(),
            optimize_options: None, // No optimization by default
            x: 0.5.to_fixed(),
            y: 0.5.to_fixed(),
            time: Fixed::ZERO,
        }
    }

    /// Add a Fixed local variable with initial value
    pub fn local_fixed(mut self, _index: usize, name: &str, value: Fixed) -> Self {
        self.declared_locals.push((String::from(name), Type::Fixed));
        self.local_initial_values
            .push((String::from(name), vec![value.0]));
        self
    }

    /// Add a Vec2 local variable with initial value
    pub fn local_vec2(mut self, _index: usize, name: &str, value: Vec2) -> Self {
        self.declared_locals.push((String::from(name), Type::Vec2));
        self.local_initial_values
            .push((String::from(name), vec![value.x.0, value.y.0]));
        self
    }

    /// Add a Vec3 local variable with initial value
    pub fn local_vec3(mut self, _index: usize, name: &str, value: Vec3) -> Self {
        self.declared_locals.push((String::from(name), Type::Vec3));
        self.local_initial_values
            .push((String::from(name), vec![value.x.0, value.y.0, value.z.0]));
        self
    }

    /// Add a Vec4 local variable with initial value
    pub fn local_vec4(mut self, _index: usize, name: &str, value: Vec4) -> Self {
        self.declared_locals.push((String::from(name), Type::Vec4));
        self.local_initial_values.push((
            String::from(name),
            vec![value.x.0, value.y.0, value.z.0, value.w.0],
        ));
        self
    }

    /// Set x value for built-in `x` variable (default: 0.5)
    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x.to_fixed();
        self
    }

    /// Set y value for built-in `y` variable (default: 0.5)
    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y.to_fixed();
        self
    }

    /// Set time value for built-in (default: 0.0)
    pub fn with_time(mut self, time: f32) -> Self {
        self.time = time.to_fixed();
        self
    }

    /// Set VM run parameters (x, y, time) for built-in variables
    /// Built-in variables like `x`, `y`, `time` derive their values from these
    pub fn with_vm_params(mut self, x: f32, y: f32, time: f32) -> Self {
        self.x = x.to_fixed();
        self.y = y.to_fixed();
        self.time = time.to_fixed();
        self
    }

    /// Enable opcode optimization with custom options
    /// By default, no optimizations are applied
    pub fn with_optimization(mut self, options: OptimizeOptions) -> Self {
        self.optimize_options = Some(options);
        self
    }

    /// Convenience method to enable only peephole optimization
    pub fn with_peephole_optimization(mut self) -> Self {
        let mut options = OptimizeOptions::none();
        options.peephole_optimization = true;
        self.optimize_options = Some(options);
        self
    }

    /// Expect a specific AST structure built with a closure
    /// The closure receives an AstBuilder and returns the root Expr
    pub fn expect_ast<F>(mut self, builder_fn: F) -> Self
    where
        F: FnOnce(&mut AstBuilder) -> Expr + 'static,
    {
        self.expected_ast_builder = Some(Box::new(builder_fn));
        self
    }

    /// Expect specific opcodes to be generated
    pub fn expect_opcodes(mut self, expected: Vec<LpsOpCode>) -> Self {
        self.expected_opcodes = Some(expected);
        self
    }

    /// Expect a specific result when executed (takes f32, converts internally)
    pub fn expect_result_fixed(mut self, expected: f32) -> Self {
        self.expected_result = Some(TestResult::Fixed(expected.to_fixed()));
        self
    }

    /// Expect a boolean result (for comparisons, logical operators)
    /// true = 1, false = 0
    pub fn expect_result_bool(mut self, expected: bool) -> Self {
        self.expected_result = Some(TestResult::Fixed(if expected {
            1.0.to_fixed()
        } else {
            0.0.to_fixed()
        }));
        self
    }

    /// Expect an int32 result (stored as raw i32, not Fixed)
    pub fn expect_result_int(mut self, expected: i32) -> Self {
        self.expected_result = Some(TestResult::Fixed(Fixed(expected)));
        self
    }

    /// Expect a vec2 result
    pub fn expect_result_vec2(mut self, expected: Vec2) -> Self {
        self.expected_result = Some(TestResult::Vec2(expected));
        self
    }

    /// Expect a vec3 result
    pub fn expect_result_vec3(mut self, expected: Vec3) -> Self {
        self.expected_result = Some(TestResult::Vec3(expected));
        self
    }

    /// Expect a vec4 result
    pub fn expect_result_vec4(mut self, expected: Vec4) -> Self {
        self.expected_result = Some(TestResult::Vec4(expected));
        self
    }

    /// Expect a specific value for a local variable after execution
    pub fn expect_local_fixed(mut self, name: &str, expected: f32) -> Self {
        self.expected_locals
            .push((String::from(name), expected.to_fixed()));
        self
    }

    /// Run all expectations and return result
    /// Collects all errors instead of stopping at the first one
    pub fn run(self) -> Result<(), String> {
        const POOL_SIZE: usize = 512 * 1024;
        let mut memory = vec![0u8; POOL_SIZE];
        let memory_ptr =
            NonNull::new(memory.as_mut_ptr()).ok_or_else(|| String::from("pool memory null"))?;
        let pool = unsafe { LpMemoryPool::new(memory_ptr, POOL_SIZE).map_err(String::from)? };

        let ExprTest {
            input,
            declared_locals,
            local_initial_values,
            expected_ast_builder,
            expected_opcodes,
            expected_result,
            expected_locals,
            optimize_options,
            x,
            y,
            time,
        } = self;

        pool.run(|| {
            let mut errors = Vec::new();

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

            if let Err(e) = typechecker::TypeChecker::check(&mut expr) {
                errors.push(format!("Type check error: {}", e));
                return Err(errors.join("\n\n"));
            }

            if let Some(builder_fn) = expected_ast_builder {
                let mut expected_builder = AstBuilder::new();
                let expected_expr = builder_fn(&mut expected_builder);
                if !expr_eq_ignore_spans(&expr, &expected_expr) {
                    errors.push(format!(
                        "AST mismatch:
Expected: {:?}
Actual:   {:?}",
                        expected_expr, expr
                    ));
                }
            }

            if let Some(ref options) = optimize_options {
                optimize::optimize_ast_expr(&mut expr, options);
            }

            let mut opcodes = if !declared_locals.is_empty() {
                let predeclared: Vec<(String, u32, Type)> = declared_locals
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, ty))| (name.clone(), idx as u32, ty.clone()))
                    .collect();
                codegen::CodeGenerator::generate_with_locals(&expr, predeclared)
            } else {
                codegen::CodeGenerator::generate(&expr)
            };

            if let Some(ref options) = optimize_options {
                opcodes = optimize::optimize_opcodes(opcodes, options);
            }

            let local_defs: Vec<crate::LocalVarDef> = declared_locals
                .iter()
                .map(|(name, ty)| {
                    let mut def = crate::LocalVarDef::new(name.clone(), ty.clone());
                    if let Some((_, init_val)) =
                        local_initial_values.iter().find(|(n, _)| n == name)
                    {
                        def = def.with_initial_value(init_val.clone());
                    }
                    def
                })
                .collect();

            let program = LpsProgram::new("test".into())
                .with_functions(vec![crate::vm::FunctionDef::new("main".into(), Type::Void)
                    .with_locals(local_defs)
                    .with_opcodes(opcodes.clone())])
                .with_source(input.clone());

            if let Some(expected) = &expected_opcodes {
                if let Some(main_fn) = program.main_function() {
                    if &main_fn.opcodes != expected {
                        errors.push(format!(
                            "Opcode mismatch:
Expected: {:#?}
Actual:   {:#?}",
                            expected, main_fn.opcodes
                        ));
                    }
                } else {
                    errors.push(String::from("Program has no main function"));
                }
            }

            if expected_result.is_some() || !expected_locals.is_empty() {
                match LpsVm::new(&program, VmLimits::default()) {
                    Ok(mut vm) => {
                        if let Some(expected_result) = expected_result {
                            match expected_result {
                                TestResult::Fixed(expected) => match vm.run_scalar(x, y, time) {
                                    Ok(actual) => {
                                        let diff = (expected.to_f32() - actual.to_f32()).abs();
                                        if diff > 0.01 {
                                            errors.push(format!(
                                                "Result mismatch:
Expected: {}
Actual:   {}
Diff:     {}",
                                                expected.to_f32(),
                                                actual.to_f32(),
                                                diff
                                            ));
                                        }
                                    }
                                    Err(e) => errors.push(format!("Runtime error: {:?}", e)),
                                },
                                TestResult::Vec2(expected) => match vm.run_vec2(x, y, time) {
                                    Ok(actual) => {
                                        let diffs = [
                                            (expected.x.to_f32() - actual.x.to_f32()).abs(),
                                            (expected.y.to_f32() - actual.y.to_f32()).abs(),
                                        ];
                                        if diffs.iter().any(|d| *d > 0.0001) {
                                            errors.push(format!(
                                                "Vec2 result mismatch:
Expected: ({}, {})
Actual:   ({}, {})",
                                                expected.x.to_f32(),
                                                expected.y.to_f32(),
                                                actual.x.to_f32(),
                                                actual.y.to_f32()
                                            ));
                                        }
                                    }
                                    Err(e) => errors.push(format!("Runtime error: {:?}", e)),
                                },
                                TestResult::Vec3(expected) => match vm.run_vec3(x, y, time) {
                                    Ok(actual) => {
                                        let diffs = [
                                            (expected.x.to_f32() - actual.x.to_f32()).abs(),
                                            (expected.y.to_f32() - actual.y.to_f32()).abs(),
                                            (expected.z.to_f32() - actual.z.to_f32()).abs(),
                                        ];
                                        if diffs.iter().any(|d| *d > 0.0001) {
                                            errors.push(format!(
                                                "Vec3 result mismatch:
Expected: ({}, {}, {})
Actual:   ({}, {}, {})",
                                                expected.x.to_f32(),
                                                expected.y.to_f32(),
                                                expected.z.to_f32(),
                                                actual.x.to_f32(),
                                                actual.y.to_f32(),
                                                actual.z.to_f32()
                                            ));
                                        }
                                    }
                                    Err(e) => errors.push(format!("Runtime error: {:?}", e)),
                                },
                                TestResult::Vec4(expected) => match vm.run_vec4(x, y, time) {
                                    Ok(actual) => {
                                        let diffs = [
                                            (expected.x.to_f32() - actual.x.to_f32()).abs(),
                                            (expected.y.to_f32() - actual.y.to_f32()).abs(),
                                            (expected.z.to_f32() - actual.z.to_f32()).abs(),
                                            (expected.w.to_f32() - actual.w.to_f32()).abs(),
                                        ];
                                        if diffs.iter().any(|d| *d > 0.0001) {
                                            errors.push(format!(
                                                "Vec4 result mismatch:
Expected: ({}, {}, {}, {})
Actual:   ({}, {}, {}, {})",
                                                expected.x.to_f32(),
                                                expected.y.to_f32(),
                                                expected.z.to_f32(),
                                                expected.w.to_f32(),
                                                actual.x.to_f32(),
                                                actual.y.to_f32(),
                                                actual.z.to_f32(),
                                                actual.w.to_f32()
                                            ));
                                        }
                                    }
                                    Err(e) => errors.push(format!("Runtime error: {:?}", e)),
                                },
                            }
                        }

                        if !expected_locals.is_empty() {
                            for (name, expected_val) in &expected_locals {
                                match vm.get_local_by_name(name) {
                                    Some(value) => {
                                        let diff = (expected_val.to_f32() - value.to_f32()).abs();
                                        if diff > 0.01 {
                                            errors.push(format!(
                                                "Local '{}' mismatch:\nExpected: {}\nActual:   {}",
                                                name,
                                                expected_val.to_f32(),
                                                value.to_f32()
                                            ));
                                        }
                                    }
                                    None => errors
                                        .push(format!("Local '{}' not found in VM locals", name)),
                                }
                            }
                        }
                    }
                    Err(e) => errors.push(format!("Failed to create VM: {:?}", e)),
                }
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors.join("\n\n"))
            }
        })
    }
}

/// Compare AST expressions ignoring spans but checking types when available
pub(crate) fn expr_eq_ignore_spans(actual: &Expr, expected: &Expr) -> bool {
    use crate::compiler::ast::ExprKind;

    match (&actual.ty, &expected.ty) {
        (Some(a), Some(b)) if a != b => return false,
        _ => {}
    }

    match (&actual.kind, &expected.kind) {
        (ExprKind::Number(a), ExprKind::Number(b)) => (a - b).abs() < 0.0001,
        (ExprKind::IntNumber(a), ExprKind::IntNumber(b)) => a == b,
        (ExprKind::Variable(a), ExprKind::Variable(b)) => a == b,

        (ExprKind::Add(a1, b1), ExprKind::Add(a2, b2))
        | (ExprKind::Sub(a1, b1), ExprKind::Sub(a2, b2))
        | (ExprKind::Mul(a1, b1), ExprKind::Mul(a2, b2))
        | (ExprKind::Div(a1, b1), ExprKind::Div(a2, b2))
        | (ExprKind::Mod(a1, b1), ExprKind::Mod(a2, b2))
        | (ExprKind::BitwiseAnd(a1, b1), ExprKind::BitwiseAnd(a2, b2))
        | (ExprKind::BitwiseOr(a1, b1), ExprKind::BitwiseOr(a2, b2))
        | (ExprKind::BitwiseXor(a1, b1), ExprKind::BitwiseXor(a2, b2))
        | (ExprKind::LeftShift(a1, b1), ExprKind::LeftShift(a2, b2))
        | (ExprKind::RightShift(a1, b1), ExprKind::RightShift(a2, b2))
        | (ExprKind::Less(a1, b1), ExprKind::Less(a2, b2))
        | (ExprKind::Greater(a1, b1), ExprKind::Greater(a2, b2))
        | (ExprKind::LessEq(a1, b1), ExprKind::LessEq(a2, b2))
        | (ExprKind::GreaterEq(a1, b1), ExprKind::GreaterEq(a2, b2))
        | (ExprKind::Eq(a1, b1), ExprKind::Eq(a2, b2))
        | (ExprKind::NotEq(a1, b1), ExprKind::NotEq(a2, b2))
        | (ExprKind::And(a1, b1), ExprKind::And(a2, b2))
        | (ExprKind::Or(a1, b1), ExprKind::Or(a2, b2)) => {
            expr_eq_ignore_spans(a1, a2) && expr_eq_ignore_spans(b1, b2)
        }

        (ExprKind::BitwiseNot(v1), ExprKind::BitwiseNot(v2))
        | (ExprKind::Not(v1), ExprKind::Not(v2))
        | (ExprKind::Neg(v1), ExprKind::Neg(v2)) => expr_eq_ignore_spans(v1, v2),

        (
            ExprKind::Ternary {
                condition: c1,
                true_expr: t1,
                false_expr: f1,
            },
            ExprKind::Ternary {
                condition: c2,
                true_expr: t2,
                false_expr: f2,
            },
        ) => {
            expr_eq_ignore_spans(c1, c2)
                && expr_eq_ignore_spans(t1, t2)
                && expr_eq_ignore_spans(f1, f2)
        }

        (
            ExprKind::Swizzle {
                expr: e1,
                components: c1,
            },
            ExprKind::Swizzle {
                expr: e2,
                components: c2,
            },
        ) => c1 == c2 && expr_eq_ignore_spans(e1, e2),

        (ExprKind::Call { name: n1, args: a1 }, ExprKind::Call { name: n2, args: a2 }) => {
            n1 == n2
                && a1.len() == a2.len()
                && a1
                    .iter()
                    .zip(a2.iter())
                    .all(|(x, y)| expr_eq_ignore_spans(x, y))
        }

        (ExprKind::Vec2Constructor(a1), ExprKind::Vec2Constructor(a2))
        | (ExprKind::Vec3Constructor(a1), ExprKind::Vec3Constructor(a2))
        | (ExprKind::Vec4Constructor(a1), ExprKind::Vec4Constructor(a2)) => {
            a1.len() == a2.len()
                && a1
                    .iter()
                    .zip(a2.iter())
                    .all(|(x, y)| expr_eq_ignore_spans(x, y))
        }

        (
            ExprKind::Assign {
                target: t1,
                value: v1,
            },
            ExprKind::Assign {
                target: t2,
                value: v2,
            },
        ) => t1 == t2 && expr_eq_ignore_spans(v1, v2),

        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_simple_pass() {
        // Most basic test - just parsing
        ExprTest::new("1 + 2").run().expect("Should parse");
    }

    #[test]
    fn test_case_ast_check() {
        // Test AST checking
        ExprTest::new("1 + 2")
            .expect_ast(|b| {
                let left = b.int32(1);
                let right = b.int32(2);
                b.add(left, right, Type::Int32)
            })
            .run()
            .expect("AST should match");
    }

    #[test]
    fn test_case_ast_mismatch() {
        // Test that AST mismatch is caught
        let result = ExprTest::new("1 + 2")
            .expect_ast(|b| {
                let left = b.int32(1);
                let right = b.int32(2);
                b.sub(left, right, Type::Int32) // Wrong: should be Add, not Sub
            })
            .run();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("AST mismatch"));
    }

    #[test]
    fn test_case_opcodes_check() {
        // Test opcode checking
        ExprTest::new("5.0")
            .expect_opcodes(vec![LpsOpCode::Push(5.0.to_fixed()), LpsOpCode::Return])
            .run()
            .expect("Opcodes should match");
    }

    #[test]
    fn test_case_opcodes_mismatch() {
        // Test that opcode mismatch is caught
        let result = ExprTest::new("5.0")
            .expect_opcodes(vec![
                LpsOpCode::Push(99.0.to_fixed()), // Wrong value
                LpsOpCode::Return,
            ])
            .run();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Opcode mismatch"));
    }

    #[test]
    fn test_case_result_check() {
        // Test execution result checking
        ExprTest::new("1.0 + 2.0")
            .expect_result_fixed(3.0)
            .run()
            .expect("Result should be 3.0");
    }

    #[test]
    fn test_case_result_mismatch() {
        // Test that result mismatch is caught
        let result = ExprTest::new("1.0 + 2.0")
            .expect_result_fixed(99.0) // Wrong result
            .run();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Result mismatch"));
    }

    #[test]
    fn test_case_with_vm_params() {
        // Test using VM parameters for built-in variables
        ExprTest::new("x + y")
            .with_vm_params(3.0, 4.0, 0.0)
            .expect_result_fixed(7.0)
            .run()
            .expect("x + y should equal 7.0");
    }

    #[test]
    fn test_case_with_time() {
        // Test using time parameter
        ExprTest::new("time * 2.0")
            .with_time(5.0)
            .expect_result_fixed(10.0)
            .run()
            .expect("time * 2 should equal 10.0");
    }

    #[test]
    fn test_case_multiple_expectations() {
        // Test multiple expectations at once
        ExprTest::new("2.0 * 3.0")
            .expect_ast(|b| {
                let left = b.num(2.0);
                let right = b.num(3.0);
                b.mul(left, right, Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::MulFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(6.0)
            .run()
            .expect("All expectations should pass");
    }

    #[test]
    fn test_case_multiple_errors_collected() {
        // Test that multiple errors are collected and reported together
        let result = ExprTest::new("1.0 + 2.0")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let right = b.num(2.0);
                b.sub(left, right, Type::Fixed) // WRONG: should be Add
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(99.0.to_fixed()), // WRONG: wrong values
                LpsOpCode::Return,
            ])
            .expect_result_fixed(99.0) // WRONG: should be 3.0
            .run();

        assert!(result.is_err());
        let error = result.unwrap_err();

        // Should contain all three errors
        assert!(error.contains("AST mismatch"), "Should report AST error");
        assert!(
            error.contains("Opcode mismatch"),
            "Should report opcode error"
        );
        assert!(
            error.contains("Result mismatch"),
            "Should report result error"
        );

        // Errors should be separated by blank lines
        assert!(error.contains("\n\n"), "Errors should be separated");
    }

    #[test]
    fn test_case_comparison_operators() {
        // Test comparison operators work correctly
        ExprTest::new("5.0 > 3.0")
            .expect_ast(|b| {
                let left = b.num(5.0);
                let right = b.num(3.0);
                b.greater(left, right)
            })
            .expect_result_fixed(1.0)
            .run()
            .expect("5.0 > 3.0 should be true");

        ExprTest::new("2.0 < 1.0")
            .expect_ast(|b| {
                let left = b.num(2.0);
                let right = b.num(1.0);
                b.less(left, right)
            })
            .expect_result_fixed(0.0)
            .run()
            .expect("2.0 < 1.0 should be false");
    }

    #[test]
    fn test_case_type_error() {
        // Test that type errors are caught
        let result = ExprTest::new("undefined_variable").run();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Type check error"));
    }

    #[test]
    fn test_case_chaining() {
        // Test that builder methods can be chained in any order
        let result1 = ExprTest::new("1.0")
            .expect_result_fixed(1.0)
            .expect_ast(|b| b.num(1.0))
            .run();

        let result2 = ExprTest::new("1.0")
            .expect_ast(|b| b.num(1.0))
            .expect_result_fixed(1.0)
            .run();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_case_with_local_fixed() {
        // Test with a local variable (would work in script mode)
        // In expression mode, this tests that locals are properly passed to VM
        ExprTest::new("time")
            .with_time(42.0)
            .expect_result_fixed(42.0)
            .run()
            .expect("Should access time value");
    }

    #[test]
    fn test_ast_comparison_ignores_spans() {
        // Test that AST comparison correctly ignores spans
        ExprTest::new("1 + 2")
            .expect_ast(|b| {
                let left = b.int32(1);
                let right = b.int32(2);
                b.add(left, right, Type::Int32)
            })
            .run()
            .expect("Should match despite different spans");
    }
}
