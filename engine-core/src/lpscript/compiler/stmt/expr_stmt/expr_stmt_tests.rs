/// Expression statement tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::stmt::stmt_test_ast::*;
    use crate::lpscript::compiler::stmt::stmt_test_util::ScriptTest;
    use crate::lpscript::error::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::math::ToFixed;

    #[test]
    fn test_expr_stmt_with_side_effect() -> Result<(), String> {
        // Expression statement for side effects (e.g., function call)
        ScriptTest::new("float x = 1.0; sin(x); return x;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(num(1.0))),
                expr_stmt(call("sin", vec![typed_var("x", Type::Fixed)], Type::Fixed)),
                return_stmt(typed_var("x", Type::Fixed)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::SinFixed,
                LpsOpCode::Drop1, // Expression statement drops result
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(1.0)
            .run()
    }

    #[test]
    fn test_multiple_expr_stmts() -> Result<(), String> {
        ScriptTest::new("1.0 + 2.0; 3.0 * 4.0; return 10.0;")
            .expect_ast(program(vec![
                expr_stmt(add(num(1.0), num(2.0), Type::Fixed)),
                expr_stmt(mul(num(3.0), num(4.0), Type::Fixed)),
                return_stmt(num(10.0)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::AddFixed,
                LpsOpCode::Drop1, // Expression statement drops result
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::MulFixed,
                LpsOpCode::Drop1, // Expression statement drops result
                LpsOpCode::Push(10.0.to_fixed()),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(10.0)
            .run()
    }

    #[test]
    fn test_expr_stmt_arithmetic() -> Result<(), String> {
        ScriptTest::new("float x = 5.0; x + 3.0; return x;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(num(5.0))),
                expr_stmt(add(typed_var("x", Type::Fixed), num(3.0), Type::Fixed)),
                return_stmt(typed_var("x", Type::Fixed)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::AddFixed,
                LpsOpCode::Drop1, // Expression statement drops result
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(5.0)
            .run()
    }
}
