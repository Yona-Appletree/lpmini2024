/// Test utilities for lp-gfx statements/scripts - builder pattern for clean testing
extern crate alloc;
use alloc::format;
use alloc::string::String;

use lp_alloc::init_test_allocator;
use lp_math::dec32::{Dec32, ToDec32};

use crate::lp_script::compiler::ast::{Program, Stmt};
use crate::lp_script::compiler::error::{CodegenErrorKind, ParseErrorKind, TypeErrorKind};
use crate::lp_script::compiler::expr::expr_test_util::expr_eq_ignore_spans;
use crate::lp_script::compiler::func::FunctionMetadata;
use crate::lp_script::compiler::optimize::{self, OptimizeOptions};
use crate::lp_script::compiler::stmt::stmt_test_ast::StmtBuilder;
use crate::lp_script::compiler::{codegen, lexer, parser, typechecker};
use crate::lp_script::shared::Type;
use crate::lp_script::vm::lps_vm::LpsVm;
use crate::lp_script::vm::vm_limits::VmLimits;
use crate::lp_script::vm::{FunctionDef, LpsOpCode, LpsProgram};

type ProgramBuilder = Box<dyn FnOnce(&mut StmtBuilder) -> Program>;

/// Expected error for test assertions
#[derive(Debug)]
#[allow(dead_code)] // Variants are used in match statements but may not be constructed if methods are unused
enum ExpectedError {
    Parser(ParseErrorKind, Option<String>),
    TypeCheck(TypeErrorKind, Option<String>),
    Codegen(CodegenErrorKind, Option<String>),
}

/// Function types assertion helper
pub struct FunctionMetadataAssertion {
    pub function_name: String,
    pub expected_param_count: Option<usize>,
    pub expected_param_types: Option<Vec<Type>>,
    pub expected_return_type: Option<Type>,
    pub expected_local_count: Option<u32>,
    pub expected_local_names: Option<Vec<String>>,
}

/// Builder for testing scripts/statements through the compilation pipeline
///
/// Supports testing all compilation phases:
/// - Parse: AST construction
/// - Analyze: Function types and local discovery
/// - Type Check: Type validation and return type checking
/// - Codegen: Bytecode generation
/// - Execution: Runtime behavior
pub struct ScriptTest {
    input: String,
    expected_ast_builder: Option<ProgramBuilder>,
    expected_opcodes: Option<Vec<LpsOpCode>>,
    expected_result: Option<TestResult>,
    expected_function_metadata: Vec<FunctionMetadataAssertion>,
    expected_error: Option<ExpectedError>, // If set, expect this error during compilation
    x: Dec32,
    y: Dec32,
    time: Dec32,
}

enum TestResult {
    Dec32(Dec32),
}

impl ScriptTest {
    pub fn new(input: &str) -> Self {
        ScriptTest {
            input: String::from(input),
            expected_ast_builder: None,
            expected_opcodes: None,
            expected_result: None,
            expected_function_metadata: Vec::new(),
            expected_error: None,
            x: 0.5.to_dec32(),
            y: 0.5.to_dec32(),
            time: Dec32::ZERO,
        }
    }

    pub fn expect_ast<F>(mut self, builder_fn: F) -> Self
    where
        F: FnOnce(&mut StmtBuilder) -> Program + 'static,
    {
        self.expected_ast_builder = Some(Box::new(builder_fn));
        self
    }

    pub fn with_time(mut self, time: f32) -> Self {
        self.time = time.to_dec32();
        self
    }

    pub fn expect_opcodes(mut self, expected: Vec<LpsOpCode>) -> Self {
        self.expected_opcodes = Some(expected);
        self
    }

    pub fn expect_result_dec32(mut self, expected: f32) -> Self {
        self.expected_result = Some(TestResult::Dec32(expected.to_dec32()));
        self
    }

