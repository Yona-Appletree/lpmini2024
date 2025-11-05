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
        let (program, _pool) = parser.parse_program().expect("parse should succeed");

        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "getPi");
        assert_eq!(program.functions[0].params.len(), 0);
    }

    #[test]
    fn test_parse_function_with_params() {
        let mut lexer = Lexer::new("float add(float a, float b) { return a + b; }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let (program, _pool) = parser.parse_program().expect("parse should succeed");

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
        let (program, _pool) = parser.parse_program().expect("parse should succeed");

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
        let (program, _pool) = parser.parse_program().expect("parse should succeed");

        assert_eq!(program.functions.len(), 2);
        assert_eq!(program.functions[0].name, "add");
        assert_eq!(program.functions[1].name, "sub");
    }
}

#[cfg(test)]
mod return_path_tests {
    use crate::lpscript::compiler::ast::Program;
    use crate::lpscript::compiler::analyzer::FunctionAnalyzer;
    use crate::lpscript::compiler::error::{TypeError, TypeErrorKind};
    use crate::lpscript::compiler::lexer::Lexer;
    use crate::lpscript::compiler::parser::Parser;
    use crate::lpscript::compiler::typechecker::TypeChecker;

    fn parse_and_typecheck_program(input: &str) -> Result<Program, TypeError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let (program, pool) = parser.parse_program().map_err(|e| TypeError {
            kind: TypeErrorKind::UndefinedVariable(format!("Parse error: {}", e)),
            span: crate::lpscript::shared::Span::EMPTY,
        })?;
        
        // Analyze to build function table
        let func_table = FunctionAnalyzer::analyze_program(&program, &pool)?;
        
        // Type check with function table
        let (typed_program, _pool) = TypeChecker::check_program(program, pool, &func_table)?;
        Ok(typed_program)
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

#[cfg(test)]
mod vector_function_tests {
    use crate::lpscript::vm::vm_limits::VmLimits;
    use crate::lpscript::{compile_script_with_options, OptimizeOptions};
    use crate::lpscript::vm::lps_vm::LpsVm;
    use crate::math::ToFixed;

    // ========================================================================
    // Function Integration Tests - Vector Parameters
    // ========================================================================

    #[test]
    fn test_function_vec2_parameter() {
        let program_text = "
            float sumComponents(vec2 v) {
                return v.x + v.y;
            }
            return sumComponents(vec2(3.0, 4.0));
        ";

        let program = compile_script_with_options(program_text, &OptimizeOptions::none())
            .expect("Compilation should succeed");
        let mut vm = LpsVm::new(&program, VmLimits::default()).expect("VM creation should succeed");
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .expect("Execution should succeed");

        let expected = 7.0.to_fixed(); // 3.0 + 4.0
        let diff = (result.to_f32() - expected.to_f32()).abs();
        assert!(
            diff < 0.0001,
            "Expected {}, got {}",
            expected.to_f32(),
            result.to_f32()
        );
    }

    #[test]
    fn test_function_vec3_parameter() {
        let program_text = "
            float maxComponent(vec3 v) {
                float m = v.x;
                if (v.y > m) { m = v.y; }
                if (v.z > m) { m = v.z; }
                return m;
            }
            return maxComponent(vec3(2.0, 5.0, 3.0));
        ";

        let program = compile_script_with_options(program_text, &OptimizeOptions::none())
            .expect("Compilation should succeed");
        let mut vm = LpsVm::new(&program, VmLimits::default()).expect("VM creation should succeed");
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .expect("Execution should succeed");

        let expected = 5.0.to_fixed();
        let diff = (result.to_f32() - expected.to_f32()).abs();
        assert!(
            diff < 0.0001,
            "Expected {}, got {}",
            expected.to_f32(),
            result.to_f32()
        );
    }

