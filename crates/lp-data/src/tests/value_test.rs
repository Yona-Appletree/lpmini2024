//! Tests for the runtime value system.

use core::ptr::NonNull;

use lp_math::fixed::Fixed;
use lp_pool::collections::string::LpString;
use lp_pool::error::AllocError;
use lp_pool::memory_pool::LpMemoryPool;

use crate::shape::record::{RecordField, StaticRecordShape};
use crate::shape::shape_ref::{RecordShapeRef, ShapeRef};
use crate::value::LpValue;

#[test]
fn test_scalar_values() {
    let value_fixed = LpValue::fixed(Fixed::from_f32(1.5));
    assert_eq!(value_fixed.as_fixed().unwrap(), Fixed::from_f32(1.5));

    let value_int = LpValue::int32(42);
    assert_eq!(value_int.as_int32().unwrap(), 42);

    let value_bool = LpValue::bool(true);
    assert_eq!(value_bool.as_bool().unwrap(), true);

    let mut memory = [0u8; 4096];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    let pool = unsafe { LpMemoryPool::new(memory_ptr, 4096).unwrap() };
    pool.run(|| {
        let s = LpString::try_from_str("hello").unwrap();
        let value_string = LpValue::string(s);
        assert_eq!(value_string.as_string().unwrap(), "hello");
        Ok::<(), AllocError>(())
    })
    .unwrap();
}

#[test]
fn test_vector_values() {
    let v2 = LpValue::vec2(Fixed::from_f32(1.0), Fixed::from_f32(2.0));
    let (x, y) = v2.as_vec2().unwrap();
    assert_eq!(x, Fixed::from_f32(1.0));
    assert_eq!(y, Fixed::from_f32(2.0));

    let v3 = LpValue::vec3(
        Fixed::from_f32(1.0),
        Fixed::from_f32(2.0),
        Fixed::from_f32(3.0),
    );
    let (x, y, z) = v3.as_vec3().unwrap();
    assert_eq!(x, Fixed::from_f32(1.0));
    assert_eq!(y, Fixed::from_f32(2.0));
    assert_eq!(z, Fixed::from_f32(3.0));

    let v4 = LpValue::vec4(
        Fixed::from_f32(1.0),
        Fixed::from_f32(2.0),
        Fixed::from_f32(3.0),
        Fixed::from_f32(4.0),
    );
    let (x, y, z, w) = v4.as_vec4().unwrap();
    assert_eq!(x, Fixed::from_f32(1.0));
    assert_eq!(y, Fixed::from_f32(2.0));
    assert_eq!(z, Fixed::from_f32(3.0));
    assert_eq!(w, Fixed::from_f32(4.0));
}

#[test]
fn test_option_values() {
    let mut memory = [0u8; 4096];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    let pool = unsafe { LpMemoryPool::new(memory_ptr, 4096).unwrap() };

    pool.run(|| {
        let fixed_shape = ShapeRef::fixed_default();

        // Test None
        let opt_none = LpValue::try_option_none(fixed_shape).unwrap();
        assert!(opt_none.is_none());
        assert!(!opt_none.is_some());
        assert!(opt_none.try_unwrap().is_err());

        // Test Some
        let inner_value = LpValue::fixed(Fixed::from_f32(42.0));
        let opt_some = LpValue::try_option_some(fixed_shape, inner_value).unwrap();
        assert!(opt_some.is_some());
        assert!(!opt_some.is_none());
        let unwrapped = opt_some.try_unwrap().unwrap();
        assert_eq!(unwrapped.as_fixed().unwrap(), Fixed::from_f32(42.0));

        Ok::<(), AllocError>(())
    })
    .unwrap();
}

