use crate::lpscript::vm::vm_limits::VmLimits;
/// Comprehensive tests for user-defined functions
use crate::lpscript::*;
use crate::math::{Fixed, ToFixed};

#[test]
fn test_function_no_params() {
    let script = "
        float get_pi() {
            return 3.14159;
        }
        return get_pi();
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    assert!((result.to_f32() - 3.14159).abs() < 0.01);
}

#[test]
fn test_function_with_vec_params() {
    let script = "
        float vec2_sum(vec2 v) {
            return v.x + v.y;
        }
        return vec2_sum(vec2(3.0, 7.0));
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    assert_eq!(result.to_f32(), 10.0);
}

#[test]
fn test_function_calling_function() {
    let script = "
        float triple(float x) {
            return x * 3.0;
        }
        
        float sextuple(float x) {
            return triple(x) * 2.0;
        }
        
        return sextuple(5.0);
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    assert_eq!(result.to_f32(), 30.0);
}

#[test]
fn test_recursive_fibonacci() {
    let script = "
        float fib(float n) {
            if (n <= 1.0) {
                return n;
            }
            return fib(n - 1.0) + fib(n - 2.0);
        }
        return fib(6.0);
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    // fib(6) = 8
    assert_eq!(result.to_f32(), 8.0);
}

#[test]
fn test_function_with_local_variables() {
    let script = "
        float compute(float a, float b) {
            float sum = a + b;
            float product = a * b;
            return sum + product;
        }
        return compute(3.0, 4.0);
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    // (3 + 4) + (3 * 4) = 7 + 12 = 19
    assert_eq!(result.to_f32(), 19.0);
}

#[test]
fn test_function_multiple_functions() {
    let script = "
        float add(float a, float b) {
            return a + b;
        }
        
        float multiply(float a, float b) {
            return a * b;
        }
        
        float divide(float a, float b) {
            return a / b;
        }
        
        float x = add(10.0, 5.0);        // 15
        float y = multiply(x, 2.0);      // 30
        float z = divide(y, 3.0);        // 10
        return z;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    assert_eq!(result.to_f32(), 10.0);
}

#[test]
fn test_function_with_conditional_return() {
    let script = "
        float abs_custom(float x) {
            if (x < 0.0) {
                return 0.0 - x;
            }
            return x;
        }
        
        float a = abs_custom(-5.0);
        float b = abs_custom(3.0);
        return a + b;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    assert!((result.to_f32() - 8.0).abs() < 0.1); // Relax for fixed-point precision
}

#[test]
fn test_function_with_loop() {
    let script = "
        float sum_range(float n) {
            float sum = 0.0;
            for (float i = 0.0; i < n; i = i + 1.0) {
                sum = sum + i;
            }
            return sum;
        }
        return sum_range(5.0);
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    // 0 + 1 + 2 + 3 + 4 = 10
    assert_eq!(result.to_f32(), 10.0);
}

#[test]
fn test_function_vec_return() {
    let script = "
        vec2 make_vec(float x, float y) {
            return vec2(x, y);
        }
        vec2 v = make_vec(3.0, 4.0);
        return v.x + v.y;
    ";
    let program = parse_script(script);
    let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();

    let result = vm
        .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
        .unwrap();
    assert_eq!(result.to_f32(), 7.0);
}
