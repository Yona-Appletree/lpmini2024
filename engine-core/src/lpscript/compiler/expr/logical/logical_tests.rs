/// Logical operator tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::expr::expr_test_util::ExprTest;
    use crate::lpscript::compiler::test_ast::*;
    use crate::lpscript::error::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::math::ToFixed;

    #[test]
    fn test_logical_and() -> Result<(), String> {
        ExprTest::new("1.0 > 0.5 && 2.0 < 1.5")
            .expect_ast(and(
                greater(num(1.0), num(0.5)),
                less(num(2.0), num(1.5)),
                Type::Bool,
            ))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(0.5.to_fixed()),
                LpsOpCode::GreaterFixed,
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(1.5.to_fixed()),
                LpsOpCode::LessFixed,
                LpsOpCode::AndFixed,
                LpsOpCode::Return,
            ])
            .expect_result_bool(false) // true && false = false
            .run()
    }

    #[test]
    fn test_logical_or() -> Result<(), String> {
        ExprTest::new("1.0 < 0.5 || 2.0 > 1.5")
            .expect_ast(or(
                less(num(1.0), num(0.5)),
                greater(num(2.0), num(1.5)),
                Type::Bool,
            ))
            .expect_result_bool(true) // false || true = true
            .run()
    }

    #[test]
    fn test_logical_precedence() -> Result<(), String> {
        // OR has lower precedence than AND
        // true || false && false should be true || (false && false) = true
        ExprTest::new("1.0 > 0.0 || 0.0 > 1.0 && 0.0 > 1.0")
            .expect_result_bool(true)
            .run()
    }

    #[test]
    fn test_logical_with_comparisons() -> Result<(), String> {
        ExprTest::new("(1.0 == 1.0) && (2.0 != 3.0)")
            .expect_result_bool(true)
            .run()?;

        ExprTest::new("(1.0 != 1.0) || (2.0 == 2.0)")
            .expect_result_bool(true)
            .run()
    }
}
