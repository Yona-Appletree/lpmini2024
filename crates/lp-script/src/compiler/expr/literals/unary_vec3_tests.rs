/// Unary negation tests for Vec3 type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::fixed::{ToFixed, Vec3};
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_negation() -> Result<(), String> {
        ExprTest::new("-vec3(1.0, 2.0, 3.0)")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let arg3 = b.num(3.0);
                let operand = b.vec3(vec![arg1, arg2, arg3]);
                b.neg(operand, Type::Vec3)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::NegVec3,
                LpsOpCode::Return,
            ])
            .expect_result_vec3(Vec3::new(-1.0.to_fixed(), -2.0.to_fixed(), -3.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_negation_zero() -> Result<(), String> {
        ExprTest::new("-vec3(0.0, 0.0, 0.0)")
            .expect_result_vec3(Vec3::new(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_negation_negative() -> Result<(), String> {
        ExprTest::new("--vec3(1.0, 2.0, 3.0)")
            .expect_result_vec3(Vec3::new(1.0.to_fixed(), 2.0.to_fixed(), 3.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_negation_with_expression() -> Result<(), String> {
        ExprTest::new("-(vec3(1.0, 2.0, 3.0) + vec3(4.0, 5.0, 6.0))")
            .expect_result_vec3(Vec3::new(-5.0.to_fixed(), -7.0.to_fixed(), -9.0.to_fixed()))
            .run()
    }
}
