/// Return statement tests
#[cfg(test)]
mod tests {

    use crate::compiler::stmt::stmt_test_util::ScriptTest;
    use crate::dec32::ToDec32;
    use crate::shared::Type;
    use crate::vm::opcodes::load::LoadSource;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_return_literal() -> Result<(), String> {
        ScriptTest::new("return 42.0;")
            .expect_ast(|b| {
                let expr = b.num(42.0);
                let stmt = b.return_stmt(expr);
                b.program(vec![stmt])
            })
            .expect_opcodes(vec![LpsOpCode::Push(42.0.to_dec32()), LpsOpCode::Return])
            .expect_result_dec32(42.0)
            .run()
    }

    #[test]
    fn test_return_expression() -> Result<(), String> {
        ScriptTest::new("return 1.0 + 2.0 * 3.0;")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let mul_left = b.num(2.0);
                let mul_right = b.num(3.0);
                let right = b.mul(mul_left, mul_right, Type::Dec32);
                let add_expr = b.add(left, right, Type::Dec32);
                let stmt = b.return_stmt(add_expr);
                b.program(vec![stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::MulDec32,
                LpsOpCode::AddDec32,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(7.0)
            .run()
    }

    #[test]
    fn test_return_variable() -> Result<(), String> {
        ScriptTest::new("float x = 10.0; return x;")
            .expect_ast(|b| {
                let init = b.num(10.0);
                let var_stmt = b.var_decl(Type::Dec32, "x", Some(init));
                let ret_var = b.typed_var("x", Type::Dec32);
                let ret_stmt = b.return_stmt(ret_var);
                b.program(vec![var_stmt, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_dec32()),
                LpsOpCode::StoreLocalDec32(0),
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::Return,
            ])
            .expect_result_dec32(10.0)
            .run()
    }

    #[test]
    fn test_return_builtin() -> Result<(), String> {
        ScriptTest::new("return time;")
            .expect_ast(|b| {
                let time_var = b.typed_var("time", Type::Dec32);
                let stmt = b.return_stmt(time_var);
                b.program(vec![stmt])
            })
            .expect_opcodes(vec![LpsOpCode::Load(LoadSource::Time), LpsOpCode::Return])
            .with_time(5.0)
            .expect_result_dec32(5.0)
            .run()
    }

    #[test]
    fn test_return_function_call() -> Result<(), String> {
        ScriptTest::new("return sin(0.0);")
            .expect_ast(|b| {
                let arg = b.num(0.0);
                let call_expr = b.call("sin", vec![arg], Type::Dec32);
                let stmt = b.return_stmt(call_expr);
                b.program(vec![stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::SinDec32,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(0.0)
            .run()
    }

    #[test]
    fn test_return_after_statements() -> Result<(), String> {
        ScriptTest::new("float a = 5.0; float b = 3.0; return a - b;")
            .expect_ast(|b| {
                let a_init = b.num(5.0);
                let a_decl = b.var_decl(Type::Dec32, "a", Some(a_init));
                let b_init = b.num(3.0);
                let b_decl = b.var_decl(Type::Dec32, "b", Some(b_init));
                let a_var = b.typed_var("a", Type::Dec32);
                let b_var = b.typed_var("b", Type::Dec32);
                let sub_expr = b.sub(a_var, b_var, Type::Dec32);
                let ret_stmt = b.return_stmt(sub_expr);
                b.program(vec![a_decl, b_decl, ret_stmt])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_dec32()),
                LpsOpCode::StoreLocalDec32(0),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::StoreLocalDec32(1),
                LpsOpCode::LoadLocalDec32(0),
                LpsOpCode::LoadLocalDec32(1),
                LpsOpCode::SubDec32,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(2.0)
            .run()
    }
}
