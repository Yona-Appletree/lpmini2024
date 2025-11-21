/// Binary arithmetic operator tests for Dec32 (scalar) type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::dec32::ToDec32;
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_addition() -> Result<(), String> {
        ExprTest::new("1.0 + 2.0")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let right = b.num(2.0);
                b.add(left, right, Type::Dec32)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::AddDec32,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(3.0)
            .run()
    }

    #[test]
    fn test_subtraction() -> Result<(), String> {
        ExprTest::new("5.0 - 2.0")
            .expect_ast(|b| {
                let left = b.num(5.0);
                let right = b.num(2.0);
                b.sub(left, right, Type::Dec32)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::SubDec32,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(3.0)
            .run()
    }

    #[test]
    fn test_multiplication() -> Result<(), String> {
        ExprTest::new("3.0 * 4.0")
            .expect_ast(|b| {
                let left = b.num(3.0);
                let right = b.num(4.0);
                b.mul(left, right, Type::Dec32)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::MulDec32,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(12.0)
            .run()
    }

    #[test]
    fn test_division() -> Result<(), String> {
        ExprTest::new("10.0 / 2.0")
            .expect_ast(|b| {
                let left = b.num(10.0);
                let right = b.num(2.0);
                b.div(left, right, Type::Dec32)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::DivDec32,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(5.0)
            .run()
    }

    #[test]
    fn test_modulo() -> Result<(), String> {
        ExprTest::new("10.0 % 3.0")
            .expect_ast(|b| {
                let left = b.num(10.0);
                let right = b.num(3.0);
                b.modulo(left, right, Type::Dec32)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::ModDec32,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(1.0)
            .run()?;

        ExprTest::new("7.5 % 2.5").expect_result_dec32(0.0).run()
    }

    #[test]
    fn test_operator_precedence() -> Result<(), String> {
        // 1 + 2 * 3 should be 1 + (2 * 3) = 7
        ExprTest::new("1.0 + 2.0 * 3.0")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let mul_left = b.num(2.0);
                let mul_right = b.num(3.0);
                let right = b.mul(mul_left, mul_right, Type::Dec32);
                b.add(left, right, Type::Dec32)
            })
            .expect_result_dec32(7.0)
            .run()
    }

    #[test]
    fn test_parenthesized_expression() -> Result<(), String> {
        // (1 + 2) * 3 should be 9
        ExprTest::new("(1.0 + 2.0) * 3.0")
            .expect_ast(|b| {
                let add_left = b.num(1.0);
                let add_right = b.num(2.0);
                let left = b.add(add_left, add_right, Type::Dec32);
                let right = b.num(3.0);
                b.mul(left, right, Type::Dec32)
            })
            .expect_result_dec32(9.0)
            .run()
    }

    #[test]
    fn test_int_float_promotion() -> Result<(), String> {
        // Int + Dec32 should promote Int to Dec32
        ExprTest::new("1 + 2.0").expect_result_dec32(3.0).run()?;

        // Dec32 + Int should promote Int to Dec32
        ExprTest::new("2.0 + 1").expect_result_dec32(3.0).run()
    }

    #[test]
    fn test_power_function() -> Result<(), String> {
        // Power operator (^) has been removed, use pow() function instead
        ExprTest::new("pow(2.0, 3.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::PowDec32,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(8.0)
            .run()
    }

    #[test]
    fn test_edge_cases() -> Result<(), String> {
        // Zero
        ExprTest::new("0.0 + 5.0").expect_result_dec32(5.0).run()?;

        ExprTest::new("5.0 * 0.0").expect_result_dec32(0.0).run()?;

        // Negative numbers
        ExprTest::new("-3.0 + 5.0").expect_result_dec32(2.0).run()?;

        ExprTest::new("-2.0 * -3.0")
            .expect_result_dec32(6.0)
            .run()?;

        // Division by small number
        ExprTest::new("1.0 / 0.5").expect_result_dec32(2.0).run()
    }

    #[test]
    fn test_complex_expressions() -> Result<(), String> {
        // Multiple operations
        ExprTest::new("1.0 + 2.0 * 3.0 - 4.0")
            .expect_result_dec32(3.0) // 1 + 6 - 4 = 3
            .run()?;

        // Nested parentheses
        ExprTest::new("((1.0 + 2.0) * 3.0) / 2.0")
            .expect_result_dec32(4.5) // (3 * 3) / 2 = 4.5
            .run()
    }
}
