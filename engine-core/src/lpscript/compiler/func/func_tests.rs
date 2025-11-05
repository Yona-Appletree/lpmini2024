/// Function tests
#[cfg(test)]
mod parse_tests {
    use crate::lpscript::compiler::lexer::Lexer;
    use crate::lpscript::compiler::parser::Parser;

    #[test]
    fn test_parse_function_no_params() {
        let mut lexer = Lexer::new("float getPi() { return 3.14; }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();

        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "getPi");
        assert_eq!(program.functions[0].params.len(), 0);
    }

    #[test]
    fn test_parse_function_with_params() {
        let mut lexer = Lexer::new("float add(float a, float b) { return a + b; }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();

        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].params.len(), 2);
        assert_eq!(program.functions[0].params[0].name, "a");
        assert_eq!(program.functions[0].params[1].name, "b");
    }

    #[test]
    fn test_parse_function_body() {
        let mut lexer = Lexer::new("float double(float x) { return x * 2.0; }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();

        assert_eq!(program.functions.len(), 1);
        assert!(!program.functions[0].body.is_empty());
    }

    #[test]
    fn test_parse_multiple_functions() {
        let mut lexer = Lexer::new(
            "
            float add(float a, float b) { return a + b; }
            float sub(float a, float b) { return a - b; }
        ",
        );
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();

        assert_eq!(program.functions.len(), 2);
        assert_eq!(program.functions[0].name, "add");
        assert_eq!(program.functions[1].name, "sub");
    }
}

#[cfg(test)]
mod return_path_tests {
    use crate::lpscript::compiler::ast::Program;
    use crate::lpscript::compiler::lexer::Lexer;
    use crate::lpscript::compiler::parser::Parser;
    use crate::lpscript::compiler::typechecker::TypeChecker;
    use crate::lpscript::error::{TypeError, TypeErrorKind};

    fn parse_and_typecheck_program(input: &str) -> Result<Program, TypeError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_program();
        TypeChecker::check_program(ast)
    }

    #[test]
    fn test_function_with_return() {
        // Function with simple return - should pass
        let program = "
            float get_value() {
                return 42.0;
            }
        ";
        let result = parse_and_typecheck_program(program);
        assert!(
            result.is_ok(),
            "Function with return should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_function_missing_return() {
        // Function without return - should fail
        let program = "
            float get_value() {
                float x = 42.0;
            }
        ";
        let result = parse_and_typecheck_program(program);
        assert!(
            matches!(
                result,
                Err(TypeError {
                    kind: TypeErrorKind::MissingReturn(_),
                    ..
                })
            ),
            "Function without return should fail: {:?}",
            result
        );
    }

    #[test]
    fn test_function_if_both_branches_return() {
        // If statement with both branches returning - should pass
        let program = "
            float abs_value(float x) {
                if (x < 0.0) {
                    return -x;
                } else {
                    return x;
                }
            }
        ";
        let result = parse_and_typecheck_program(program);
        assert!(
            result.is_ok(),
            "Function with if-else both returning should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_function_if_only_then_returns() {
        // If statement with only then branch returning - should fail
        let program = "
            float abs_value(float x) {
                if (x < 0.0) {
                    return -x;
                }
            }
        ";
        let result = parse_and_typecheck_program(program);
        assert!(
            matches!(
                result,
                Err(TypeError {
                    kind: TypeErrorKind::MissingReturn(_),
                    ..
                })
            ),
            "Function with only then branch returning should fail: {:?}",
            result
        );
    }

    #[test]
    fn test_function_if_no_else_but_return_after() {
        // If statement without else, but return after - should pass
        let program = "
            float get_value(float x) {
                if (x < 0.0) {
                    return 0.0;
                }
                return x;
            }
        ";
        let result = parse_and_typecheck_program(program);
        assert!(
            result.is_ok(),
            "Function with return after if should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_function_with_void_no_return() {
        // Void function without return - should pass
        let program = "
            void do_nothing() {
                float x = 42.0;
            }
        ";
        let result = parse_and_typecheck_program(program);
        assert!(
            result.is_ok(),
            "Void function without return should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_function_nested_if_returns() {
        // Nested if statements with all paths returning - should pass
        let program = "
            float classify(float x) {
                if (x < 0.0) {
                    return -1.0;
                } else {
                    if (x > 0.0) {
                        return 1.0;
                    } else {
                        return 0.0;
                    }
                }
            }
        ";
        let result = parse_and_typecheck_program(program);
        assert!(
            result.is_ok(),
            "Function with nested if-else all returning should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_function_block_with_return() {
        // Block containing a return - should pass
        let program = "
            float get_value() {
                {
                    return 42.0;
                }
            }
        ";
        let result = parse_and_typecheck_program(program);
        assert!(
            result.is_ok(),
            "Function with block containing return should pass: {:?}",
            result
        );
    }

    #[test]
    fn test_function_loop_does_not_guarantee_return() {
        // While loop with return inside doesn't guarantee - should fail
        let program = "
            float get_value(float x) {
                while (x > 0.0) {
                    return x;
                }
            }
        ";
        let result = parse_and_typecheck_program(program);
        assert!(
            matches!(
                result,
                Err(TypeError {
                    kind: TypeErrorKind::MissingReturn(_),
                    ..
                })
            ),
            "Function with only while loop returning should fail: {:?}",
            result
        );
    }

    #[test]
    fn test_function_for_loop_does_not_guarantee_return() {
        // For loop with return inside doesn't guarantee - should fail
        let program = "
            float get_value() {
                for (float i = 0.0; i < 10.0; i = i + 1.0) {
                    return i;
                }
            }
        ";
        let result = parse_and_typecheck_program(program);
        assert!(
            matches!(
                result,
                Err(TypeError {
                    kind: TypeErrorKind::MissingReturn(_),
                    ..
                })
            ),
            "Function with only for loop returning should fail: {:?}",
            result
        );
    }
}
