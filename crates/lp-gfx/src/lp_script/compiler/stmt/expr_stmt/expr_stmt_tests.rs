/// Expression statement tests
#[cfg(test)]
mod tests {

    use lp_math::dec32::ToDec32;

    use crate::lp_script::compiler::stmt::stmt_test_util::ScriptTest;
    use crate::lp_script::shared::Type;
    use crate::lp_script::vm::opcodes::LpsOpCode;

    #[test]
    fn test_expr_stmt_with_side_effect() -> Result<(), String> {
        // Expression statement for side effects (e.g., function call)
        ScriptTest::new("float x = 1.0; sin(x); return x;")
            .expect_ast(|b| {
                let init = b.num(1.0);
                let var_stmt = b.var_decl(Type::Dec32, "x", Some(init));
                let call_arg = b.typed_var("x", Type::Dec32);
                let call_expr = b.call("sin", vec![call_arg], Type::Dec32);
                let call_stmt = b.expr_stmt(call_expr);
                let ret_var = b.typed_var("x", Type::Dec32);
                let ret_stmt = b.return_stmt(ret_var);
                b.program(vec![var_stmt, call_stmt, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::StoreLocalDec32(0),
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::SinDec32,
                LpsOpCode::Drop1, // Expression statement drops result
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::Return,
            ])
            .expect_result_dec32(1.0)
            .run()
    }

    #[test]
    fn test_multiple_expr_stmts() -> Result<(), String> {
        ScriptTest::new("1.0 + 2.0; 3.0 * 4.0; return 10.0;")
            .expect_ast(|b| {
                let add_left = b.num(1.0);
                let add_right = b.num(2.0);
                let add_expr = b.add(add_left, add_right, Type::Dec32);
                let add_stmt = b.expr_stmt(add_expr);
                let mul_left = b.num(3.0);
                let mul_right = b.num(4.0);
                let mul_expr = b.mul(mul_left, mul_right, Type::Dec32);
                let mul_stmt = b.expr_stmt(mul_expr);
                let ret_expr = b.num(10.0);
                let ret_stmt = b.return_stmt(ret_expr);
                b.program(vec![add_stmt, mul_stmt, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::AddDec32,
                LpsOpCode::Drop1, // Expression statement drops result
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::MulDec32,
                LpsOpCode::Drop1, // Expression statement drops result
                LpsOpCode::Push(10.0.to_dec32()),
                LpsOpCode::Return,
            ])
            .expect_result_dec32(10.0)
            .run()
    }

    #[test]
    fn test_expr_stmt_arithmetic() -> Result<(), String> {
        ScriptTest::new("float x = 5.0; x + 3.0; return x;")
            .expect_ast(|b| {
                let init = b.num(5.0);
                let var_stmt = b.var_decl(Type::Dec32, "x", Some(init));
                let x_var = b.typed_var("x", Type::Dec32);
                let add_right = b.num(3.0);
                let add_expr = b.add(x_var, add_right, Type::Dec32);
                let expr_stmt = b.expr_stmt(add_expr);
                let ret_var = b.typed_var("x", Type::Dec32);
                let ret_stmt = b.return_stmt(ret_var);
                b.program(vec![var_stmt, expr_stmt, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_dec32()),
                LpsOpCode::StoreLocalDec32(0),
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::AddDec32,
                LpsOpCode::Drop1, // Expression statement drops result
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::Return,
            ])
            .expect_result_dec32(5.0)
            .run()
    }
}