    pub fn expect_function_local_count(mut self, func_name: &str, count: u32) -> Self {
        self.expected_function_metadata
            .push(FunctionMetadataAssertion {
                function_name: String::from(func_name),
                expected_param_count: None,
                expected_param_types: None,
                expected_return_type: None,
                expected_local_count: Some(count),
                expected_local_names: None,
            });
        self
    }

    pub fn expect_function_params(mut self, func_name: &str, param_types: Vec<Type>) -> Self {
        self.expected_function_metadata
            .push(FunctionMetadataAssertion {
                function_name: String::from(func_name),
                expected_param_count: Some(param_types.len()),
                expected_param_types: Some(param_types),
                expected_return_type: None,
                expected_local_count: None,
                expected_local_names: None,
            });
        self
    }

    pub fn expect_function_metadata(
        mut self,
        func_name: &str,
        param_types: Vec<Type>,
        return_type: Type,
        local_count: u32,
    ) -> Self {
        self.expected_function_metadata
            .push(FunctionMetadataAssertion {
                function_name: String::from(func_name),
                expected_param_count: Some(param_types.len()),
                expected_param_types: Some(param_types),
                expected_return_type: Some(return_type),
                expected_local_count: Some(local_count),
                expected_local_names: None,
            });
        self
    }

    pub fn expect_function_local_names(mut self, func_name: &str, local_names: Vec<&str>) -> Self {
        if let Some(assertion) = self
            .expected_function_metadata
            .iter_mut()
            .find(|a| a.function_name == func_name)
        {
            assertion.expected_local_names =
                Some(local_names.iter().map(|s| String::from(*s)).collect());
        } else {
            self.expected_function_metadata
                .push(FunctionMetadataAssertion {
                    function_name: String::from(func_name),
                    expected_param_count: None,
                    expected_param_types: None,
                    expected_return_type: None,
                    expected_local_count: None,
                    expected_local_names: Some(
                        local_names.iter().map(|s| String::from(*s)).collect(),
                    ),
                });
        }
        self
    }

    fn check_metadata_assertion(
        metadata: &FunctionMetadata,
        assertion: &FunctionMetadataAssertion,
        errors: &mut Vec<String>,
    ) {
        let func_name = &assertion.function_name;

        if let Some(expected_count) = assertion.expected_param_count {
            if metadata.params.len() != expected_count {
                errors.push(format!(
                    "Function '{}': expected {} params, found {}",
                    func_name,
                    expected_count,
                    metadata.params.len()
                ));
            }
        }

        if let Some(expected_types) = &assertion.expected_param_types {
            if metadata.params != *expected_types {
                errors.push(format!(
                    "Function '{}': param type mismatch
  Expected: {:?}
  Got: {:?}",
                    func_name, expected_types, metadata.params
                ));
            }
        }

        if let Some(expected_return) = &assertion.expected_return_type {
            if &metadata.return_type != expected_return {
                errors.push(format!(
                    "Function '{}': return type mismatch
  Expected: {:?}
  Got: {:?}",
                    func_name, expected_return, metadata.return_type
                ));
            }
        }

        if let Some(expected_count) = assertion.expected_local_count {
            if metadata.locals.len() as u32 != expected_count {
                errors.push(format!(
                    "Function '{}': local count mismatch
  Expected: {}
  Got: {}",
                    func_name,
                    expected_count,
                    metadata.locals.len()
                ));
            }
        }

        if let Some(expected_names) = &assertion.expected_local_names {
            let actual_names: Vec<String> =
                metadata.locals.iter().map(|l| l.name.clone()).collect();
            if &actual_names != expected_names {
                errors.push(format!(
                    "Function '{}': local names mismatch
  Expected: {:?}
  Got: {:?}",
                    func_name, expected_names, actual_names
                ));
            }
        }
    }

    pub fn run(self) -> Result<(), String> {
        init_test_allocator();
        self.execute()
    }

