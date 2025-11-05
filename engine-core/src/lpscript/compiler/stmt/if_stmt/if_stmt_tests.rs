/// If statement tests
#[cfg(test)]
mod tests {
    use crate::lpscript::vm::VmLimits;
    use crate::lpscript::*;
    use crate::math::{Fixed, ToFixed};

    #[test]
    fn test_if_without_else() {
        let script = "if (1.0 > 0.5) { return 10.0; } return 0.0;";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 10.0);
    }

    #[test]
    fn test_if_with_else() {
        let script = "if (1.0 > 0.5) { return 10.0; } else { return 20.0; }";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 10.0);
    }

    #[test]
    fn test_if_with_variable() {
        let script = "float x = 0.3; if (x > 0.5) { return 1.0; } else { return 0.0; }";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 0.0);
    }

    #[test]
    fn test_if_with_builtin() {
        let script = "if (time > 5.0) { return 100.0; } else { return -100.0; }";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, 10.0.to_fixed())
            .unwrap();
        assert_eq!(result.to_f32(), 100.0);
    }
}
