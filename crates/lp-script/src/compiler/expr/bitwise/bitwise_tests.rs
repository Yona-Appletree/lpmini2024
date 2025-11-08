/// Tests for bitwise operators
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_bitwise_and() -> Result<(), String> {
        ExprTest::new("12 & 10")
            .expect_ast(|b| {
                let left = b.int32(12);
                let right = b.int32(10);
                b.bitwise_and(left, right, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(12),
                LpsOpCode::PushInt32(10),
                LpsOpCode::BitwiseAndInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(8) // 12 & 10 = 8
            .run()
    }

    #[test]
    fn test_bitwise_or() -> Result<(), String> {
        ExprTest::new("12 | 10")
            .expect_ast(|b| {
                let left = b.int32(12);
                let right = b.int32(10);
                b.bitwise_or(left, right, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(12),
                LpsOpCode::PushInt32(10),
                LpsOpCode::BitwiseOrInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(14) // 12 | 10 = 14
            .run()
    }

    #[test]
    fn test_bitwise_xor() -> Result<(), String> {
        ExprTest::new("12 ^ 10")
            .expect_ast(|b| {
                let left = b.int32(12);
                let right = b.int32(10);
                b.bitwise_xor(left, right, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(12),
                LpsOpCode::PushInt32(10),
                LpsOpCode::BitwiseXorInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(6) // 12 ^ 10 = 6
            .run()
    }

    #[test]
    fn test_bitwise_not() -> Result<(), String> {
        ExprTest::new("~42")
            .expect_ast(|b| {
                let operand = b.int32(42);
                b.bitwise_not(operand, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(42),
                LpsOpCode::BitwiseNotInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(-43) // ~42 = -43
            .run()
    }

    #[test]
    fn test_left_shift() -> Result<(), String> {
        ExprTest::new("5 << 2")
            .expect_ast(|b| {
                let left = b.int32(5);
                let right: crate::compiler::ast::Expr = b.int32(2);
                b.left_shift(left, right, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(5),
                LpsOpCode::PushInt32(2),
                LpsOpCode::LeftShiftInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(20) // 5 << 2 = 20
            .run()
    }

    #[test]
    fn test_right_shift() -> Result<(), String> {
        ExprTest::new("20 >> 2")
            .expect_ast(|b| {
                let left = b.int32(20);
                let right = b.int32(2);
                b.right_shift(left, right, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(20),
                LpsOpCode::PushInt32(2),
                LpsOpCode::RightShiftInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(5) // 20 >> 2 = 5
            .run()
    }

    #[test]
    fn test_negative_right_shift() -> Result<(), String> {
        ExprTest::new("-8 >> 1")
            .expect_ast(|b| {
                let neg_eight = b.int32(-8); // Parser optimizes -8 to a single literal
                let one = b.int32(1);
                b.right_shift(neg_eight, one, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(-8),
                LpsOpCode::PushInt32(1),
                LpsOpCode::RightShiftInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(-4) // -8 >> 1 = -4 (arithmetic shift)
            .run()
    }

    #[test]
    fn test_bitwise_precedence() -> Result<(), String> {
        // Test that & has higher precedence than |
        // 12 | 8 & 4 should parse as 12 | (8 & 4) = 12 | 0 = 12
        ExprTest::new("12 | 8 & 4")
            .expect_ast(|b| {
                let eight = b.int32(8);
                let four = b.int32(4);
                let and_result = b.bitwise_and(eight, four, Type::Int32);
                let twelve = b.int32(12);
                b.bitwise_or(twelve, and_result, Type::Int32)
            })
            .expect_result_int(12)
            .run()
    }

    #[test]
    fn test_bitwise_complex() -> Result<(), String> {
        // Complex expression with multiple bitwise operators
        // (12 & 10) | (5 ^ 3) = 8 | 6 = 14
        ExprTest::new("(12 & 10) | (5 ^ 3)")
            .expect_ast(|b| {
                let twelve = b.int32(12);
                let ten = b.int32(10);
                let left = b.bitwise_and(twelve, ten, Type::Int32);
                let five = b.int32(5);
                let three = b.int32(3);
                let right = b.bitwise_xor(five, three, Type::Int32);
                b.bitwise_or(left, right, Type::Int32)
            })
            .expect_result_int(14)
            .run()
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::compile_script;
    use crate::fixed::ToFixed;
    use crate::vm::lps_vm::LpsVm;
    use crate::vm::vm_limits::VmLimits;

    #[test]
    fn test_bitwise_and_integration() {
        let script = "
            int x = 12 & 10;
            return x;
        ";
        let program = compile_script(script).unwrap();
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 8); // 12 & 10 = 8 (use .0 for raw Int32)
    }

    #[test]
    fn test_bitwise_or_integration() {
        let script = "
            int x = 12 | 10;
            return x;
        ";
        let program = compile_script(script).unwrap();
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 14); // 12 | 10 = 14
    }

    #[test]
    fn test_bitwise_xor_integration() {
        let script = "
            int x = 12 ^ 10;
            return x;
        ";
        let program = compile_script(script).unwrap();
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 6); // 12 ^ 10 = 6
    }

    #[test]
    fn test_bitwise_not_integration() {
        let script = "
            int x = ~5;
            return x;
        ";
        let program = compile_script(script).unwrap();
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, -6); // ~5 = -6
    }

    #[test]
    fn test_left_shift_integration() {
        let script = "
            int x = 5 << 2;
            return x;
        ";
        let program = compile_script(script).unwrap();
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 20); // 5 << 2 = 20
    }

    #[test]
    fn test_right_shift_integration() {
        let script = "
            int x = 20 >> 2;
            return x;
        ";
        let program = compile_script(script).unwrap();
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 5); // 20 >> 2 = 5
    }

    #[test]
    fn test_bitwise_precedence_integration() {
        let script = "
            int x = 8 & 4 << 1;
            return x;
        ";
        let program = compile_script(script).unwrap();
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        // Should be 8 & (4 << 1) = 8 & 8 = 8
        assert_eq!(result.0, 8);
    }
}