    #[test]
    fn test_function_multiple_vec_parameters() {
        let program_text = "
            float dotProduct(vec2 a, vec2 b) {
                return a.x * b.x + a.y * b.y;
            }
            return dotProduct(vec2(2.0, 3.0), vec2(4.0, 5.0));
        ";

        let program = compile_script_with_options(program_text, &OptimizeOptions::none())
            .expect("Compilation should succeed");
        let mut vm = LpsVm::new(&program, VmLimits::default()).expect("VM creation should succeed");
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .expect("Execution should succeed");

        let expected = 23.0.to_fixed(); // 2*4 + 3*5 = 8 + 15 = 23
        let diff = (result.to_f32() - expected.to_f32()).abs();
        assert!(
            diff < 0.0001,
            "Expected {}, got {}",
            expected.to_f32(),
            result.to_f32()
        );
    }

    // ========================================================================
    // Function Integration Tests - Vector Return Values
    // ========================================================================

    #[test]
    fn test_function_returns_vec2() {
        let program_text = "
            vec2 scale(vec2 v, float s) {
                return v * s;
            }
            vec2 result = scale(vec2(2.0, 3.0), 2.0);
            return result.x + result.y;
        ";

        let program = compile_script_with_options(program_text, &OptimizeOptions::none())
            .expect("Compilation should succeed");
        let mut vm = LpsVm::new(&program, VmLimits::default()).expect("VM creation should succeed");
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .expect("Execution should succeed");

        let expected = 10.0.to_fixed(); // (2*2) + (3*2) = 4 + 6 = 10
        let diff = (result.to_f32() - expected.to_f32()).abs();
        assert!(
            diff < 0.0001,
            "Expected {}, got {}",
            expected.to_f32(),
            result.to_f32()
        );
    }

    #[test]
    fn test_function_returns_vec3() {
        let program_text = "
            vec3 addVec3(vec3 a, vec3 b) {
                return a + b;
            }
            vec3 result = addVec3(vec3(1.0, 2.0, 3.0), vec3(4.0, 5.0, 6.0));
            return result.x + result.y + result.z;
        ";

        let program = compile_script_with_options(program_text, &OptimizeOptions::none())
            .expect("Compilation should succeed");
        let mut vm = LpsVm::new(&program, VmLimits::default()).expect("VM creation should succeed");
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .expect("Execution should succeed");

        let expected = 21.0.to_fixed(); // (1+4) + (2+5) + (3+6) = 5 + 7 + 9 = 21
        let diff = (result.to_f32() - expected.to_f32()).abs();
        assert!(
            diff < 0.0001,
            "Expected {}, got {}",
            expected.to_f32(),
            result.to_f32()
        );
    }

    #[test]
    fn test_function_returns_vec4() {
        let program_text = "
            vec4 makeVec4(float x) {
                return vec4(x, x * 2.0, x * 3.0, x * 4.0);
            }
            vec4 result = makeVec4(2.0);
            return result.w;
        ";

        let program = compile_script_with_options(program_text, &OptimizeOptions::none())
            .expect("Compilation should succeed");
        let mut vm = LpsVm::new(&program, VmLimits::default()).expect("VM creation should succeed");
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .expect("Execution should succeed");

        let expected = 8.0.to_fixed(); // 2.0 * 4.0
        let diff = (result.to_f32() - expected.to_f32()).abs();
        assert!(
            diff < 0.0001,
            "Expected {}, got {}",
            expected.to_f32(),
            result.to_f32()
        );
    }

    // ========================================================================
    // Function Integration Tests - Mixed Types
    // ========================================================================

    #[test]
    fn test_function_mixed_scalar_vector_params() {
        let program_text = "
            vec2 scaleAndOffset(vec2 v, float scale, float offset) {
                return v * scale + vec2(offset, offset);
            }
            vec2 result = scaleAndOffset(vec2(2.0, 3.0), 2.0, 1.0);
            return result.x;
        ";

        let program = compile_script_with_options(program_text, &OptimizeOptions::none())
            .expect("Compilation should succeed");
        let mut vm = LpsVm::new(&program, VmLimits::default()).expect("VM creation should succeed");
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .expect("Execution should succeed");

        let expected = 5.0.to_fixed(); // (2.0 * 2.0) + 1.0 = 5.0
        let diff = (result.to_f32() - expected.to_f32()).abs();
        assert!(
            diff < 0.0001,
            "Expected {}, got {}",
            expected.to_f32(),
            result.to_f32()
        );
    }

