/// Block statement tests
#[cfg(test)]
mod tests {

    use lp_math::dec32::ToDec32;

    use crate::lp_script::compiler::stmt::stmt_test_util::ScriptTest;
    use crate::lp_script::shared::Type;
    use crate::lp_script::vm::opcodes::LpsOpCode;

    #[test]
    fn test_simple_block() -> Result<(), String> {
        ScriptTest::new("{ float x = 5.0; return x; }")
            .expect_ast(|b| {
                let init = b.num(5.0);
                let var_stmt = b.var_decl(Type::Dec32, "x", Some(init));
                let ret_var = b.typed_var("x", Type::Dec32);
                let ret_stmt = b.return_stmt(ret_var);
                let block_stmt = b.block(vec![var_stmt, ret_stmt]);
                b.program(vec![block_stmt])
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
    fn test_nested_blocks() -> Result<(), String> {
        ScriptTest::new("{ float x = 1.0; { float y = 2.0; x = x + y; } return x; }")
            .expect_ast(|b| {
                let x_init = b.num(1.0);
                let x_decl = b.var_decl(Type::Dec32, "x", Some(x_init));

                let y_init = b.num(2.0);
                let y_decl = b.var_decl(Type::Dec32, "y", Some(y_init));
                let x_var = b.typed_var("x", Type::Dec32);
                let y_var = b.typed_var("y", Type::Dec32);
                let add_expr = b.add(x_var, y_var, Type::Dec32);
                let assign_expr = b.assign("x", add_expr, Type::Dec32);
                let assign_stmt = b.expr_stmt(assign_expr);
                let inner_block = b.block(vec![y_decl, assign_stmt]);

                let ret_var = b.typed_var("x", Type::Dec32);
                let ret_stmt = b.return_stmt(ret_var);
                let outer_block = b.block(vec![x_decl, inner_block, ret_stmt]);
                b.program(vec![outer_block])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::StoreLocalDec32(0),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::StoreLocalDec32(1),
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::LoadLocalDec32(1),
                LpsOpCode::AddDec32,
                LpsOpCode::Dup1,
                LpsOpCode::StoreLocalDec32(0),
                LpsOpCode::Drop1,
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::Return,
            ])
            .expect_result_dec32(3.0)
            .run()
    }

    #[test]
    fn test_block_with_multiple_statements() -> Result<(), String> {
        ScriptTest::new("{ float a = 1.0; float b = 2.0; float c = a + b; return c; }")
            .expect_ast(|b| {
                let a_init = b.num(1.0);
                let a_decl = b.var_decl(Type::Dec32, "a", Some(a_init));
                let b_init = b.num(2.0);
                let b_decl = b.var_decl(Type::Dec32, "b", Some(b_init));
                let a_var = b.typed_var("a", Type::Dec32);
                let b_var = b.typed_var("b", Type::Dec32);
                let c_init = b.add(a_var, b_var, Type::Dec32);
                let c_decl = b.var_decl(Type::Dec32, "c", Some(c_init));
                let ret_var = b.typed_var("c", Type::Dec32);
                let ret_stmt = b.return_stmt(ret_var);
                let block_stmt = b.block(vec![a_decl, b_decl, c_decl, ret_stmt]);
                b.program(vec![block_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::StoreLocalDec32(0),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::StoreLocalDec32(1),
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::LoadLocalDec32(1),
                LpsOpCode::AddDec32,
                LpsOpCode::StoreLocalDec32(2),
                LpsOpCode::LoadLocalDec32(2),
                LpsOpCode::Return,
            ])
            .expect_result_dec32(3.0)
            .run()
    }

    #[test]
    fn test_empty_block() -> Result<(), String> {
        ScriptTest::new("{ } return 42.0;")
            .expect_ast(|b| {
                let block_stmt = b.block(vec![]);
                let ret_expr = b.num(42.0);
                let ret_stmt = b.return_stmt(ret_expr);
                b.program(vec![block_stmt, ret_stmt])
            })
            .expect_opcodes(vec![LpsOpCode::Push(42.0.to_dec32()), LpsOpCode::Return])
            .expect_result_dec32(42.0)
            .run()
    }
}

#[cfg(test)]
mod scoping_integration_tests {
    use lp_math::dec32::Dec32;

    use crate::lp_script::vm::vm_limits::VmLimits;
    use crate::lp_script::*;

    #[test]
    fn test_block_scoping() {
        let script = "
            float x = 1.0;
            {
                float x = 2.0;
                x = x + 10.0;  // Inner x becomes 12
            }
            return x;  // Outer x is still 1
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Dec32::ZERO, Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 1.0);
    }

    #[test]
    fn test_nested_scopes() {
        let script = "
            float x = 1.0;
            {
                float y = 2.0;
                {
                    float z = 3.0;
                    x = x + y + z;  // Can access outer variables
                }
            }
            return x;
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(Dec32::ZERO, Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        // 1 + 2 + 3 = 6
        assert_eq!(result.to_f32(), 6.0);
    }
}
