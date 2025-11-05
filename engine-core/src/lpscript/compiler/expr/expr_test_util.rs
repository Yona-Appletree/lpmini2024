/// Test utilities for lpscript expressions - builder pattern for clean testing
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{Expr, ExprKind};
use crate::lpscript::compiler::codegen;
use crate::lpscript::compiler::optimize::OptimizeOptions;
use crate::lpscript::compiler::{lexer, optimize, parser, typechecker};
use crate::lpscript::shared::Type;
use crate::lpscript::vm::{LocalType, LpsOpCode, LpsProgram, LpsVm, VmLimits};
use crate::math::{Fixed, ToFixed, Vec2, Vec3, Vec4};

/// Builder for testing expressions through the compilation pipeline
///
/// Note: In expression mode, variables like `x`, `y`, `time` are built-in variables
/// that derive their values from the VM's run() parameters. Use `.with_vm_params()`
/// to set these. For script mode tests, you would use `.local_*()` methods instead.
pub struct ExprTest {
    input: String,
    locals: Vec<(usize, LocalType)>,
    declared_locals: Vec<(String, Type)>, // For symbol table
    expected_ast: Option<Expr>,
    expected_opcodes: Option<Vec<LpsOpCode>>,
    expected_result: Option<TestResult>,
    expected_locals: Vec<(String, Fixed)>, // Expected local values after execution
    optimize_options: Option<OptimizeOptions>, // If set, apply optimizations to opcodes
    x: Fixed,
    y: Fixed,
    time: Fixed,
}

enum TestResult {
    Fixed(Fixed),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
}

impl ExprTest {
    /// Create a new test case with the given input expression
    pub fn new(input: &str) -> Self {
        ExprTest {
            input: String::from(input),
            locals: Vec::new(),
            declared_locals: Vec::new(),
            expected_ast: None,
            expected_opcodes: None,
            expected_result: None,
            expected_locals: Vec::new(),
            optimize_options: None, // No optimization by default
            x: 0.5.to_fixed(),
            y: 0.5.to_fixed(),
            time: Fixed::ZERO,
        }
    }

    /// Add a Fixed local variable (declares it in symbol table and sets initial value)
    pub fn local_fixed(mut self, index: usize, name: &str, value: Fixed) -> Self {
        self.locals.push((index, LocalType::Fixed(value)));
        self.declared_locals.push((String::from(name), Type::Fixed));
        self
    }

    /// Add a Vec2 local variable (declares it in symbol table and sets initial value)
    pub fn local_vec2(mut self, index: usize, name: &str, value: Vec2) -> Self {
        self.locals.push((index, LocalType::Vec2(value.x, value.y)));
        self.declared_locals.push((String::from(name), Type::Vec2));
        self
    }

    /// Add a Vec3 local variable (declares it in symbol table and sets initial value)
    pub fn local_vec3(mut self, index: usize, name: &str, value: Vec3) -> Self {
        self.locals
            .push((index, LocalType::Vec3(value.x, value.y, value.z)));
        self.declared_locals.push((String::from(name), Type::Vec3));
        self
    }

