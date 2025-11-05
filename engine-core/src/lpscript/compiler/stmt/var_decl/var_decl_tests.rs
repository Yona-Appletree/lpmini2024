/// Variable declaration tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::stmt::stmt_test_ast::*;
    use crate::lpscript::compiler::stmt::stmt_test_util::ScriptTest;
    use crate::lpscript::shared::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::math::ToFixed;

    #[test]
    fn test_var_decl_with_init() -> Result<(), String> {
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
    }

    #[test]
    fn test_var_decl_without_init() -> Result<(), String> {
        ScriptTest::new("float x; x = 10.0; return x;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", None),
                expr_stmt(assign("x", num(10.0), Type::Fixed)),
                return_stmt(typed_var("x", Type::Fixed)),
            ]))
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
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(num(1.0))),
                var_decl(Type::Fixed, "y", Some(num(2.0))),
                return_stmt(add(
                    typed_var("x", Type::Fixed),
                    typed_var("y", Type::Fixed),
                    Type::Fixed,
                )),
            ]))
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
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(add(num(1.0), num(2.0), Type::Fixed))),
                return_stmt(typed_var("x", Type::Fixed)),
            ]))
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
        use crate::lpscript::vm::{LpsVm, VmLimits};
        use crate::lpscript::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec2 v = vec2(1.0, 2.0); return v.x;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default())
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
        use crate::lpscript::vm::{LpsVm, VmLimits};
        use crate::lpscript::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec3 v = vec3(1.0, 2.0, 3.0); return v.y;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default())
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
        use crate::lpscript::vm::{LpsVm, VmLimits};
        use crate::lpscript::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec4 v = vec4(1.0, 2.0, 3.0, 4.0); return v.w;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default())
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
        use crate::lpscript::vm::{LpsVm, VmLimits};
        use crate::lpscript::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec2 a = vec2(1.0, 2.0); vec2 b = vec2(3.0, 4.0); vec2 c = a + b; return c.x;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default())
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
        use crate::lpscript::vm::{LpsVm, VmLimits};
        use crate::lpscript::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec3 a = vec3(2.0, 3.0, 4.0); vec3 b = a * 2.0; return b.z;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default())
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
        use crate::lpscript::vm::{LpsVm, VmLimits};
        use crate::lpscript::{compile_script_with_options, OptimizeOptions};

        let program = compile_script_with_options(
            "vec4 v; v = vec4(5.0, 6.0, 7.0, 8.0); return v.y;",
            &OptimizeOptions::none(),
        )
        .map_err(|e| format!("Compilation failed: {}", e))?;
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default())
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
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "t", Some(typed_var("time", Type::Fixed))),
                return_stmt(mul(typed_var("t", Type::Fixed), num(2.0), Type::Fixed)),
            ]))
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