    fn execute(self) -> Result<(), String> {
        let ScriptTest {
            input,
            expected_ast_builder,
            expected_opcodes,
            expected_result,
            expected_function_metadata,
            expected_error,
            x,
            y,
            time,
        } = self;

        let mut errors = Vec::new();

        // Helper to check if an error matches the expected error
        fn check_error_match(actual_error_str: &str, expected: &ExpectedError) -> bool {
            match expected {
                ExpectedError::Parser(_expected_kind, msg_opt) => {
                    if let Some(msg) = msg_opt {
                        actual_error_str.contains(msg)
                    } else {
                        true
                    }
                }
                ExpectedError::TypeCheck(_expected_kind, msg_opt) => {
                    if let Some(msg) = msg_opt {
                        actual_error_str.contains(msg)
                    } else {
                        true
                    }
                }
                ExpectedError::Codegen(_expected_kind, msg_opt) => {
                    if let Some(msg) = msg_opt {
                        actual_error_str.contains(msg)
                    } else {
                        true
                    }
                }
            }
        }

        let mut lexer = lexer::Lexer::new(&input);
        let tokens = lexer.tokenize();
        let parser = parser::Parser::new(tokens);
        let mut program = match parser.parse_program() {
            Ok(program) => program,
            Err(e) => {
                if let Some(expected) = &expected_error {
                    if let ExpectedError::Parser(..) = expected {
                        let error_str = format!("{}", e);
                        if check_error_match(&error_str, expected) {
                            return Ok(()); // Expected error occurred, test passes
                        } else {
                            return Err(format!(
                                "Expected parse error but got different error: {}",
                                e
                            ));
                        }
                    } else {
                        return Err(format!(
                            "Unexpected parse error: {} (expected {:?})",
                            e, expected
                        ));
                    }
                } else {
                    return Err(format!("Parse error: {}", e));
                }
            }
        };

        let func_table =
            match crate::lp_script::compiler::analyzer::FunctionAnalyzer::analyze_program(&program)
            {
                Ok(table) => table,
                Err(e) => {
                    errors.push(format!("Analysis error: {}", e));
                    return Err(errors.join("\n\n"));
                }
            };

        for assertion in &expected_function_metadata {
            if let Some(metadata) = func_table.lookup(&assertion.function_name) {
                Self::check_metadata_assertion(metadata, assertion, &mut errors);
            } else {
                errors.push(format!(
                    "Function types check: Function '{}' not found in function table",
                    assertion.function_name
                ));
            }
        }

        if let Err(e) = typechecker::TypeChecker::check_program(&mut program, &func_table) {
            if let Some(expected) = &expected_error {
                if let ExpectedError::TypeCheck(..) = expected {
                    let error_str = format!("{}", e);
                    if check_error_match(&error_str, expected) {
                        return Ok(()); // Expected error occurred, test passes
                    } else {
                        return Err(format!(
                            "Expected type check error but got different error: {}",
                            e
                        ));
                    }
                } else {
                    return Err(format!(
                        "Unexpected type check error: {} (expected {:?})",
                        e, expected
                    ));
                }
            } else {
                return Err(format!("Type check error: {}", e));
            }
        }

        if let Some(builder_fn) = expected_ast_builder {
            let mut builder = StmtBuilder::new();
            let expected_program = builder_fn(&mut builder);
            if !program_eq_ignore_spans(&program, &expected_program) {
                errors.push(format!(
                    "Program AST mismatch:\nExpected: {:#?}\nActual:   {:#?}",
                    expected_program, program
                ));
            }
        }

        let functions =
            match codegen::CodeGenerator::generate_program_with_functions(&program, &func_table) {
                Ok(funcs) => funcs,
                Err(e) => {
                    if let Some(expected) = &expected_error {
                        if let ExpectedError::Codegen(..) = expected {
                            let error_str = format!("{}", e);
                            if check_error_match(&error_str, expected) {
                                return Ok(()); // Expected error occurred, test passes
                            } else {
                                return Err(format!(
                                    "Expected codegen error but got different error: {}",
                                    e
                                ));
                            }
                        } else {
                            return Err(format!(
                                "Unexpected codegen error: {} (expected {:?})",
                                e, expected
                            ));
                        }
                    } else {
                        return Err(format!("Codegen error: {}", e));
                    }
                }
            };
        let optimize_options = OptimizeOptions::none();
        let vm_functions: Vec<FunctionDef> = functions
            .into_iter()
            .map(|func| {
                let optimized_opcodes =
                    optimize::optimize_opcodes(func.opcodes.clone(), &optimize_options);
                FunctionDef::new(func.name.clone(), func.return_type.clone())
                    .with_params(func.params.clone())
                    .with_locals(func.locals.clone())
                    .with_opcodes(optimized_opcodes)
            })
            .collect();

        let program_obj = LpsProgram::new("test".into())
            .with_functions(vm_functions)
            .with_source(input.clone());

        if let Some(expected) = expected_opcodes.as_ref() {
            if let Some(main_fn) = program_obj.main_function() {
                if &main_fn.opcodes != expected {
                    errors.push(format!(
                        "Opcode mismatch:\nExpected: {:#?}\nActual:   {:#?}",
                        expected, main_fn.opcodes
                    ));
                }
            } else {
                errors.push(String::from("Program has no main function"));
            }
        }

        if let Some(expected) = expected_result {
            match LpsVm::new(&program_obj, VmLimits::default()) {
                Ok(mut vm) => match expected {
                    TestResult::Dec32(expected) => match vm.run_scalar(x, y, time) {
                        Ok(result) => {
                            let diff = (expected.to_f32() - result.to_f32()).abs();
                            if diff > 0.01 {
                                errors.push(format!(
                                    "Result mismatch:\nExpected: {}\nActual:   {}\nDiff:     {}",
                                    expected.to_f32(),
                                    result.to_f32(),
                                    diff
                                ));
                            }
                        }
                        Err(e) => errors.push(format!("Runtime error: {:?}", e)),
                    },
                },
                Err(e) => errors.push(format!("Failed to create VM: {:?}", e)),
            }
        }

        // If we got here and an error was expected, that's a problem
        if let Some(expected) = &expected_error {
            return Err(format!(
                "Expected compilation error ({:?}) but compilation succeeded",
                expected
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n\n"))
        }
    }
}

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

fn stmt_eq_ignore_spans(actual: &Stmt, expected: &Stmt) -> bool {
    use crate::lp_script::compiler::ast::StmtKind;

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
            if t1 != t2 || n1 != n2 {
                return false;
            }
            match (i1, i2) {
                (None, None) => true,
                (Some(a), Some(b)) => expr_eq_ignore_spans(a, b),
                _ => false,
            }
        }
        (StmtKind::Return(a), StmtKind::Return(b)) => expr_eq_ignore_spans(a, b),
        (StmtKind::Expr(a), StmtKind::Expr(b)) => expr_eq_ignore_spans(a, b),
        (StmtKind::Block(a), StmtKind::Block(b)) => {
            a.len() == b.len()
                && a.iter()
                    .zip(b.iter())
                    .all(|(sa, sb)| stmt_eq_ignore_spans(sa, sb))
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
            expr_eq_ignore_spans(c1, c2)
                && stmt_eq_ignore_spans(t1, t2)
                && match (e1, e2) {
                    (None, None) => true,
                    (Some(a), Some(b)) => stmt_eq_ignore_spans(a, b),
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
        ) => expr_eq_ignore_spans(c1, c2) && stmt_eq_ignore_spans(b1, b2),
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
            match (i1, i2) {
                (None, None) => {}
                (Some(a), Some(b)) if stmt_eq_ignore_spans(a, b) => {}
                _ => return false,
            }
            match (c1, c2) {
                (None, None) => {}
                (Some(a), Some(b)) if expr_eq_ignore_spans(a, b) => {}
                _ => return false,
            }
            match (inc1, inc2) {
                (None, None) => {}
                (Some(a), Some(b)) if expr_eq_ignore_spans(a, b) => {}
                _ => return false,
            }
            stmt_eq_ignore_spans(b1, b2)
        }
        _ => false,
    }
}
