//! Tests for Mat3 type support.

use lp_math::fixed::Mat3;

use crate::kind::mat3::mat3_static::MAT3_SHAPE;
use crate::kind::shape::LpShape;
use crate::kind::value::LpValue;

#[test]
fn test_mat3_shape() {
    let shape = &MAT3_SHAPE;
    assert_eq!(shape.kind(), crate::kind::kind::LpKind::Mat3);
}

#[test]
fn test_mat3_value() {
    let m = Mat3::identity();
    let shape = m.shape();
    assert_eq!(shape.kind(), crate::kind::kind::LpKind::Mat3);
}

#[test]
fn test_mat3_value_box() {
    let m = Mat3::from_f32(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let boxed: crate::kind::value::LpValueBox = m.into();
    match boxed {
        crate::kind::value::LpValueBox::Mat3(_) => {}
        _ => panic!("Expected Mat3 variant"),
    }
}