    #[test]
    fn test_function_scalar_from_vectors() {
        let program_text = "
            float distance2D(vec2 a, vec2 b) {
                vec2 diff = b - a;
                return sqrt(diff.x * diff.x + diff.y * diff.y);
            }
            return distance2D(vec2(0.0, 0.0), vec2(3.0, 4.0));
        ";

        let program = compile_script_with_options(program_text, &OptimizeOptions::none())
            .expect("Compilation should succeed");
        let mut vm = LpsVm::new(&program, VmLimits::default()).expect("VM creation should succeed");
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .expect("Execution should succeed");

        let expected = 5.0.to_fixed(); // sqrt(3^2 + 4^2) = sqrt(25) = 5
        let diff = (result.to_f32() - expected.to_f32()).abs();
        assert!(
            diff < 0.01,
            "Expected {}, got {}",
            expected.to_f32(),
            result.to_f32()
        );
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::lpscript::vm::vm_limits::VmLimits;
    use crate::lpscript::*;
    use crate::math::{Fixed, ToFixed};

    #[test]
    fn test_function_no_params() {
        let script = "
            float get_pi() {
                return 3.14159;
            }
            return get_pi();
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert!((result.to_f32() - 3.14159).abs() < 0.01);
    }

    #[test]
    fn test_function_with_vec_params() {
        let script = "
            float vec2_sum(vec2 v) {
                return v.x + v.y;
            }
            return vec2_sum(vec2(3.0, 7.0));
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 10.0);
    }

    #[test]
    fn test_function_calling_function() {
        let script = "
            float triple(float x) {
                return x * 3.0;
            }
            
            float sextuple(float x) {
                return triple(x) * 2.0;
            }
            
            return sextuple(5.0);
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 30.0);
    }

    #[test]
    fn test_recursive_fibonacci() {
        let script = "
            float fib(float n) {
                if (n <= 1.0) {
                    return n;
                }
                return fib(n - 1.0) + fib(n - 2.0);
            }
            return fib(6.0);
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        // fib(6) = 8
        assert_eq!(result.to_f32(), 8.0);
    }

    #[test]
    fn test_function_with_local_variables() {
        let script = "
            float compute(float a, float b) {
                float sum = a + b;
                float product = a * b;
                return sum + product;
            }
            return compute(3.0, 4.0);
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        // (3 + 4) + (3 * 4) = 7 + 12 = 19
        assert_eq!(result.to_f32(), 19.0);
    }

    #[test]
    fn test_function_multiple_functions() {
        let script = "
            float add(float a, float b) {
                return a + b;
            }
            
            float multiply(float a, float b) {
                return a * b;
            }
            
            float divide(float a, float b) {
                return a / b;
            }
            
            float x = add(10.0, 5.0);        // 15
            float y = multiply(x, 2.0);      // 30
            float z = divide(y, 3.0);        // 10
            return z;
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 10.0);
    }

    #[test]
    fn test_function_with_conditional_return() {
        let script = "
            float abs_custom(float x) {
                if (x < 0.0) {
                    return 0.0 - x;
                }
                return x;
            }
            
            float a = abs_custom(-5.0);
            float b = abs_custom(3.0);
            return a + b;
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert!((result.to_f32() - 8.0).abs() < 0.1); // Relax for fixed-point precision
    }

    #[test]
    fn test_function_with_loop() {
        let script = "
            float sum_range(float n) {
                float sum = 0.0;
                for (float i = 0.0; i < n; i = i + 1.0) {
                    sum = sum + i;
                }
                return sum;
            }
            return sum_range(5.0);
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        // 0 + 1 + 2 + 3 + 4 = 10
        assert_eq!(result.to_f32(), 10.0);
    }

    #[test]
    fn test_function_vec_return() {
        let script = "
            vec2 make_vec(float x, float y) {
                return vec2(x, y);
            }
            vec2 v = make_vec(3.0, 4.0);
            return v.x + v.y;
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 7.0);
    }
}
