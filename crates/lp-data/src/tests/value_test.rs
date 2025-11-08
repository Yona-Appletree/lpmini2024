//! Tests for the runtime value system.

use core::ptr::NonNull;

use lp_math::fixed::Fixed;
use lp_pool::collections::string::LpString;
use lp_pool::error::AllocError;
use lp_pool::memory_pool::LpMemoryPool;

use crate::metadata::{LpTypeMeta, TypeRef};
use crate::types::{RecordField, RecordType};
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
        const FIXED_META: LpTypeMeta = LpTypeMeta::new(crate::LpType::fixed());

        // Test None
        let opt_none = LpValue::try_option_none(&FIXED_META).unwrap();
        assert!(opt_none.is_none());
        assert!(!opt_none.is_some());
        assert!(opt_none.try_unwrap().is_err());

        // Test Some
        let inner_value = LpValue::fixed(Fixed::from_f32(42.0));
        let opt_some = LpValue::try_option_some(&FIXED_META, inner_value).unwrap();
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
        const FIXED_META: LpTypeMeta = LpTypeMeta::new(crate::LpType::fixed());
        const INT_META: LpTypeMeta = LpTypeMeta::new(crate::LpType::int32());

        const FIELDS: &[RecordField<TypeRef>] = &[
            RecordField::new("x", &FIXED_META),
            RecordField::new("y", &INT_META),
        ];
        const RECORD_TYPE: RecordType<TypeRef> = RecordType::new("Point", FIELDS);
        const RECORD_META: LpTypeMeta = LpTypeMeta::new(crate::LpType::Record(RECORD_TYPE));

        // Create record
        let mut record = LpValue::try_record(&RECORD_META).unwrap();

        // Set field values
        record
            .try_set_field("x", LpValue::fixed(Fixed::from_f32(10.0)))
            .unwrap();
        record.try_set_field("y", LpValue::int32(20)).unwrap();

        // Get field values
        let x_field = record.get_field("x").unwrap();
        assert_eq!(x_field.as_fixed().unwrap(), Fixed::from_f32(10.0));

        let y_field = record.get_field("y").unwrap();
        assert_eq!(y_field.as_int32().unwrap(), 20);

        // Test field not found
        assert!(record.get_field("z").is_err());

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
        const FIXED_META: LpTypeMeta = LpTypeMeta::new(crate::LpType::fixed());

        // Create array
        let mut array = LpValue::try_array(&FIXED_META, 0).unwrap();

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
        assert_eq!(elem0.as_fixed().unwrap(), Fixed::from_f32(1.0));

        let elem1 = array.get_element(1).unwrap();
        assert_eq!(elem1.as_fixed().unwrap(), Fixed::from_f32(2.0));

        // Set element
        array
            .try_set_element(1, LpValue::fixed(Fixed::from_f32(99.0)))
            .unwrap();
        let updated_elem1 = array.get_element(1).unwrap();
        assert_eq!(updated_elem1.as_fixed().unwrap(), Fixed::from_f32(99.0));

        // Test index out of bounds
        assert!(array.get_element(10).is_err());

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
        const INT_META: LpTypeMeta = LpTypeMeta::new(crate::LpType::int32());

        // Inner record: { c: int32 }
        const INNER_FIELDS: &[RecordField<TypeRef>] = &[RecordField::new("c", &INT_META)];
        const INNER_TYPE: RecordType<TypeRef> = RecordType::new("Inner", INNER_FIELDS);
        const INNER_META: LpTypeMeta = LpTypeMeta::new(crate::LpType::Record(INNER_TYPE));

        // Middle record: { b: Inner }
        const MIDDLE_FIELDS: &[RecordField<TypeRef>] = &[RecordField::new("b", &INNER_META)];
        const MIDDLE_TYPE: RecordType<TypeRef> = RecordType::new("Middle", MIDDLE_FIELDS);
        const MIDDLE_META: LpTypeMeta = LpTypeMeta::new(crate::LpType::Record(MIDDLE_TYPE));

        // Outer record: { a: Middle }
        const OUTER_FIELDS: &[RecordField<TypeRef>] = &[RecordField::new("a", &MIDDLE_META)];
        const OUTER_TYPE: RecordType<TypeRef> = RecordType::new("Outer", OUTER_FIELDS);
        const OUTER_META: LpTypeMeta = LpTypeMeta::new(crate::LpType::Record(OUTER_TYPE));

        let mut outer = LpValue::try_record(&OUTER_META).unwrap();
        let a = outer.get_field_mut("a").unwrap();
        let b = a.get_field_mut("b").unwrap();
        *b.get_field_mut("c").unwrap() = LpValue::int32(42);

        // Test path access
        let c_value = outer.get_path("a.b.c").unwrap();
        assert_eq!(c_value.as_int32().unwrap(), 42);

        Ok::<(), AllocError>(())
    })
    .unwrap();
}
