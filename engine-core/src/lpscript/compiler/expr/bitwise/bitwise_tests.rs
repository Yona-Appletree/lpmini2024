/// Tests for bitwise operators
#[cfg(test)]
mod tests {
    use crate::lpscript::compile_expr;
    use crate::lpscript::vm::{LpsOpCode, LpsVm, VmLimits};
    use crate::math::ToFixed;

    #[test]
    fn test_debug_opcodes() {
        let program = compile_expr("5 << 2").unwrap();
        println!("Opcodes for '5 << 2':");
        for (i, op) in program.opcodes.iter().enumerate() {
            println!("  {}: {}", i, op.name());
        }
    }

    #[test]
    fn test_bitwise_and() {
        let program = compile_expr("12 & 10").unwrap();
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 8); // 12 & 10 = 8 (use .0 for raw Int32 value)
    }

    #[test]
    fn test_bitwise_or() {
        let program = compile_expr("12 | 10").unwrap();
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 14); // 12 | 10 = 14
    }

    #[test]
    fn test_bitwise_xor() {
        let program = compile_expr("12 ^ 10").unwrap();
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 6); // 12 ^ 10 = 6
    }

    #[test]
    fn test_bitwise_not() {
        let program = compile_expr("~5").unwrap();
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, -6); // ~5 = -6
    }

    #[test]
    fn test_left_shift() {
        let program = compile_expr("5 << 2").unwrap();
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 20); // 5 << 2 = 20
    }

    #[test]
    fn test_right_shift() {
        let program = compile_expr("20 >> 2").unwrap();
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 5); // 20 >> 2 = 5
    }

    #[test]
    fn test_bitwise_precedence() {
        // Test that & has lower precedence than <<
        let program = compile_expr("8 & 4 << 1").unwrap();
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        // Should be 8 & (4 << 1) = 8 & 8 = 8
        assert_eq!(result.0, 8);
    }

    #[test]
    fn test_bitwise_complex() {
        // Complex expression with multiple bitwise operators
        let program = compile_expr("(12 & 10) | (5 ^ 3)").unwrap();
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        // (12 & 10) | (5 ^ 3) = 8 | 6 = 14
        assert_eq!(result.0, 14);
    }
}
