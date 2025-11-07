/// Integration tests for the analyzer + type checker + codegen pipeline
///
/// These tests verify the multi-pass compilation architecture:
/// 1. Parse: AST + AstPool construction
/// 2. Analyze: Build function metadata table (signatures + locals)
/// 3. Type Check: Validate types using metadata (includes return type validation)
/// 4. Codegen: Generate bytecode using pre-analyzed locals
///
/// Key features tested:
/// - Function signature discovery
/// - Local variable allocation and tracking
/// - Return type validation
/// - Variable shadowing support
/// - Vector parameter handling
/// - Multi-function programs
#[cfg(test)]
mod tests {
    use crate::compiler::analyzer::FunctionAnalyzer;
    use crate::compiler::codegen::CodeGenerator;
    use crate::compiler::lexer::Lexer;
    use crate::compiler::parser::Parser;
    use crate::compiler::typechecker::TypeChecker;
    use crate::shared::Type;
    use crate::{compile_script_with_options, OptimizeOptions};

    #[test]
    fn test_pipeline_simple_function_no_params() {
        let program_text = "
            float getPi() {
                return 3.14;
            }
            return getPi();
        ";

        // Parse
        let mut lexer = Lexer::new(program_text);
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        let (program, pool) = parser.parse_program().expect("parse should succeed");

        // Analyze
        let func_table =
            FunctionAnalyzer::analyze_program(&program, &pool).expect("analysis should succeed");

        // Verify function metadata
        let get_pi_meta = func_table.lookup("getPi").expect("getPi should exist");
        assert_eq!(get_pi_meta.params.len(), 0);
        assert_eq!(get_pi_meta.return_type, Type::Fixed);
        assert_eq!(get_pi_meta.local_count, 0);

        // Type check
        let (typed_program, pool) = TypeChecker::check_program(program, pool, &func_table)
            .expect("type check should succeed");

        // Codegen
        let functions =
            CodeGenerator::generate_program_with_functions(&pool, &typed_program, &func_table);

        // Verify we have main + getPi
        assert_eq!(functions.len(), 2);
        assert_eq!(functions[0].name, "main");
        assert_eq!(functions[1].name, "getPi");
    }

    #[test]
    fn test_pipeline_function_with_params() {
        let program_text = "
            float add(float a, float b) {
                return a + b;
            }
            return add(1.0, 2.0);
        ";

        let mut lexer = Lexer::new(program_text);
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        let (program, pool) = parser.parse_program().expect("parse should succeed");

        let func_table =
            FunctionAnalyzer::analyze_program(&program, &pool).expect("analysis should succeed");

        let add_meta = func_table.lookup("add").expect("add should exist");
        assert_eq!(add_meta.params.len(), 2);
        assert_eq!(add_meta.local_count, 2); // 2 params = 2 locals

        let (_typed_program, pool) = TypeChecker::check_program(program, pool, &func_table)
            .expect("type check should succeed");

        // Make sure we can generate code
        let _functions =
            CodeGenerator::generate_program_with_functions(&pool, &_typed_program, &func_table);
    }

    #[test]
    fn test_pipeline_function_with_local_variables() {
        let program_text = "
            float calculate(float x) {
                float temp = x * 2.0;
                float result = temp + 1.0;
                return result;
            }
            return calculate(5.0);
        ";

        let mut lexer = Lexer::new(program_text);
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        let (program, pool) = parser.parse_program().expect("parse should succeed");

        let func_table =
            FunctionAnalyzer::analyze_program(&program, &pool).expect("analysis should succeed");

        let calc_meta = func_table
            .lookup("calculate")
            .expect("calculate should exist");
        // 1 param + 2 locals = 3 total
        assert_eq!(calc_meta.local_count, 3);
        assert_eq!(calc_meta.locals.len(), 3);

        // Verify ordering: param first, then locals in declaration order
        assert_eq!(calc_meta.locals[0].name, "x");
        assert_eq!(calc_meta.locals[0].index, 0);
        assert_eq!(calc_meta.locals[1].name, "temp");
        assert_eq!(calc_meta.locals[1].index, 1);
        assert_eq!(calc_meta.locals[2].name, "result");
        assert_eq!(calc_meta.locals[2].index, 2);

        let (_typed_program, pool) = TypeChecker::check_program(program, pool, &func_table)
            .expect("type check should succeed");

        let functions =
            CodeGenerator::generate_program_with_functions(&pool, &_typed_program, &func_table);

        // Verify codegen produced correct locals
        let calc_func = functions.iter().find(|f| f.name == "calculate").unwrap();
        assert_eq!(calc_func.locals.len(), 3);
    }

