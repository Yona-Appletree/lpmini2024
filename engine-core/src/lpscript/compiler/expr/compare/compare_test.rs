/// Integration tests for comparison operators
#[cfg(test)]
mod tests {
    use crate::lpscript::*;
    use crate::math::{Fixed, ToFixed};

    #[test]
    fn test_parse_less_than() {
        let program = parse_expr("x > 5.0");
        // Should parse without panicking
        assert!(program.opcodes.len() > 0);
    }

    #[test]
    fn test_parse_all_comparisons() {
        // Test all comparison operators parse correctly
        let _ = parse_expr("1.0 < 2.0");
        let _ = parse_expr("1.0 > 2.0");
        let _ = parse_expr("1.0 <= 2.0");
        let _ = parse_expr("1.0 >= 2.0");
        let _ = parse_expr("1.0 == 2.0");
        let _ = parse_expr("1.0 != 2.0");
    }

    #[test]
    fn test_comparison_codegen() {
        use crate::lpscript::vm::opcodes::LpsOpCode;
        
        let program = parse_expr("5.0 > 3.0");
        // Should have Push, Push, GreaterFixed, Return
        let has_greater = program.opcodes.iter().any(|op| matches!(op, LpsOpCode::GreaterFixed));
        assert!(has_greater, "Should generate GreaterFixed opcode");
    }

    #[test]
    fn test_comparison_execution() {
        use crate::lpscript::vm::LpsVm;
        use crate::lpscript::vm::VmLimits;

        // Test: 0.6 > 0.5 should be 1.0 (true)
        let program = parse_expr("uv.x > 0.5");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm.run(0.6.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 1.0, "0.6 > 0.5 should be true (1.0)");

        // Test: 0.4 > 0.5 should be 0.0 (false)
        let result = vm.run(0.4.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 0.0, "0.4 > 0.5 should be false (0.0)");
    }

    #[test]
    fn test_less_than_execution() {
        use crate::lpscript::vm::LpsVm;
        use crate::lpscript::vm::VmLimits;

        let program = parse_expr("uv.x < 0.5");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        
        let result = vm.run(0.3.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 1.0, "0.3 < 0.5 should be true");
        
        let result = vm.run(0.7.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 0.0, "0.7 < 0.5 should be false");
    }

    #[test]
    fn test_equality_execution() {
        use crate::lpscript::vm::LpsVm;
        use crate::lpscript::vm::VmLimits;

        let program = parse_expr("uv.x == 0.5");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        
        let result = vm.run(0.5.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 1.0, "0.5 == 0.5 should be true");
        
        let result = vm.run(0.3.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 0.0, "0.3 == 0.5 should be false");
    }

    #[test]
    fn test_not_equal_execution() {
        use crate::lpscript::vm::LpsVm;
        use crate::lpscript::vm::VmLimits;

        let program = parse_expr("uv.x != 0.5");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        
        let result = vm.run(0.3.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 1.0, "0.3 != 0.5 should be true");
        
        let result = vm.run(0.5.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 0.0, "0.5 != 0.5 should be false");
    }

    #[test]
    fn test_comparison_chain() {
        use crate::lpscript::vm::LpsVm;
        use crate::lpscript::vm::VmLimits;

        // Test chained comparisons with ternary
        let program = parse_expr("uv.x > 0.3 && uv.x < 0.7 ? 1.0 : 0.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        
        // 0.5 is between 0.3 and 0.7
        let result = vm.run(0.5.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 1.0);
        
        // 0.2 is not between 0.3 and 0.7
        let result = vm.run(0.2.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 0.0);
    }

    #[test]
    fn test_type_checking_comparisons() {
        // Comparisons should compile successfully and type check
        let result = compile_expr("1.0 > 2.0");
        assert!(result.is_ok(), "Simple comparison should type check");
        
        let result = compile_expr("uv.x < 0.5");
        assert!(result.is_ok(), "Variable comparison should type check");
    }
}