#[test]
fn test_record_values() {
    let mut memory = [0u8; 4096];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    let pool = unsafe { LpMemoryPool::new(memory_ptr, 4096).unwrap() };

    pool.run(|| {
        // Create static shapes for fields
        use crate::shape::fixed::StaticFixedShape;
        use crate::shape::int32::StaticInt32Shape;
        use crate::shape::shape_ref::{FixedShapeRef, Int32ShapeRef};

        static FIXED_SHAPE_STATIC: StaticFixedShape = StaticFixedShape::default();
        static INT_SHAPE_STATIC: StaticInt32Shape = StaticInt32Shape::default();

        static FIXED_SHAPE: ShapeRef = ShapeRef::Fixed(FixedShapeRef::Static(&FIXED_SHAPE_STATIC));
        static INT_SHAPE: ShapeRef = ShapeRef::Int32(Int32ShapeRef::Static(&INT_SHAPE_STATIC));

        // Create record fields
        static FIELDS: &[RecordField] = &[
            RecordField::new("x", FIXED_SHAPE),
            RecordField::new("y", INT_SHAPE),
        ];

        // Create static record shape
        static RECORD_SHAPE: StaticRecordShape = StaticRecordShape {
            name: "Point",
            fields: FIELDS,
            ui: crate::shape::record::RecordUi { collapsible: false },
        };

        // Create ShapeRef for the record
        let record_shape_ref = ShapeRef::Record(RecordShapeRef::Static(&RECORD_SHAPE));

        // Create record
        let mut record = LpValue::try_record(record_shape_ref).unwrap();

        // Set field values
        record
            .try_set_field("x", LpValue::fixed(Fixed::from_f32(10.0)))
            .unwrap();
        record.try_set_field("y", LpValue::int32(20)).unwrap();

        // Get field values
        let x_field = record.get_field("x").unwrap();
        // x_field is &dyn LpValueTrait, need to downcast to LpValue
        // For now, use try_set_field to verify it works
        let x_value = match &record {
            LpValue::Struct(s) => s.get_field("x").unwrap(),
            _ => panic!("expected Struct"),
        };
        assert_eq!(x_value.as_fixed().unwrap(), Fixed::from_f32(10.0));

        let y_value = match &record {
            LpValue::Struct(s) => s.get_field("y").unwrap(),
            _ => panic!("expected Struct"),
        };
        assert_eq!(y_value.as_int32().unwrap(), 20);

        // Test field not found
        match &record {
            LpValue::Struct(s) => assert!(s.get_field("z").is_err()),
            _ => panic!("expected Struct"),
        }

        Ok::<(), AllocError>(())
    })
    .unwrap();
}

#[test]
fn test_array_values() {
    let mut memory = [0u8; 4096];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    let pool = unsafe { LpMemoryPool::new(memory_ptr, 4096).unwrap() };

    pool.run(|| {
        let fixed_shape = ShapeRef::fixed_default();

        // Create array shape
        use crate::shape::array::StaticArrayShape;
        use crate::shape::fixed::StaticFixedShape;
        use crate::shape::shape_ref::ArrayShapeRef;
        use crate::shape::shape_ref::FixedShapeRef;

        static ELEMENT_SHAPE_STATIC: StaticFixedShape = StaticFixedShape::default();
        static ELEMENT_SHAPE: ShapeRef =
            ShapeRef::Fixed(FixedShapeRef::Static(&ELEMENT_SHAPE_STATIC));

        static ARRAY_SHAPE: StaticArrayShape = StaticArrayShape {
            element: ELEMENT_SHAPE,
            ui: crate::shape::record::RecordUi { collapsible: false },
        };
        let array_shape_ref = ShapeRef::Array(ArrayShapeRef::Static(&ARRAY_SHAPE));

        // Create array
        let mut array = LpValue::try_array(array_shape_ref, 0).unwrap();

        // Push elements
        array
            .try_push_element(LpValue::fixed(Fixed::from_f32(1.0)))
            .unwrap();
        array
            .try_push_element(LpValue::fixed(Fixed::from_f32(2.0)))
            .unwrap();
        array
            .try_push_element(LpValue::fixed(Fixed::from_f32(3.0)))
            .unwrap();

        // Get elements
        let elem0 = array.get_element(0).unwrap();
        // elem0 is &dyn LpValueTrait, need to access through array value
        match &array {
            LpValue::Array(arr) => {
                let val = arr.get(0).unwrap();
                assert_eq!(val.as_fixed().unwrap(), Fixed::from_f32(1.0));
            }
            _ => panic!("expected Array"),
        }

        match &array {
            LpValue::Array(arr) => {
                let val = arr.get(1).unwrap();
                assert_eq!(val.as_fixed().unwrap(), Fixed::from_f32(2.0));
            }
            _ => panic!("expected Array"),
        }

        // Set element
        array
            .try_set_element(1, LpValue::fixed(Fixed::from_f32(99.0)))
            .unwrap();
        match &array {
            LpValue::Array(arr) => {
                let val = arr.get(1).unwrap();
                assert_eq!(val.as_fixed().unwrap(), Fixed::from_f32(99.0));
            }
            _ => panic!("expected Array"),
        }

        // Test index out of bounds
        match &array {
            LpValue::Array(arr) => assert!(arr.get(10).is_err()),
            _ => panic!("expected Array"),
        }

        Ok::<(), AllocError>(())
    })
    .unwrap();
}