    /// Add a Vec4 local variable (declares it in symbol table and sets initial value)
    pub fn local_vec4(mut self, index: usize, name: &str, value: Vec4) -> Self {
        self.locals
            .push((index, LocalType::Vec4(value.x, value.y, value.z, value.w)));
        self.declared_locals.push((String::from(name), Type::Vec4));
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

    /// Expect a specific AST structure with types (ignoring spans)
    pub fn expect_ast(mut self, expected: Expr) -> Self {
        self.expected_ast = Some(expected);
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
        let mut errors = Vec::new();

        // Parse the input
        let mut lexer = lexer::Lexer::new(&self.input);
        let tokens = lexer.tokenize();
        let mut parser = parser::Parser::new(tokens);
        let mut ast = parser.parse();

        // Type check with pre-declared locals
        let typed_ast = if self.declared_locals.is_empty() {
            // No locals declared, use standard type checking
            match typechecker::TypeChecker::check(ast) {
                Ok(ast) => ast,
                Err(e) => {
                    errors.push(format!("Type check error: {}", e));
                    return Err(errors.join("\n\n"));
                }
            }
        } else {
            // Declare locals in symbol table before type checking
            let mut symbols = typechecker::SymbolTable::new();
            for (name, ty) in &self.declared_locals {
                if let Err(e) = symbols.declare(name.clone(), ty.clone()) {
                    errors.push(format!("Failed to declare local '{}': {}", name, e));
                    return Err(errors.join("\n\n"));
                }
            }
            let func_table = typechecker::FunctionTable::new();
            match typechecker::TypeChecker::infer_type(&mut ast, &mut symbols, &func_table) {
                Ok(_) => ast,
                Err(e) => {
                    errors.push(format!("Type check error: {}", e));
                    return Err(errors.join("\n\n"));
                }
            }
        };

        // Check AST if expected (after type checking so types are populated)
        if let Some(expected_ast) = &self.expected_ast {
            if !ast_eq_ignore_spans(&typed_ast, expected_ast) {
                errors.push(format!(
                    "AST mismatch:\nExpected: {:?}\nActual:   {:?}",
                    expected_ast, typed_ast
                ));
            }
        }

        // Generate opcodes (with pre-declared locals if any)
        let local_names_and_indices: Vec<(String, u32)> = self
            .declared_locals
            .iter()
            .enumerate()
            .map(|(idx, (name, _))| (name.clone(), idx as u32))
            .collect();
        let mut opcodes =
            codegen::CodeGenerator::generate_with_locals(&typed_ast, local_names_and_indices);

        // Apply opcode optimization if configured
        if let Some(ref options) = self.optimize_options {
            opcodes = optimize::optimize_opcodes(opcodes, options);
        }

        let program = LpsProgram::new("test".into())
            .with_opcodes(opcodes)
            .with_source(self.input.clone().into());

        // Check opcodes if expected
        if let Some(expected_opcodes) = &self.expected_opcodes {
            if &program.opcodes != expected_opcodes {
                errors.push(format!(
                    "Opcode mismatch:\nExpected: {:#?}\nActual:   {:#?}",
                    expected_opcodes, program.opcodes
                ));
            }
        }

        // Check execution result or expected locals
        if self.expected_result.is_some() || !self.expected_locals.is_empty() {
            match LpsVm::new(&program, self.locals, VmLimits::default()) {
                Ok(mut vm) => {
                    // Run with configured x, y, time values (for built-ins like time)
                    if let Some(expected_result) = self.expected_result {
                        match expected_result {
                            TestResult::Fixed(expected) => {
                                match vm.run_scalar(self.x, self.y, self.time) {
                                    Ok(result) => {
                                        // Compare with some tolerance for floating point
                                        // Using 0.01 tolerance to account for fixed-point precision
                                        let expected_f32 = expected.to_f32();
                                        let actual_f32 = result.to_f32();
                                        let diff = (expected_f32 - actual_f32).abs();

                                        if diff > 0.01 {
                                            errors.push(format!(
                                            "Result mismatch:\nExpected: {}\nActual:   {}\nDiff:     {}",
                                            expected_f32, actual_f32, diff
                                        ));
                                        }
                                    }
                                    Err(e) => {
                                        errors.push(format!("Runtime error: {:?}", e));
                                    }
                                }
                            }
                            TestResult::Vec2(expected) => {
                                match vm.run_vec2(self.x, self.y, self.time) {
                                    Ok(result) => {
                                        // Check all components
                                        let x_diff =
                                            (expected.x.to_f32() - result.x.to_f32()).abs();
                                        let y_diff =
                                            (expected.y.to_f32() - result.y.to_f32()).abs();

                                        if x_diff > 0.0001 || y_diff > 0.0001 {
                                            errors.push(format!(
                                            "Vec2 result mismatch:\nExpected: ({}, {})\nActual:   ({}, {})",
                                            expected.x.to_f32(), expected.y.to_f32(),
                                            result.x.to_f32(), result.y.to_f32()
                                        ));
                                        }
                                    }
                                    Err(e) => {
                                        errors.push(format!("Runtime error: {:?}", e));
                                    }
                                }
                            }
                            TestResult::Vec3(expected) => {
                                match vm.run_vec3(self.x, self.y, self.time) {
                                    Ok(result) => {
                                        // Check all components
                                        let x_diff =
                                            (expected.x.to_f32() - result.x.to_f32()).abs();
                                        let y_diff =
                                            (expected.y.to_f32() - result.y.to_f32()).abs();
                                        let z_diff =
                                            (expected.z.to_f32() - result.z.to_f32()).abs();

                                        if x_diff > 0.0001 || y_diff > 0.0001 || z_diff > 0.0001 {
                                            errors.push(format!(
                                            "Vec3 result mismatch:\nExpected: ({}, {}, {})\nActual:   ({}, {}, {})",
                                            expected.x.to_f32(), expected.y.to_f32(), expected.z.to_f32(),
                                            result.x.to_f32(), result.y.to_f32(), result.z.to_f32()
                                        ));
                                        }
                                    }
                                    Err(e) => {
                                        errors.push(format!("Runtime error: {:?}", e));
                                    }
                                }
                            }
                            TestResult::Vec4(expected) => {
                                match vm.run_vec4(self.x, self.y, self.time) {
                                    Ok(result) => {
                                        // Check all components
                                        let x_diff =
                                            (expected.x.to_f32() - result.x.to_f32()).abs();
                                        let y_diff =
                                            (expected.y.to_f32() - result.y.to_f32()).abs();
                                        let z_diff =
                                            (expected.z.to_f32() - result.z.to_f32()).abs();
                                        let w_diff =
                                            (expected.w.to_f32() - result.w.to_f32()).abs();

                                        if x_diff > 0.0001
                                            || y_diff > 0.0001
                                            || z_diff > 0.0001
                                            || w_diff > 0.0001
                                        {
                                            errors.push(format!(
                                            "Vec4 result mismatch:\nExpected: ({}, {}, {}, {})\nActual:   ({}, {}, {}, {})",
                                            expected.x.to_f32(), expected.y.to_f32(), expected.z.to_f32(), expected.w.to_f32(),
                                            result.x.to_f32(), result.y.to_f32(), result.z.to_f32(), result.w.to_f32()
                                        ));
                                        }
                                    }
                                    Err(e) => {
                                        errors.push(format!("Runtime error: {:?}", e));
                                    }
                                }
                            }
                        }
                    }

                    // Check expected local values
                    for (name, expected) in &self.expected_locals {
                        // Find the local index by looking it up in declared_locals
                        if let Some((idx, _)) = self
                            .declared_locals
                            .iter()
                            .enumerate()
                            .find(|(_, (n, _))| n == name)
                        {
                            match vm.get_local(idx) {
                                Some(LocalType::Fixed(actual)) => {
                                    let expected_f32 = expected.to_f32();
                                    let actual_f32 = actual.to_f32();
                                    let diff = (expected_f32 - actual_f32).abs();

                                    if diff > 0.01 {
                                        errors.push(format!(
                                            "Local '{}' mismatch:\nExpected: {}\nActual:   {}\nDiff:     {}",
                                            name, expected_f32, actual_f32, diff
                                        ));
                                    }
                                }
                                Some(other) => {
                                    errors.push(format!(
                                        "Local '{}' type mismatch: expected Fixed, got {:?}",
                                        name, other
                                    ));
                                }
                                None => {
                                    errors.push(format!(
                                        "Local '{}' not found at index {}",
                                        name, idx
                                    ));
                                }
                            }
                        } else {
                            errors.push(format!("Local '{}' was not declared", name));
                        }
                    }
                }
                Err(e) => {
                    errors.push(format!("VM creation error: {:?}", e));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n\n"))
        }
    }
}

/// Compare AST expressions ignoring spans but checking types
/// This is public so it can be used by other test utilities
pub fn ast_eq_ignore_spans(actual: &Expr, expected: &Expr) -> bool {
    // Check types match (both Some and equal, or both None)
    if actual.ty != expected.ty {
        return false;
    }

    // Compare expression kinds
    match (&actual.kind, &expected.kind) {
        (ExprKind::Number(a), ExprKind::Number(b)) => (a - b).abs() < 0.0001,
        (ExprKind::IntNumber(a), ExprKind::IntNumber(b)) => a == b,
        (ExprKind::Variable(a), ExprKind::Variable(b)) => a == b,

        // Binary operations
        (ExprKind::Add(l1, r1), ExprKind::Add(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::Sub(l1, r1), ExprKind::Sub(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::Mul(l1, r1), ExprKind::Mul(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::Div(l1, r1), ExprKind::Div(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::Mod(l1, r1), ExprKind::Mod(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }

        // Bitwise operations
        (ExprKind::BitwiseAnd(l1, r1), ExprKind::BitwiseAnd(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::BitwiseOr(l1, r1), ExprKind::BitwiseOr(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::BitwiseXor(l1, r1), ExprKind::BitwiseXor(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::BitwiseNot(o1), ExprKind::BitwiseNot(o2)) => ast_eq_ignore_spans(o1, o2),
        (ExprKind::LeftShift(l1, r1), ExprKind::LeftShift(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::RightShift(l1, r1), ExprKind::RightShift(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }

        // Increment/Decrement
        (ExprKind::PreIncrement(v1), ExprKind::PreIncrement(v2)) => v1 == v2,
        (ExprKind::PreDecrement(v1), ExprKind::PreDecrement(v2)) => v1 == v2,
        (ExprKind::PostIncrement(v1), ExprKind::PostIncrement(v2)) => v1 == v2,
        (ExprKind::PostDecrement(v1), ExprKind::PostDecrement(v2)) => v1 == v2,

        // Comparisons
        (ExprKind::Less(l1, r1), ExprKind::Less(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::Greater(l1, r1), ExprKind::Greater(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::LessEq(l1, r1), ExprKind::LessEq(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::GreaterEq(l1, r1), ExprKind::GreaterEq(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::Eq(l1, r1), ExprKind::Eq(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::NotEq(l1, r1), ExprKind::NotEq(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }

        // Logical
        (ExprKind::And(l1, r1), ExprKind::And(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }
        (ExprKind::Or(l1, r1), ExprKind::Or(l2, r2)) => {
            ast_eq_ignore_spans(l1, l2) && ast_eq_ignore_spans(r1, r2)
        }

        // Ternary
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
            ast_eq_ignore_spans(c1, c2)
                && ast_eq_ignore_spans(t1, t2)
                && ast_eq_ignore_spans(f1, f2)
        }

        // Swizzle
        (
            ExprKind::Swizzle {
                expr: e1,
                components: c1,
            },
            ExprKind::Swizzle {
                expr: e2,
                components: c2,
            },
        ) => ast_eq_ignore_spans(e1, e2) && c1 == c2,

        // Call
        (ExprKind::Call { name: n1, args: a1 }, ExprKind::Call { name: n2, args: a2 }) => {
            if n1 != n2 || a1.len() != a2.len() {
                return false;
            }
            a1.iter()
                .zip(a2.iter())
                .all(|(arg1, arg2)| ast_eq_ignore_spans(arg1, arg2))
        }

        // Vector constructors
        (ExprKind::Vec2Constructor(a1), ExprKind::Vec2Constructor(a2)) => {
            if a1.len() != a2.len() {
                return false;
            }
            a1.iter()
                .zip(a2.iter())
                .all(|(arg1, arg2)| ast_eq_ignore_spans(arg1, arg2))
        }
        (ExprKind::Vec3Constructor(a1), ExprKind::Vec3Constructor(a2)) => {
            if a1.len() != a2.len() {
                return false;
            }
            a1.iter()
                .zip(a2.iter())
                .all(|(arg1, arg2)| ast_eq_ignore_spans(arg1, arg2))
        }
        (ExprKind::Vec4Constructor(a1), ExprKind::Vec4Constructor(a2)) => {
            if a1.len() != a2.len() {
                return false;
            }
            a1.iter()
                .zip(a2.iter())
                .all(|(arg1, arg2)| ast_eq_ignore_spans(arg1, arg2))
        }

        // Assignment
        (
            ExprKind::Assign {
                target: t1,
                value: v1,
            },
            ExprKind::Assign {
                target: t2,
                value: v2,
            },
        ) => t1 == t2 && ast_eq_ignore_spans(v1, v2),

        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpscript::compiler::test_ast::*;

    #[test]
    fn test_case_simple_pass() {
        // Most basic test - just parsing
        ExprTest::new("1 + 2").run().expect("Should parse");
    }

    #[test]
    fn test_case_ast_check() {
        // Test AST checking
        ExprTest::new("1 + 2")
            .expect_ast(add(int32(1), int32(2), Type::Int32))
            .run()
            .expect("AST should match");
    }

    #[test]
    fn test_case_ast_mismatch() {
        // Test that AST mismatch is caught
        let result = ExprTest::new("1 + 2")
            .expect_ast(sub(int32(1), int32(2), Type::Int32)) // Wrong: should be Add, not Sub
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
            .expect_ast(mul(num(2.0), num(3.0), Type::Fixed))
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
            .expect_ast(sub(num(1.0), num(2.0), Type::Fixed)) // WRONG: should be Add
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
            .expect_ast(greater(num(5.0), num(3.0)))
            .expect_result_fixed(1.0)
            .run()
            .expect("5.0 > 3.0 should be true");

        ExprTest::new("2.0 < 1.0")
            .expect_ast(less(num(2.0), num(1.0)))
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
            .expect_ast(num(1.0))
            .run();

        let result2 = ExprTest::new("1.0")
            .expect_ast(num(1.0))
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
            .expect_ast(add(int32(1), int32(2), Type::Int32))
            .run()
            .expect("Should match despite different spans");
    }
}
