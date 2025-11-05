/// Test utilities for lpscript statements/scripts - builder pattern for clean testing
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::Program;
use crate::lpscript::compiler::codegen;
use crate::lpscript::compiler::{lexer, parser, typechecker};
use crate::lpscript::vm::{LpsOpCode, LpsProgram, LpsVm, VmLimits};
use crate::math::{Fixed, ToFixed, Vec2, Vec3, Vec4};

/// Builder for testing scripts/statements through the compilation pipeline
///
/// Note: Script mode supports variable declarations, control flow, etc.
/// Built-in variables like `x`, `y`, `time` from uv/coord still work.
pub struct ScriptTest {
    input: String,
    expected_ast: Option<Program>,
    expected_opcodes: Option<Vec<LpsOpCode>>,
    expected_result: Option<TestResult>,
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

impl ScriptTest {
    /// Create a new test case with the given input script
    pub fn new(input: &str) -> Self {
        ScriptTest {
            input: String::from(input),
            expected_ast: None,
            expected_opcodes: None,
            expected_result: None,
            x: 0.5.to_fixed(),
            y: 0.5.to_fixed(),
            time: Fixed::ZERO,
        }
    }

    /// Expect a specific Program AST structure (ignoring spans)
    pub fn expect_ast(mut self, expected: Program) -> Self {
        self.expected_ast = Some(expected);
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
    pub fn with_vm_params(mut self, x: f32, y: f32, time: f32) -> Self {
        self.x = x.to_fixed();
        self.y = y.to_fixed();
        self.time = time.to_fixed();
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

    /// Run all expectations and return result
    /// Collects all errors instead of stopping at the first one
    pub fn run(self) -> Result<(), String> {
        let mut errors = Vec::new();

        if self.expected_opcodes.is_none() {
            return Err("No expectations set for opcodes.".to_string());
        }

        if self.expected_ast.is_none() {
            return Err("No expectations set for ast.".to_string());
        }

        if self.expected_result.is_none() {
            return Err("No expectations set for result.".to_string());
        }

        // Parse the script
        let mut lexer = lexer::Lexer::new(&self.input);
        let tokens = lexer.tokenize();
        let mut parser = parser::Parser::new(tokens);
        let ast_program = parser.parse_program();

        // Type check
        let typed_program = match typechecker::TypeChecker::check_program(ast_program) {
            Ok(prog) => prog,
            Err(e) => {
                errors.push(format!("Type check error: {}", e));
                return Err(errors.join("\n\n"));
            }
        };

        // Check AST if expected (after type checking)
        if let Some(expected_ast) = &self.expected_ast {
            if !program_eq_ignore_spans(&typed_program, expected_ast) {
                errors.push(format!(
                    "Program AST mismatch:\nExpected: {:#?}\nActual:   {:#?}",
                    expected_ast, typed_program
                ));
            }
        }

        // Generate opcodes and create program
        let (opcodes, local_count) = codegen::CodeGenerator::generate_program(&typed_program);

        // Create LocalDef entries
        let locals: Vec<crate::lpscript::vm::LocalDef> = (0..local_count)
            .map(|i| {
                crate::lpscript::vm::LocalDef::new(
                    alloc::format!("local_{}", i),
                    crate::lpscript::vm::LocalType::Fixed(Fixed::ZERO),
                    crate::lpscript::vm::LocalAccess::Scratch,
                )
            })
            .collect();

        let program = LpsProgram::new("test".into())
            .with_opcodes(opcodes)
            .with_locals(locals)
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

        // Check execution result if expected
        if let Some(expected_result) = self.expected_result {
            match LpsVm::new(&program, vec![], VmLimits::default()) {
                Ok(mut vm) => {
                    match expected_result {
                        TestResult::Fixed(expected) => {
                            match vm.run_scalar(self.x, self.y, self.time) {
                                Ok(result) => {
                                    let expected_f32 = expected.to_f32();
                                    let actual_f32 = result.to_f32();
                                    let diff = (expected_f32 - actual_f32).abs();

                                    if diff > 0.0001 {
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
                                    let x_diff = (expected.x.to_f32() - result.x.to_f32()).abs();
                                    let y_diff = (expected.y.to_f32() - result.y.to_f32()).abs();

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
                                    let x_diff = (expected.x.to_f32() - result.x.to_f32()).abs();
                                    let y_diff = (expected.y.to_f32() - result.y.to_f32()).abs();
                                    let z_diff = (expected.z.to_f32() - result.z.to_f32()).abs();

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
                                    let x_diff = (expected.x.to_f32() - result.x.to_f32()).abs();
                                    let y_diff = (expected.y.to_f32() - result.y.to_f32()).abs();
                                    let z_diff = (expected.z.to_f32() - result.z.to_f32()).abs();
                                    let w_diff = (expected.w.to_f32() - result.w.to_f32()).abs();

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

/// Compare Program AST ignoring spans
fn program_eq_ignore_spans(actual: &Program, expected: &Program) -> bool {
    if actual.stmts.len() != expected.stmts.len() {
        return false;
    }

    actual
        .stmts
        .iter()
        .zip(expected.stmts.iter())
        .all(|(a, e)| stmt_eq_ignore_spans(a, e))
}

/// Compare Statement AST ignoring spans
fn stmt_eq_ignore_spans(
    actual: &crate::lpscript::compiler::ast::Stmt,
    expected: &crate::lpscript::compiler::ast::Stmt,
) -> bool {
    use crate::lpscript::compiler::ast::StmtKind;
    use crate::lpscript::compiler::expr::expr_test_util::ast_eq_ignore_spans;

    match (&actual.kind, &expected.kind) {
        (
            StmtKind::VarDecl {
                ty: t1,
                name: n1,
                init: i1,
            },
            StmtKind::VarDecl {
                ty: t2,
                name: n2,
                init: i2,
            },
        ) => {
            t1 == t2
                && n1 == n2
                && match (i1, i2) {
                    (None, None) => true,
                    (Some(e1), Some(e2)) => ast_eq_ignore_spans(e1, e2),
                    _ => false,
                }
        }

        (StmtKind::Return(e1), StmtKind::Return(e2)) => ast_eq_ignore_spans(e1, e2),

        (StmtKind::Expr(e1), StmtKind::Expr(e2)) => ast_eq_ignore_spans(e1, e2),

        (StmtKind::Block(s1), StmtKind::Block(s2)) => {
            s1.len() == s2.len()
                && s1
                    .iter()
                    .zip(s2.iter())
                    .all(|(a, b)| stmt_eq_ignore_spans(a, b))
        }

        (
            StmtKind::If {
                condition: c1,
                then_stmt: t1,
                else_stmt: e1,
            },
            StmtKind::If {
                condition: c2,
                then_stmt: t2,
                else_stmt: e2,
            },
        ) => {
            ast_eq_ignore_spans(c1, c2)
                && stmt_eq_ignore_spans(t1, t2)
                && match (e1, e2) {
                    (None, None) => true,
                    (Some(s1), Some(s2)) => stmt_eq_ignore_spans(s1, s2),
                    _ => false,
                }
        }

        (
            StmtKind::While {
                condition: c1,
                body: b1,
            },
            StmtKind::While {
                condition: c2,
                body: b2,
            },
        ) => ast_eq_ignore_spans(c1, c2) && stmt_eq_ignore_spans(b1, b2),

        (
            StmtKind::For {
                init: i1,
                condition: c1,
                increment: inc1,
                body: b1,
            },
            StmtKind::For {
                init: i2,
                condition: c2,
                increment: inc2,
                body: b2,
            },
        ) => {
            let init_match = match (i1, i2) {
                (None, None) => true,
                (Some(s1), Some(s2)) => stmt_eq_ignore_spans(s1, s2),
                _ => false,
            };
            let cond_match = match (c1, c2) {
                (None, None) => true,
                (Some(e1), Some(e2)) => ast_eq_ignore_spans(e1, e2),
                _ => false,
            };
            let inc_match = match (inc1, inc2) {
                (None, None) => true,
                (Some(e1), Some(e2)) => ast_eq_ignore_spans(e1, e2),
                _ => false,
            };
            init_match && cond_match && inc_match && stmt_eq_ignore_spans(b1, b2)
        }

        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpscript::compiler::stmt::stmt_test_ast::*;
    use crate::lpscript::shared::Type;

    #[test]
    fn test_script_simple_var_decl() {
        ScriptTest::new("float x = 5.0; return x;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(num(5.0))),
                return_stmt(typed_var("x", Type::Fixed)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(5.0)
            .run()
            .expect("Should compile and run");
    }

    #[test]
    fn test_script_with_ast() {
        ScriptTest::new("return 42.0;")
            .expect_ast(program(vec![return_stmt(num(42.0))]))
            .expect_opcodes(vec![LpsOpCode::Push(42.0.to_fixed()), LpsOpCode::Return])
            .expect_result_fixed(42.0)
            .run()
            .expect("Should match AST and result");
    }
}
