use crate::lpscript::vm::VmLimits;
/// Tests for variable declarations, scoping, and mutations
use crate::lpscript::*;
use crate::math::{Fixed, ToFixed};

#[test]
fn test_variable_declaration_with_init() {
    let script = "
        float x = 5.0;
        return x;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    assert_eq!(result.to_f32(), 5.0);
}

#[test]
fn test_variable_mutation() {
    let script = "
        float x = 1.0;
        x = x + 2.0;
        x = x * 3.0;
        return x;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    // (1 + 2) * 3 = 9
    assert_eq!(result.to_f32(), 9.0);
}

#[test]
fn test_block_scoping() {
    let script = "
        float x = 1.0;
        {
            float x = 2.0;
            x = x + 10.0;  // Inner x becomes 12
        }
        return x;  // Outer x is still 1
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    assert_eq!(result.to_f32(), 1.0);
}

#[test]
fn test_nested_scopes() {
    let script = "
        float x = 1.0;
        {
            float y = 2.0;
            {
                float z = 3.0;
                x = x + y + z;  // Can access outer variables
            }
        }
        return x;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    // 1 + 2 + 3 = 6
    assert_eq!(result.to_f32(), 6.0);
}

#[test]
fn test_variable_in_loop_scope() {
    let script = "
        float sum = 0.0;
        for (float i = 0.0; i < 3.0; i = i + 1.0) {
            float temp = i * 2.0;
            sum = sum + temp;
        }
        return sum;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    // (0*2) + (1*2) + (2*2) = 0 + 2 + 4 = 6
    assert_eq!(result.to_f32(), 6.0);
}

#[test]
fn test_multiple_variables() {
    let script = "
        float a = 1.0;
        float b = 2.0;
        float c = 3.0;
        float d = 4.0;
        return a + b + c + d;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    assert_eq!(result.to_f32(), 10.0);
}

#[test]
fn test_assignment_expression_value() {
    let script = "
        float x = 0.0;
        float y = (x = 5.0);  // Assignment returns the assigned value
        return y;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    assert_eq!(result.to_f32(), 5.0);
}

#[test]
fn test_chained_assignments() {
    let script = "
        float x = 0.0;
        float y = 0.0;
        float z = 0.0;
        z = y = x = 7.0;  // Right-associative
        return x + y + z;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    assert_eq!(result.to_f32(), 21.0);
}
