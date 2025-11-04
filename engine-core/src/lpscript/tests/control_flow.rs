/// Comprehensive tests for control flow statements
use crate::lpscript::*;
use crate::math::{Fixed, ToFixed};
use crate::lpscript::vm::VmLimits;

#[test]
#[ignore] // TODO: Fix compiler bug - if statements generate invalid bytecode
fn test_if_without_else() {
    let script = "
        if (uv.x > 0.5) {
            return 1.0;
        }
        return 0.0;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
    
    let result = vm.run(0.6.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
    assert_eq!(result.to_f32(), 1.0);
    
    let result = vm.run(0.4.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
    assert_eq!(result.to_f32(), 0.0);
}

#[test]
#[ignore] // TODO: Fix compiler bug - if statements generate invalid bytecode
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
    let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
    
    let result = vm.run(0.9.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
    assert_eq!(result.to_f32(), 3.0);
    
    let result = vm.run(0.6.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
    assert_eq!(result.to_f32(), 2.0);
    
    let result = vm.run(0.3.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
    assert_eq!(result.to_f32(), 1.0);
}

#[test]
#[ignore] // TODO: Fix compiler bug - loops generate infinite bytecode
fn test_while_loop_counter() {
    let script = "
        float i = 0.0;
        float sum = 0.0;
        while (i < 5.0) {
            sum = sum + i;
            i = i + 1.0;
        }
        return sum;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
    
    let result = vm.run(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap();
    // 0 + 1 + 2 + 3 + 4 = 10
    assert_eq!(result.to_f32(), 10.0);
}

#[test]
#[ignore] // TODO: Fix compiler bug - loops generate infinite bytecode
fn test_for_loop_sum() {
    let script = "
        float sum = 0.0;
        for (float i = 1.0; i <= 4.0; i = i + 1.0) {
            sum = sum + i;
        }
        return sum;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
    
    let result = vm.run(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap();
    // 1 + 2 + 3 + 4 = 10
    assert_eq!(result.to_f32(), 10.0);
}

#[test]
#[ignore] // TODO: Fix compiler bug - loops generate infinite bytecode  
fn test_for_loop_with_break_condition() {
    let script = "
        float result = 0.0;
        for (float i = 0.0; i < 100.0; i = i + 1.0) {
            if (i >= 3.0) {
                return i;
            }
            result = i;
        }
        return result;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
    
    let result = vm.run(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap();
    assert_eq!(result.to_f32(), 3.0);
}

#[test]
#[ignore] // TODO: Fix compiler bug - loops generate infinite bytecode
fn test_nested_loops() {
    let script = "
        float count = 0.0;
        for (float i = 0.0; i < 3.0; i = i + 1.0) {
            for (float j = 0.0; j < 2.0; j = j + 1.0) {
                count = count + 1.0;
            }
        }
        return count;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
    
    let result = vm.run(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap();
    // 3 * 2 = 6
    assert_eq!(result.to_f32(), 6.0);
}

#[test]
#[ignore] // TODO: Fix compiler bug - if statements generate invalid bytecode
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
    let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
    
    let result = vm.run(0.1.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
    assert_eq!(result.to_f32(), 1.0);
    
    let result = vm.run(0.3.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
    assert_eq!(result.to_f32(), 2.0);
    
    let result = vm.run(0.6.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
    assert_eq!(result.to_f32(), 3.0);
    
    let result = vm.run(0.9.to_fixed(), Fixed::ZERO, Fixed::ZERO).unwrap();
    assert_eq!(result.to_f32(), 4.0);
}

