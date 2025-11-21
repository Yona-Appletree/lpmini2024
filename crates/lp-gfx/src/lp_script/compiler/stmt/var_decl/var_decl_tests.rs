/// Variable declaration tests
#[cfg(test)]
mod tests {
    use lp_math::dec32::ToDec32;

    use crate::lp_script::compiler::stmt::stmt_test_util::ScriptTest;
    use crate::lp_script::shared::Type;
    use crate::lp_script::vm::lps_vm::LpsVm;
    use crate::lp_script::vm::opcodes::LpsOpCode;
    use crate::lp_script::vm::vm_limits::VmLimits;

    #[test]
    fn test_var_decl_with_init() -> Result<(), String> {
        ScriptTest::new("float x = 5.0; return x;")
            .expect_ast(|b| {
                let init = b.num(5.0);
                let var_stmt = b.var_decl(Type::Dec32, "x", Some(init));
                let ret_var = b.typed_var("x", Type::Dec32);
                let ret_stmt = b.return_stmt(ret_var);
                b.program(vec![var_stmt, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_dec32()),
                LpsOpCode::StoreLocalDec32(0),
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::Return,
            ])
            .expect_result_dec32(5.0)
            .run()
    }

    #[test]
    fn test_var_decl_without_init() -> Result<(), String> {
        ScriptTest::new("float x; x = 10.0; return x;")
            .expect_ast(|b| {
                let var_stmt = b.var_decl(Type::Dec32, "x", None);
                let assign_value = b.num(10.0);
                let assign_expr = b.assign("x", assign_value, Type::Dec32);
                let assign_stmt = b.expr_stmt(assign_expr);
                let ret_var = b.typed_var("x", Type::Dec32);
                let ret_stmt = b.return_stmt(ret_var);
                b.program(vec![var_stmt, assign_stmt, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_dec32()),
                LpsOpCode::Dup1,
                LpsOpCode::StoreLocalDec32(0),
                LpsOpCode::Drop1,
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::Return,
            ])
            .expect_result_dec32(10.0)
            .run()
    }

    #[test]
    fn test_multiple_var_decls() -> Result<(), String> {
        ScriptTest::new("float x = 1.0; float y = 2.0; return x + y;")
            .expect_ast(|b| {
                let x_init = b.num(1.0);
                let x_decl = b.var_decl(Type::Dec32, "x", Some(x_init));
                let y_init = b.num(2.0);
                let y_decl = b.var_decl(Type::Dec32, "y", Some(y_init));
                let x_var = b.typed_var("x", Type::Dec32);
                let y_var = b.typed_var("y", Type::Dec32);
                let add_expr = b.add(x_var, y_var, Type::Dec32);
                let ret_stmt = b.return_stmt(add_expr);
                b.program(vec![x_decl, y_decl, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::StoreLocalDec32(0),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::StoreLocalDec32(1),
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::LoadLocalDec32(1),
                LpsOpCode::AddDec32,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(3.0)
            .run()
    }

    #[test]
    fn test_var_decl_with_expression() -> Result<(), String> {
        ScriptTest::new("float x = 1.0 + 2.0; return x;")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let right = b.num(2.0);
                let init = b.add(left, right, Type::Dec32);
                let var_stmt = b.var_decl(Type::Dec32, "x", Some(init));
                let ret_var = b.typed_var("x", Type::Dec32);
                let ret_stmt = b.return_stmt(ret_var);
                b.program(vec![var_stmt, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::AddDec32,
                LpsOpCode::StoreLocalDec32(0),
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::Return,
            ])
            .expect_result_dec32(3.0)
            .run()
    }

    #[test]
    fn test_var_decl_vec2() -> Result<(), String> {
        use crate::lp_script::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec2 v = vec2(1.0, 2.0); return v.x;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, VmLimits::default())
            .map_err(|e| format!("VM creation failed: {:?}", e))?;
        let result = vm
            .run_scalar(0.5.to_dec32(), 0.5.to_dec32(), 0.0.to_dec32())
            .map_err(|e| format!("Execution failed: {:?}", e))?;

        let expected = 1.0.to_dec32();
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
        use crate::lp_script::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec3 v = vec3(1.0, 2.0, 3.0); return v.y;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, VmLimits::default())
            .map_err(|e| format!("VM creation failed: {:?}", e))?;
        let result = vm
            .run_scalar(0.5.to_dec32(), 0.5.to_dec32(), 0.0.to_dec32())
            .map_err(|e| format!("Execution failed: {:?}", e))?;

        let expected = 2.0.to_dec32();
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
        use crate::lp_script::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec4 v = vec4(1.0, 2.0, 3.0, 4.0); return v.w;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, VmLimits::default())
            .map_err(|e| format!("VM creation failed: {:?}", e))?;
        let result = vm
            .run_scalar(0.5.to_dec32(), 0.5.to_dec32(), 0.0.to_dec32())
            .map_err(|e| format!("Execution failed: {:?}", e))?;

        let expected = 4.0.to_dec32();
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
        use crate::lp_script::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec2 a = vec2(1.0, 2.0); vec2 b = vec2(3.0, 4.0); vec2 c = a + b; return c.x;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, VmLimits::default())
            .map_err(|e| format!("VM creation failed: {:?}", e))?;
        let result = vm
            .run_scalar(0.5.to_dec32(), 0.5.to_dec32(), 0.0.to_dec32())
            .map_err(|e| format!("Execution failed: {:?}", e))?;

        let expected = 4.0.to_dec32(); // 1.0 + 3.0
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
        use crate::lp_script::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec3 a = vec3(2.0, 3.0, 4.0); vec3 b = a * 2.0; return b.z;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, VmLimits::default())
            .map_err(|e| format!("VM creation failed: {:?}", e))?;
        let result = vm
            .run_scalar(0.5.to_dec32(), 0.5.to_dec32(), 0.0.to_dec32())
            .map_err(|e| format!("Execution failed: {:?}", e))?;

        let expected = 8.0.to_dec32(); // 4.0 * 2.0
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
        use crate::lp_script::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec4 v; v = vec4(5.0, 6.0, 7.0, 8.0); return v.y;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, VmLimits::default())
            .map_err(|e| format!("VM creation failed: {:?}", e))?;
        let result = vm
            .run_scalar(0.5.to_dec32(), 0.5.to_dec32(), 0.0.to_dec32())
            .map_err(|e| format!("Execution failed: {:?}", e))?;

        let expected = 6.0.to_dec32();
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
                let time_var = b.typed_var("time", Type::Dec32);
                let t_decl = b.var_decl(Type::Dec32, "t", Some(time_var));
                let t_var = b.typed_var("t", Type::Dec32);
                let mul_right = b.num(2.0);
                let mul_expr = b.mul(t_var, mul_right, Type::Dec32);
                let ret_stmt = b.return_stmt(mul_expr);
                b.program(vec![t_decl, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Load(crate::lp_script::vm::opcodes::load::LoadSource::Time),
                LpsOpCode::StoreLocalDec32(0),
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::MulDec32,
                LpsOpCode::Return,
            ])
            .with_time(3.0)
            .expect_result_dec32(6.0)
            .run()
    }
}

#[cfg(test)]
mod variable_integration_tests {
    use lp_math::dec32::Dec32;

    use crate::lp_script::vm::vm_limits::VmLimits;
    use crate::lp_script::*;

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
            .run_scalar(Dec32::ZERO, Dec32::ZERO, Dec32::ZERO)
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
            .run_scalar(Dec32::ZERO, Dec32::ZERO, Dec32::ZERO)
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
            .run_scalar(Dec32::ZERO, Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 21.0);
    }
}
