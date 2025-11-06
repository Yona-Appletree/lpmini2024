/// Variable declaration tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::stmt::stmt_test_util::ScriptTest;
    use crate::lpscript::shared::Type;
    use crate::lpscript::vm::lps_vm::LpsVm;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::lpscript::vm::vm_limits::VmLimits;
    use crate::math::ToFixed;

    #[test]
    fn test_var_decl_with_init() -> Result<(), String> {
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
    }

    #[test]
    fn test_var_decl_without_init() -> Result<(), String> {
        ScriptTest::new("float x; x = 10.0; return x;")
            .expect_ast(|b| {
                let var_stmt = b.var_decl(Type::Fixed, "x", None);
                let assign_value = b.num(10.0);
                let assign_expr = b.assign("x", assign_value, Type::Fixed);
                let assign_stmt = b.expr_stmt(assign_expr);
                let ret_var = b.typed_var("x", Type::Fixed);
                let ret_stmt = b.return_stmt(ret_var);
                b.program(vec![var_stmt, assign_stmt, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_fixed()),
                LpsOpCode::Dup1,
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::Drop1,
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(10.0)
            .run()
    }

    #[test]
    fn test_multiple_var_decls() -> Result<(), String> {
        ScriptTest::new("float x = 1.0; float y = 2.0; return x + y;")
            .expect_ast(|b| {
                let x_init = b.num(1.0);
                let x_decl = b.var_decl(Type::Fixed, "x", Some(x_init));
                let y_init = b.num(2.0);
                let y_decl = b.var_decl(Type::Fixed, "y", Some(y_init));
                let x_var = b.typed_var("x", Type::Fixed);
                let y_var = b.typed_var("y", Type::Fixed);
                let add_expr = b.add(x_var, y_var, Type::Fixed);
                let ret_stmt = b.return_stmt(add_expr);
                b.program(vec![x_decl, y_decl, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(1),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::LoadLocalFixed(1),
                LpsOpCode::AddFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_var_decl_with_expression() -> Result<(), String> {
        ScriptTest::new("float x = 1.0 + 2.0; return x;")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let right = b.num(2.0);
                let init = b.add(left, right, Type::Fixed);
                let var_stmt = b.var_decl(Type::Fixed, "x", Some(init));
                let ret_var = b.typed_var("x", Type::Fixed);
                let ret_stmt = b.return_stmt(ret_var);
                b.program(vec![var_stmt, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::AddFixed,
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_var_decl_vec2() -> Result<(), String> {
        
        use crate::lpscript::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec2 v = vec2(1.0, 2.0); return v.x;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, VmLimits::default())
            .map_err(|e| format!("VM creation failed: {:?}", e))?;
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .map_err(|e| format!("Execution failed: {:?}", e))?;

        let expected = 1.0.to_fixed();
        let diff = (result.to_f32() - expected.to_f32()).abs();
        if diff > 0.0001 {
            return Err(format!(
                "Expected {}, got {}",
                expected.to_f32(),
                result.to_f32()
            ));
        }
        Ok(())
    }

    #[test]
    fn test_var_decl_vec3() -> Result<(), String> {
        
        use crate::lpscript::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec3 v = vec3(1.0, 2.0, 3.0); return v.y;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, VmLimits::default())
            .map_err(|e| format!("VM creation failed: {:?}", e))?;
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .map_err(|e| format!("Execution failed: {:?}", e))?;

        let expected = 2.0.to_fixed();
        let diff = (result.to_f32() - expected.to_f32()).abs();
        if diff > 0.0001 {
            return Err(format!(
                "Expected {}, got {}",
                expected.to_f32(),
                result.to_f32()
            ));
        }
        Ok(())
    }

    #[test]
    fn test_var_decl_vec4() -> Result<(), String> {
        
        use crate::lpscript::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec4 v = vec4(1.0, 2.0, 3.0, 4.0); return v.w;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, VmLimits::default())
            .map_err(|e| format!("VM creation failed: {:?}", e))?;
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .map_err(|e| format!("Execution failed: {:?}", e))?;

        let expected = 4.0.to_fixed();
        let diff = (result.to_f32() - expected.to_f32()).abs();
        if diff > 0.0001 {
            return Err(format!(
                "Expected {}, got {}",
                expected.to_f32(),
                result.to_f32()
            ));
        }
        Ok(())
    }

    #[test]
    fn test_vec2_in_expression() -> Result<(), String> {
        
        use crate::lpscript::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec2 a = vec2(1.0, 2.0); vec2 b = vec2(3.0, 4.0); vec2 c = a + b; return c.x;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, VmLimits::default())
            .map_err(|e| format!("VM creation failed: {:?}", e))?;
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .map_err(|e| format!("Execution failed: {:?}", e))?;

        let expected = 4.0.to_fixed(); // 1.0 + 3.0
        let diff = (result.to_f32() - expected.to_f32()).abs();
        if diff > 0.0001 {
            return Err(format!(
                "Expected {}, got {}",
                expected.to_f32(),
                result.to_f32()
            ));
        }
        Ok(())
    }

    #[test]
    fn test_vec3_in_expression() -> Result<(), String> {
        
        use crate::lpscript::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec3 a = vec3(2.0, 3.0, 4.0); vec3 b = a * 2.0; return b.z;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, VmLimits::default())
            .map_err(|e| format!("VM creation failed: {:?}", e))?;
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .map_err(|e| format!("Execution failed: {:?}", e))?;

        let expected = 8.0.to_fixed(); // 4.0 * 2.0
        let diff = (result.to_f32() - expected.to_f32()).abs();
        if diff > 0.0001 {
            return Err(format!(
                "Expected {}, got {}",
                expected.to_f32(),
                result.to_f32()
            ));
        }
        Ok(())
    }

    #[test]
    fn test_vec4_uninitialized() -> Result<(), String> {
        
        use crate::lpscript::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec4 v; v = vec4(5.0, 6.0, 7.0, 8.0); return v.y;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, VmLimits::default())
            .map_err(|e| format!("VM creation failed: {:?}", e))?;
        let result = vm
            .run_scalar(0.5.to_fixed(), 0.5.to_fixed(), 0.0.to_fixed())
            .map_err(|e| format!("Execution failed: {:?}", e))?;

        let expected = 6.0.to_fixed();
        let diff = (result.to_f32() - expected.to_f32()).abs();
        if diff > 0.0001 {
            return Err(format!(
                "Expected {}, got {}",
                expected.to_f32(),
                result.to_f32()
            ));
        }
        Ok(())
    }

    #[test]
    fn test_var_decl_from_builtin() -> Result<(), String> {
        ScriptTest::new("float t = time; return t * 2.0;")
            .expect_ast(|b| {
                let time_var = b.typed_var("time", Type::Fixed);
                let t_decl = b.var_decl(Type::Fixed, "t", Some(time_var));
                let t_var = b.typed_var("t", Type::Fixed);
                let mul_right = b.num(2.0);
                let mul_expr = b.mul(t_var, mul_right, Type::Fixed);
                let ret_stmt = b.return_stmt(mul_expr);
                b.program(vec![t_decl, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Load(crate::test_engine::LoadSource::Time),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::MulFixed,
                LpsOpCode::Return,
            ])
            .with_time(3.0)
            .expect_result_fixed(6.0)
            .run()
    }
}

#[cfg(test)]
mod variable_integration_tests {
    use crate::lpscript::vm::vm_limits::VmLimits;
    use crate::lpscript::*;
    use crate::math::Fixed;

    #[test]
    fn test_variable_mutation() {
        let script = "
            float x = 1.0;
            x = x + 2.0;
            x = x * 3.0;
            return x;
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        // (1 + 2) * 3 = 9
        assert_eq!(result.to_f32(), 9.0);
    }

    #[test]
    fn test_assignment_expression_value() {
        let script = "
            float x = 0.0;
            float y = (x = 5.0);  // Assignment returns the assigned value
            return y;
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 5.0);
    }

    #[test]
    fn test_chained_assignments() {
        let script = "
            float x = 0.0;
            float y = 0.0;
            float z = 0.0;
            z = y = x = 7.0;  // Right-associative
            return x + y + z;
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 21.0);
    }
}
