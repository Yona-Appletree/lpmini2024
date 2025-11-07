/// While loop tests
#[cfg(test)]
mod tests {
    use crate::vm::vm_limits::VmLimits;
    use crate::*;
    use crate::fixed::Fixed;

    #[test]
    fn test_while_loop_counter() {
        let script = "float i = 0.0; while (i < 5.0) { i = i + 1.0; } return i;";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 5.0);
    }

    #[test]
    fn test_while_loop_sum() {
        let script = "float sum = 0.0; float i = 1.0; while (i <= 3.0) { sum = sum + i; i = i + 1.0; } return sum;";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 6.0); // 1 + 2 + 3
    }

    #[test]
    fn test_while_loop_with_break_condition() {
        let script = "float x = 0.0; while (x < 10.0) { x = x + 2.0; } return x;";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 10.0);
    }
}