#[test]
fn test_path_access() {
    let mut memory = [0u8; 4096];
    let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
    let pool = unsafe { LpMemoryPool::new(memory_ptr, 4096).unwrap() };

    pool.run(|| {
        // Create nested structure: { a: { b: { c: 42 } } }
        use crate::shape::int32::StaticInt32Shape;
        use crate::shape::shape_ref::Int32ShapeRef;

        static INT_SHAPE_STATIC: StaticInt32Shape = StaticInt32Shape::default();
        static INT_SHAPE: ShapeRef = ShapeRef::Int32(Int32ShapeRef::Static(&INT_SHAPE_STATIC));

        // Inner record: { c: int32 }
        static INNER_FIELDS: &[RecordField] = &[RecordField::new("c", INT_SHAPE)];
        static INNER_SHAPE: StaticRecordShape = StaticRecordShape {
            name: "Inner",
            fields: INNER_FIELDS,
            ui: crate::shape::record::RecordUi { collapsible: false },
        };
        static INNER_SHAPE_REF: ShapeRef = ShapeRef::Record(RecordShapeRef::Static(&INNER_SHAPE));

        // Middle record: { b: Inner }
        static MIDDLE_FIELDS: &[RecordField] = &[RecordField::new("b", INNER_SHAPE_REF)];
        static MIDDLE_SHAPE: StaticRecordShape = StaticRecordShape {
            name: "Middle",
            fields: MIDDLE_FIELDS,
            ui: crate::shape::record::RecordUi { collapsible: false },
        };
        static MIDDLE_SHAPE_REF: ShapeRef = ShapeRef::Record(RecordShapeRef::Static(&MIDDLE_SHAPE));

        // Outer record: { a: Middle }
        static OUTER_FIELDS: &[RecordField] = &[RecordField::new("a", MIDDLE_SHAPE_REF)];
        static OUTER_SHAPE: StaticRecordShape = StaticRecordShape {
            name: "Outer",
            fields: OUTER_FIELDS,
            ui: crate::shape::record::RecordUi { collapsible: false },
        };
        let outer_shape_ref = ShapeRef::Record(RecordShapeRef::Static(&OUTER_SHAPE));

        let mut outer = LpValue::try_record(outer_shape_ref).unwrap();

        // Access fields mutably
        match &mut outer {
            LpValue::Struct(s) => {
                let a = s.get_field_mut("a").unwrap();
                match a {
                    LpValue::Struct(s2) => {
                        let b = s2.get_field_mut("b").unwrap();
                        match b {
                            LpValue::Struct(s3) => {
                                *s3.get_field_mut("c").unwrap() = LpValue::int32(42);
                            }
                            _ => panic!("expected Struct"),
                        }
                    }
                    _ => panic!("expected Struct"),
                }
            }
            _ => panic!("expected Struct"),
        }

        // Test path access
        let c_value = outer.get_path("a.b.c").unwrap();
        assert_eq!(c_value.as_int32().unwrap(), 42);

        Ok::<(), AllocError>(())
    })
    .unwrap();
}
