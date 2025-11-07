/// Block statement tests
#[cfg(test)]
mod tests {
    
    use crate::compiler::stmt::stmt_test_util::ScriptTest;
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;
    use crate::math::ToFixed;

    #[test]
    fn test_simple_block() -> Result<(), String> {
        ScriptTest::new("{ float x = 5.0; return x; }")
            .expect_ast(|b| {
                let init = b.num(5.0);
                let var_stmt = b.var_decl(Type::Fixed, "x", Some(init));
                let ret_var = b.typed_var("x", Type::Fixed);
                let ret_stmt = b.return_stmt(ret_var);
                let block_stmt = b.block(vec![var_stmt, ret_stmt]);
                b.program(vec![block_stmt])
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
    fn test_nested_blocks() -> Result<(), String> {
        ScriptTest::new("{ float x = 1.0; { float y = 2.0; x = x + y; } return x; }")
            .expect_ast(|b| {
                let x_init = b.num(1.0);
                let x_decl = b.var_decl(Type::Fixed, "x", Some(x_init));
                
                let y_init = b.num(2.0);
                let y_decl = b.var_decl(Type::Fixed, "y", Some(y_init));
                let x_var = b.typed_var("x", Type::Fixed);
                let y_var = b.typed_var("y", Type::Fixed);
                let add_expr = b.add(x_var, y_var, Type::Fixed);
                let assign_expr = b.assign("x", add_expr, Type::Fixed);
                let assign_stmt = b.expr_stmt(assign_expr);
                let inner_block = b.block(vec![y_decl, assign_stmt]);
                
                let ret_var = b.typed_var("x", Type::Fixed);
                let ret_stmt = b.return_stmt(ret_var);
                let outer_block = b.block(vec![x_decl, inner_block, ret_stmt]);
                b.program(vec![outer_block])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(1),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::LoadLocalFixed(1),
                LpsOpCode::AddFixed,
                LpsOpCode::Dup1,
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::Drop1,
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_block_with_multiple_statements() -> Result<(), String> {
        ScriptTest::new("{ float a = 1.0; float b = 2.0; float c = a + b; return c; }")
            .expect_ast(|b| {
                let a_init = b.num(1.0);
                let a_decl = b.var_decl(Type::Fixed, "a", Some(a_init));
                let b_init = b.num(2.0);
                let b_decl = b.var_decl(Type::Fixed, "b", Some(b_init));
                let a_var = b.typed_var("a", Type::Fixed);
                let b_var = b.typed_var("b", Type::Fixed);
                let c_init = b.add(a_var, b_var, Type::Fixed);
                let c_decl = b.var_decl(Type::Fixed, "c", Some(c_init));
                let ret_var = b.typed_var("c", Type::Fixed);
                let ret_stmt = b.return_stmt(ret_var);
                let block_stmt = b.block(vec![a_decl, b_decl, c_decl, ret_stmt]);
                b.program(vec![block_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(1),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::LoadLocalFixed(1),
                LpsOpCode::AddFixed,
                LpsOpCode::StoreLocalFixed(2),
                LpsOpCode::LoadLocalFixed(2),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
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
            .expect_opcodes(vec![LpsOpCode::Push(42.0.to_fixed()), LpsOpCode::Return])
            .expect_result_fixed(42.0)
            .run()
    }
}

#[cfg(test)]
mod scoping_integration_tests {
    use crate::vm::vm_limits::VmLimits;
    use crate::*;
    use crate::math::Fixed;

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
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
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
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        // 1 + 2 + 3 = 6
        assert_eq!(result.to_f32(), 6.0);
    }
}