    #[test]
    fn test_pipeline_vec_parameter() {
        let program_text = "
            float sumVec2(vec2 v) {
                return v.x + v.y;
            }
            return sumVec2(vec2(3.0, 4.0));
        ";

        let mut lexer = Lexer::new(program_text);
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        let (program, pool) = parser.parse_program().expect("parse should succeed");

        let func_table =
            FunctionAnalyzer::analyze_program(&program, &pool).expect("analysis should succeed");

        let sum_meta = func_table.lookup("sumVec2").expect("sumVec2 should exist");
        assert_eq!(sum_meta.params.len(), 1);
        assert_eq!(sum_meta.params[0], Type::Vec2);
        assert_eq!(sum_meta.local_count, 1);
        assert_eq!(sum_meta.locals[0].ty, Type::Vec2);

        let (_typed_program, pool) = TypeChecker::check_program(program, pool, &func_table)
            .expect("type check should succeed");

        let _functions =
            CodeGenerator::generate_program_with_functions(&pool, &_typed_program, &func_table);
    }

    #[test]
    fn test_return_type_validation_success() {
        let program_text = "
            float getFloat() {
                return 42.0;
            }
            return getFloat();
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(
            result.is_ok(),
            "Function with correct return type should compile"
        );
    }

    #[test]
    fn test_return_type_validation_failure() {
        let program_text = "
            float getFloat() {
                return vec2(1.0, 2.0);
            }
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(
            result.is_err(),
            "Function with wrong return type should fail to compile"
        );
    }

    #[test]
    fn test_shadowing_in_function() {
        let program_text = "
            float test() {
                float x = 1.0;
                {
                    float x = 2.0;
                    x = x + 10.0; // Inner x = 12
                }
                return x; // Outer x = 1
            }
            return test();
        ";

        let mut lexer = Lexer::new(program_text);
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        let (program, pool) = parser.parse_program().expect("parse should succeed");

        let func_table =
            FunctionAnalyzer::analyze_program(&program, &pool).expect("analysis should succeed");

        let test_meta = func_table.lookup("test").expect("test should exist");
        // Outer x + inner x = 2 locals
        assert_eq!(test_meta.local_count, 2);
        assert_eq!(test_meta.locals.len(), 2);

        // Both have name "x" but different indices
        assert_eq!(test_meta.locals[0].name, "x");
        assert_eq!(test_meta.locals[0].index, 0);
        assert_eq!(test_meta.locals[1].name, "x");
        assert_eq!(test_meta.locals[1].index, 1);

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(result.is_ok(), "Shadowing should compile successfully");
    }

    #[test]
    fn test_multiple_functions() {
        let program_text = "
            float double(float x) {
                return x * 2.0;
            }
            float triple(float x) {
                return x * 3.0;
            }
            return double(5.0) + triple(3.0);
        ";

        let mut lexer = Lexer::new(program_text);
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        let (program, pool) = parser.parse_program().expect("parse should succeed");

        let func_table =
            FunctionAnalyzer::analyze_program(&program, &pool).expect("analysis should succeed");

        assert!(func_table.lookup("double").is_some());
        assert!(func_table.lookup("triple").is_some());

        let (_typed_program, pool) = TypeChecker::check_program(program, pool, &func_table)
            .expect("type check should succeed");

        let functions =
            CodeGenerator::generate_program_with_functions(&pool, &_typed_program, &func_table);

        // main + double + triple
        assert_eq!(functions.len(), 3);
    }

    #[test]
    fn test_function_calling_function() {
        let program_text = "
            float add(float a, float b) {
                return a + b;
            }
            float addThree(float a, float b, float c) {
                return add(add(a, b), c);
            }
            return addThree(1.0, 2.0, 3.0);
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(
            result.is_ok(),
            "Function calling function should compile: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_nested_blocks_with_locals() {
        let program_text = "
            float test() {
                float a = 1.0;
                {
                    float b = 2.0;
                    {
                        float c = 3.0;
                        return a + b + c;
                    }
                }
            }
            return test();
        ";

        let mut lexer = Lexer::new(program_text);
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        let (program, pool) = parser.parse_program().expect("parse should succeed");

        let func_table =
            FunctionAnalyzer::analyze_program(&program, &pool).expect("analysis should succeed");

        let test_meta = func_table.lookup("test").expect("test should exist");
        // a, b, c = 3 locals
        assert_eq!(test_meta.local_count, 3);

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(result.is_ok(), "Nested blocks should compile successfully");
    }
}
