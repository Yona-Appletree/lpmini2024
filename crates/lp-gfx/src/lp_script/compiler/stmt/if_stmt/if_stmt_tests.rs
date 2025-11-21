/// If statement tests
#[cfg(test)]
mod tests {
    use lp_math::dec32::{Dec32, ToDec32};

    use crate::lp_script::vm::vm_limits::VmLimits;
    use crate::lp_script::*;

    #[test]
    fn test_if_without_else() {
        let script = "if (1.0 > 0.5) { return 10.0; } return 0.0;";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Dec32::ZERO, Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 10.0);
    }

    #[test]
    fn test_if_with_else() {
        let script = "if (1.0 > 0.5) { return 10.0; } else { return 20.0; }";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Dec32::ZERO, Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 10.0);
    }

    #[test]
    fn test_if_with_variable() {
        let script = "float x = 0.3; if (x > 0.5) { return 1.0; } else { return 0.0; }";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Dec32::ZERO, Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 0.0);
    }

    #[test]
    fn test_if_with_builtin() {
        let script = "if (time > 5.0) { return 100.0; } else { return -100.0; }";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Dec32::ZERO, Dec32::ZERO, 10.0.to_dec32())
            .unwrap();
        assert_eq!(result.to_f32(), 100.0);
    }

    #[test]
    fn test_nested_if_statements() {
        let script = "
            float x = uv.x;
            if (x > 0.5) {
                if (x > 0.75) {
                    return 3.0;
                } else {
                    return 2.0;
                }
            } else {
                return 1.0;
            }
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(0.9.to_dec32(), Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 3.0);

        let result = vm
            .run_scalar(0.6.to_dec32(), Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 2.0);

        let result = vm
            .run_scalar(0.3.to_dec32(), Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 1.0);
    }

    #[test]
    fn test_if_else_chain() {
        let script = "
            float x = uv.x;
            if (x < 0.25) {
                return 1.0;
            } else if (x < 0.5) {
                return 2.0;
            } else if (x < 0.75) {
                return 3.0;
            } else {
                return 4.0;
            }
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        let result = vm
            .run_scalar(0.1.to_dec32(), Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 1.0);

        let result = vm
            .run_scalar(0.3.to_dec32(), Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 2.0);

        let result = vm
            .run_scalar(0.6.to_dec32(), Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 3.0);

        let result = vm
            .run_scalar(0.9.to_dec32(), Dec32::ZERO, Dec32::ZERO)
            .unwrap();
        assert_eq!(result.to_f32(), 4.0);
    }
}
