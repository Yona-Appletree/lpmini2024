/// Binary arithmetic operator tests for Fixed (scalar) type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::fixed::ToFixed;
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_addition() -> Result<(), String> {
        ExprTest::new("1.0 + 2.0")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let right = b.num(2.0);
                b.add(left, right, Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::AddFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_subtraction() -> Result<(), String> {
        ExprTest::new("5.0 - 2.0")
            .expect_ast(|b| {
                let left = b.num(5.0);
                let right = b.num(2.0);
                b.sub(left, right, Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::SubFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_multiplication() -> Result<(), String> {
        ExprTest::new("3.0 * 4.0")
            .expect_ast(|b| {
                let left = b.num(3.0);
                let right = b.num(4.0);
                b.mul(left, right, Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::MulFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(12.0)
            .run()
    }

    #[test]
    fn test_division() -> Result<(), String> {
        ExprTest::new("10.0 / 2.0")
            .expect_ast(|b| {
                let left = b.num(10.0);
                let right = b.num(2.0);
                b.div(left, right, Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::DivFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(5.0)
            .run()
    }

    #[test]
    fn test_modulo() -> Result<(), String> {
        ExprTest::new("10.0 % 3.0")
            .expect_ast(|b| {
                let left = b.num(10.0);
                let right = b.num(3.0);
                b.modulo(left, right, Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::ModFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(1.0)
            .run()?;

        ExprTest::new("7.5 % 2.5").expect_result_fixed(0.0).run()
    }

    #[test]
    fn test_operator_precedence() -> Result<(), String> {
        // 1 + 2 * 3 should be 1 + (2 * 3) = 7
        ExprTest::new("1.0 + 2.0 * 3.0")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let mul_left = b.num(2.0);
                let mul_right = b.num(3.0);
                let right = b.mul(mul_left, mul_right, Type::Fixed);
                b.add(left, right, Type::Fixed)
            })
            .expect_result_fixed(7.0)
            .run()
    }

    #[test]
    fn test_parenthesized_expression() -> Result<(), String> {
        // (1 + 2) * 3 should be 9
        ExprTest::new("(1.0 + 2.0) * 3.0")
            .expect_ast(|b| {
                let add_left = b.num(1.0);
                let add_right = b.num(2.0);
                let left = b.add(add_left, add_right, Type::Fixed);
                let right = b.num(3.0);
                b.mul(left, right, Type::Fixed)
            })
            .expect_result_fixed(9.0)
            .run()
    }

    #[test]
    fn test_int_float_promotion() -> Result<(), String> {
        // Int + Fixed should promote Int to Fixed
        ExprTest::new("1 + 2.0").expect_result_fixed(3.0).run()?;

        // Fixed + Int should promote Int to Fixed
        ExprTest::new("2.0 + 1").expect_result_fixed(3.0).run()
    }

    #[test]
    fn test_power_function() -> Result<(), String> {
        // Power operator (^) has been removed, use pow() function instead
        ExprTest::new("pow(2.0, 3.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::PowFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(8.0)
            .run()
    }

    #[test]
    fn test_edge_cases() -> Result<(), String> {
        // Zero
        ExprTest::new("0.0 + 5.0").expect_result_fixed(5.0).run()?;

        ExprTest::new("5.0 * 0.0").expect_result_fixed(0.0).run()?;

        // Negative numbers
        ExprTest::new("-3.0 + 5.0").expect_result_fixed(2.0).run()?;

        ExprTest::new("-2.0 * -3.0")
            .expect_result_fixed(6.0)
            .run()?;

        // Division by small number
        ExprTest::new("1.0 / 0.5").expect_result_fixed(2.0).run()
    }

    #[test]
    fn test_complex_expressions() -> Result<(), String> {
        // Multiple operations
        ExprTest::new("1.0 + 2.0 * 3.0 - 4.0")
            .expect_result_fixed(3.0) // 1 + 6 - 4 = 3
            .run()?;

        // Nested parentheses
        ExprTest::new("((1.0 + 2.0) * 3.0) / 2.0")
            .expect_result_fixed(4.5) // (3 * 3) / 2 = 4.5
            .run()
    }
}
