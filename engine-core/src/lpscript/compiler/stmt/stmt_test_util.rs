/// Test utilities for lpscript statements/scripts - builder pattern for clean testing
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{AstPool, Program};
use crate::lpscript::compiler::codegen;
use crate::lpscript::compiler::stmt::stmt_test_ast::StmtBuilder;
use crate::lpscript::compiler::{lexer, parser, typechecker};
use crate::lpscript::shared::Type;
use crate::lpscript::vm::{LpsOpCode, LpsProgram, LpsVm, VmLimits};
use crate::math::{Fixed, ToFixed, Vec2, Vec3, Vec4};

/// Builder for testing scripts/statements through the compilation pipeline
///
/// Note: Script mode supports variable declarations, control flow, etc.
/// Built-in variables like `x`, `y`, `time` from uv/coord still work.
pub struct ScriptTest {
    input: String,
    expected_ast_builder: Option<Box<dyn FnOnce(&mut StmtBuilder) -> Program>>,
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
            expected_ast_builder: None,
            expected_opcodes: None,
            expected_result: None,
            x: 0.5.to_fixed(),
            y: 0.5.to_fixed(),
            time: Fixed::ZERO,
        }
    }

    /// Expect a specific Program AST structure using a builder closure
    pub fn expect_ast<F>(mut self, builder_fn: F) -> Self
    where
        F: FnOnce(&mut StmtBuilder) -> Program + 'static,
    {
        self.expected_ast_builder = Some(Box::new(builder_fn));
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

        // if self.expected_opcodes.is_none() {
        //     return Err("No expectations set for opcodes.".to_string());
        // }
        //
        // if self.expected_ast_builder.is_none() {
        //     return Err("No expectations set for ast.".to_string());
        // }
        //
        // if self.expected_result.is_none() {
        //     return Err("No expectations set for result.".to_string());
        // }

        // Parse the script
        let mut lexer = lexer::Lexer::new(&self.input);
        let tokens = lexer.tokenize();
        let mut parser = parser::Parser::new(tokens);
        let (ast_program, pool) = match parser.parse_program() {
            Ok(result) => result,
            Err(e) => {
                errors.push(format!("Parse error: {}", e));
                return Err(errors.join("\n\n"));
            }
        };

        // Type check
        let (typed_program, pool) = match typechecker::TypeChecker::check_program(ast_program, pool)
        {
            Ok(result) => result,
            Err(e) => {
                errors.push(format!("Type check error: {}", e));
                return Err(errors.join("\n\n"));
            }
        };

        // Check AST if expected (after type checking)
        if let Some(builder_fn) = self.expected_ast_builder {
            let mut builder = StmtBuilder::new();
            let expected_program = builder_fn(&mut builder);
            let expected_pool = builder.into_pool();
            if !program_eq_ignore_spans_with_pool(
                &typed_program,
                &pool,
                &expected_program,
                &expected_pool,
            ) {
                errors.push(format!(
                    "Program AST mismatch:\nExpected: {:#?}\nActual:   {:#?}",
                    expected_program, typed_program
                ));
            }
        }

        // Generate opcodes and create program
        let (opcodes, local_count, local_types) =
            codegen::CodeGenerator::generate_program(&pool, &typed_program);

        // Create LocalDef entries with correct types
        let locals: Vec<crate::lpscript::vm::LocalDef> = (0..local_count)
            .map(|i| {
                use crate::lpscript::vm::LocalType;
                let ty_enum = match local_types.get(&i) {
                    Some(Type::Int32) => LocalType::Int32(0),
                    Some(Type::Vec2) => LocalType::Vec2(Fixed::ZERO, Fixed::ZERO),
                    Some(Type::Vec3) => LocalType::Vec3(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO),
                    Some(Type::Vec4) => {
                        LocalType::Vec4(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                    }
                    _ => LocalType::Fixed(Fixed::ZERO), // Default for Bool and Fixed
                };
                crate::lpscript::vm::LocalDef::new(
                    alloc::format!("local_{}", i),
                    ty_enum,
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

/// Compare Program AST ignoring spans with pools
fn program_eq_ignore_spans_with_pool(
    actual: &Program,
    actual_pool: &AstPool,
    expected: &Program,
    expected_pool: &AstPool,
) -> bool {
    if actual.stmts.len() != expected.stmts.len() {
        return false;
    }

    actual
        .stmts
        .iter()
        .zip(expected.stmts.iter())
        .all(|(a, e)| stmt_eq_ignore_spans_with_pool(*a, actual_pool, *e, expected_pool))
}

/// Compare Expression AST ignoring spans with pools
fn expr_eq_ignore_spans_with_pool(
    actual_id: crate::lpscript::compiler::ast::ExprId,
    actual_pool: &AstPool,
    expected_id: crate::lpscript::compiler::ast::ExprId,
    expected_pool: &AstPool,
) -> bool {
    use crate::lpscript::compiler::ast::ExprKind;

    let actual = actual_pool.expr(actual_id);
    let expected = expected_pool.expr(expected_id);

    // Compare types if both present
    if actual.ty != expected.ty {
        return false;
    }

    match (&actual.kind, &expected.kind) {
        (ExprKind::Number(n1), ExprKind::Number(n2)) => (n1 - n2).abs() < 0.001,
        (ExprKind::IntNumber(i1), ExprKind::IntNumber(i2)) => i1 == i2,
        (ExprKind::Variable(v1), ExprKind::Variable(v2)) => v1 == v2,

        (ExprKind::Add(l1, r1), ExprKind::Add(l2, r2))
        | (ExprKind::Sub(l1, r1), ExprKind::Sub(l2, r2))
        | (ExprKind::Mul(l1, r1), ExprKind::Mul(l2, r2))
        | (ExprKind::Div(l1, r1), ExprKind::Div(l2, r2))
        | (ExprKind::Mod(l1, r1), ExprKind::Mod(l2, r2)) => {
            expr_eq_ignore_spans_with_pool(*l1, actual_pool, *l2, expected_pool)
                && expr_eq_ignore_spans_with_pool(*r1, actual_pool, *r2, expected_pool)
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
        ) => t1 == t2 && expr_eq_ignore_spans_with_pool(*v1, actual_pool, *v2, expected_pool),

        (ExprKind::Call { name: n1, args: a1 }, ExprKind::Call { name: n2, args: a2 }) => {
            n1 == n2
                && a1.len() == a2.len()
                && a1.iter().zip(a2.iter()).all(|(arg1, arg2)| {
                    expr_eq_ignore_spans_with_pool(*arg1, actual_pool, *arg2, expected_pool)
                })
        }

        _ => false, // Other cases not needed for current tests
    }
}

fn stmt_eq_ignore_spans_with_pool(
    actual_id: crate::lpscript::compiler::ast::StmtId,
    actual_pool: &AstPool,
    expected_id: crate::lpscript::compiler::ast::StmtId,
    expected_pool: &AstPool,
) -> bool {
    use crate::lpscript::compiler::ast::StmtKind;

    let actual = actual_pool.stmt(actual_id);
    let expected = expected_pool.stmt(expected_id);

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
                    (Some(e1), Some(e2)) => {
                        expr_eq_ignore_spans_with_pool(*e1, actual_pool, *e2, expected_pool)
                    }
                    _ => false,
                }
        }

        (StmtKind::Return(e1), StmtKind::Return(e2)) => {
            expr_eq_ignore_spans_with_pool(*e1, actual_pool, *e2, expected_pool)
        }

        (StmtKind::Expr(e1), StmtKind::Expr(e2)) => {
            expr_eq_ignore_spans_with_pool(*e1, actual_pool, *e2, expected_pool)
        }

        (StmtKind::Block(s1), StmtKind::Block(s2)) => {
            s1.len() == s2.len()
                && s1.iter().zip(s2.iter()).all(|(a, b)| {
                    stmt_eq_ignore_spans_with_pool(*a, actual_pool, *b, expected_pool)
                })
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
            expr_eq_ignore_spans_with_pool(*c1, actual_pool, *c2, expected_pool)
                && stmt_eq_ignore_spans_with_pool(*t1, actual_pool, *t2, expected_pool)
                && match (e1, e2) {
                    (None, None) => true,
                    (Some(s1), Some(s2)) => {
                        stmt_eq_ignore_spans_with_pool(*s1, actual_pool, *s2, expected_pool)
                    }
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
        ) => {
            expr_eq_ignore_spans_with_pool(*c1, actual_pool, *c2, expected_pool)
                && stmt_eq_ignore_spans_with_pool(*b1, actual_pool, *b2, expected_pool)
        }

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
                (Some(s1), Some(s2)) => {
                    stmt_eq_ignore_spans_with_pool(*s1, actual_pool, *s2, expected_pool)
                }
                _ => false,
            };
            let cond_match = match (c1, c2) {
                (None, None) => true,
                (Some(e1), Some(e2)) => {
                    expr_eq_ignore_spans_with_pool(*e1, actual_pool, *e2, expected_pool)
                }
                _ => false,
            };
            let inc_match = match (inc1, inc2) {
                (None, None) => true,
                (Some(e1), Some(e2)) => {
                    expr_eq_ignore_spans_with_pool(*e1, actual_pool, *e2, expected_pool)
                }
                _ => false,
            };
            init_match
                && cond_match
                && inc_match
                && stmt_eq_ignore_spans_with_pool(*b1, actual_pool, *b2, expected_pool)
        }

        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpscript::shared::Type;

    #[test]
    fn test_script_simple_var_decl() {
        ScriptTest::new("float x = 5.0; return x;")
            .expect_ast(|b| {
                let init = b.num(5.0);
                let var_stmt = b.var_decl(Type::Fixed, "x", Some(init));
                let ret_var = b.typed_var("x", Type::Fixed);
                let ret_stmt = b.return_stmt(ret_var);
                b.program(vec![var_stmt, ret_stmt])
            })
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
            .expect_ast(|b| {
                let expr = b.num(42.0);
                let stmt = b.return_stmt(expr);
                b.program(vec![stmt])
            })
            .expect_opcodes(vec![LpsOpCode::Push(42.0.to_fixed()), LpsOpCode::Return])
            .expect_result_fixed(42.0)
            .run()
            .expect("Should match AST and result");
    }
}
