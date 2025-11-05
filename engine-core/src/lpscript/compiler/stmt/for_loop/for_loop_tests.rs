/// For loop tests
#[cfg(test)]
mod tests {
    use crate::lpscript::vm::VmLimits;
    use crate::lpscript::*;
    use crate::math::{Fixed, ToFixed};

    #[test]
    fn test_for_loop_basic() {
        let script = "float sum = 0.0; for (float i = 0.0; i < 3.0; i = i + 1.0) { sum = sum + i; } return sum;";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 3.0); // 0 + 1 + 2
    }

    #[test]
    fn test_for_loop_no_init() {
        let script = "float i = 0.0; for (; i < 3.0; i = i + 1.0) { } return i;";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 3.0);
    }

    #[test]
    fn test_for_loop_nested() {
        let script = "
            float sum = 0.0;
            for (float i = 0.0; i < 2.0; i = i + 1.0) {
                for (float j = 0.0; j < 2.0; j = j + 1.0) {
                    sum = sum + 1.0;
                }
            }
            return sum;
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 4.0); // 2 * 2 iterations
    }

    #[test]
    fn test_for_loop_with_builtin() {
        let script = "float result = 0.0; for (float i = 0.0; i < time; i = i + 1.0) { result = result + 1.0; } return result;";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, 5.0.to_fixed())
            .unwrap();
        assert_eq!(result.to_f32(), 5.0);
    }
}
