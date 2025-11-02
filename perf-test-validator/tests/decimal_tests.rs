/// Tests for the Decimal struct
use perf_tests_common::perlin3_decimal::Decimal;

const EPSILON: f32 = 0.01;

fn assert_close(d: Decimal, expected: f32, msg: &str) {
    let actual = d.to_f32();
    let diff = (actual - expected).abs();
    assert!(
        diff < EPSILON,
        "{}: expected {}, got {} (diff: {})",
        msg, expected, actual, diff
    );
}

#[test]
fn test_from_f32() {
    assert_close(Decimal::from_f32(0.0), 0.0, "from_f32(0.0)");
    assert_close(Decimal::from_f32(1.0), 1.0, "from_f32(1.0)");
    assert_close(Decimal::from_f32(2.5), 2.5, "from_f32(2.5)");
}

#[test]
fn test_mul_int_is_correct() {
    // The critical test: Decimal * int should NOT do fixed-point multiply
    let one = Decimal::from_int(1);
    assert_close(one * 4, 4.0, "1 * 4 = 4 (proves int mul is correct)");
    
    let half = Decimal::from_f32(0.5);
    assert_close(half * 8, 4.0, "0.5 * 8 = 4");
}

#[test]
fn test_mul_decimal_does_shift() {
    // Decimal * Decimal SHOULD shift
    let two = Decimal::from_f32(2.0);
    let three = Decimal::from_f32(3.0);
    assert_close(two * three, 6.0, "2.0 * 3.0 = 6.0");
}

#[test]
fn test_coordinate_scaling() {
    // The actual perlin coordinate calculation
    let width = Decimal::from_int(8);
    let x = Decimal::from_int(1);
    let nx = (x * 4) / width;
    assert_close(nx, 0.5, "(1 * 4) / 8 = 0.5");
}

#[test]
fn test_div_decimal() {
    let one = Decimal::from_f32(1.0);
    let four = Decimal::from_f32(4.0);
    assert_close(one / four, 0.25, "1.0 / 4.0 = 0.25");
}

