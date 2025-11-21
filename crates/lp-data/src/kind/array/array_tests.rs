//! Integration tests for Array shapes and values.

#[cfg(test)]
mod tests {
    use lp_alloc::{
        enter_global_alloc_allowance, init_test_allocator, AllocLimitError as AllocError,
    };
    use lp_math::dec32::Dec32;

    use crate::kind::array::array_dyn::ArrayShapeDyn;
    use crate::kind::array::array_meta::ArrayMetaDyn;
    use crate::kind::array::array_shape::ArrayShape;
    use crate::kind::array::array_static::ArrayShapeStatic;
    use crate::kind::array::array_value::ArrayValue;
    use crate::kind::array::ArrayValueDyn;
    use crate::kind::dec32::dec32_static::DEC32_SHAPE;
    use crate::kind::int32::int32_static::INT32_SHAPE;
    use crate::kind::kind::LpKind;
    use crate::kind::shape::LpShape;
    use crate::kind::value::LpValueBox;

    struct TestPool;

    impl TestPool {
        fn run<F, R>(&self, f: F) -> R
        where
            F: FnOnce() -> R,
        {
            let _guard = enter_global_alloc_allowance();
            f()
        }
    }

    fn setup_pool() -> TestPool {
        init_test_allocator();
        TestPool
    }

    #[test]
    fn test_array_shape_static() {
        let shape = ArrayShapeStatic {
            meta: crate::kind::array::array_meta::ArrayMetaStatic {
                name: "Int32Array",
                docs: None,
            },
            element_shape: &INT32_SHAPE,
            len: 3,
        };

        assert_eq!(shape.kind(), LpKind::Array);
        assert_eq!(shape.len(), 3);
        assert_eq!(shape.element_shape().kind(), LpKind::Int32);
        assert_eq!(shape.meta().name(), "Int32Array");
    }

    #[test]
    fn test_array_value_dyn_new() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("Int32Array".to_string())?;
            let shape = ArrayShapeDyn {
                meta: ArrayMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                element_shape: &INT32_SHAPE,
                len: 0,
            };
            let array = ArrayValueDyn::new(shape);
            assert_eq!(array.len(), 0);
            assert_eq!(ArrayValue::shape(&array).len(), 0);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_array_value_dyn_push() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("Int32Array".to_string())?;
            let shape = ArrayShapeDyn {
                meta: ArrayMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                element_shape: &INT32_SHAPE,
                len: 0,
            };
            let mut array = ArrayValueDyn::new(shape);

            let value1 = LpValueBox::from(42i32);
            let value2 = LpValueBox::from(100i32);

            array
                .push(value1)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            assert_eq!(array.len(), 1);
            assert_eq!(ArrayValue::shape(&array).len(), 1);

            array
                .push(value2)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            assert_eq!(array.len(), 2);
            assert_eq!(ArrayValue::shape(&array).len(), 2);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_array_value_dyn_get_element() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("Int32Array".to_string())?;
            let shape = ArrayShapeDyn {
                meta: ArrayMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                element_shape: &INT32_SHAPE,
                len: 0,
            };
            let mut array = ArrayValueDyn::new(shape);

            let value1 = LpValueBox::from(42i32);
            let value2 = LpValueBox::from(100i32);

            array
                .push(value1)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            array
                .push(value2)
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            let elem0 = array
                .get_element(0)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            assert_eq!(
                elem0.as_lp_value().shape().kind(),
                LpKind::Int32,
                "Element 0 should be Int32"
            );

            let elem1 = array
                .get_element(1)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            assert_eq!(
                elem1.as_lp_value().shape().kind(),
                LpKind::Int32,
                "Element 1 should be Int32"
            );

            // Test out of bounds
            assert!(array.get_element(2).is_err());

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_array_value_dyn_type_validation() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("Int32Array".to_string())?;
            let shape = ArrayShapeDyn {
                meta: ArrayMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                element_shape: &INT32_SHAPE,
                len: 0,
            };
            let mut array = ArrayValueDyn::new(shape);

            // Push valid Int32 value
            let valid_value = LpValueBox::from(42i32);
            array
                .push(valid_value)
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            // Try to push invalid Dec32 value - should fail
            let invalid_value = LpValueBox::from(Dec32::ZERO);
            assert!(
                array.push(invalid_value).is_err(),
                "Should reject Dec32 value in Int32 array"
            );

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_array_value_dyn_remove() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("Int32Array".to_string())?;
            let shape = ArrayShapeDyn {
                meta: ArrayMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                element_shape: &INT32_SHAPE,
                len: 0,
            };
            let mut array = ArrayValueDyn::new(shape);

            let value1 = LpValueBox::from(42i32);
            let value2 = LpValueBox::from(100i32);
            let value3 = LpValueBox::from(200i32);

            array
                .push(value1)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            array
                .push(value2)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            array
                .push(value3)
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            assert_eq!(array.len(), 3);

            // Remove middle element
            array.remove(1).map_err(|_| AllocError::SoftLimitExceeded)?;
            assert_eq!(array.len(), 2);
            assert_eq!(ArrayValue::shape(&array).len(), 2);

            // Remove first element
            array.remove(0).map_err(|_| AllocError::SoftLimitExceeded)?;
            assert_eq!(array.len(), 1);
            assert_eq!(ArrayValue::shape(&array).len(), 1);

            // Test out of bounds
            assert!(array.remove(1).is_err());

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_array_value_dyn_get_element_mut() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("Int32Array".to_string())?;
            let shape = ArrayShapeDyn {
                meta: ArrayMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                element_shape: &INT32_SHAPE,
                len: 0,
            };
            let mut array = ArrayValueDyn::new(shape);

            let value1 = LpValueBox::from(42i32);
            array
                .push(value1)
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            let mut elem0 = array
                .get_element_mut(0)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            assert_eq!(
                elem0.as_lp_value_mut().shape().kind(),
                LpKind::Int32,
                "Element 0 should be Int32"
            );

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_array_value_dyn_with_dec32_elements() {
        let pool = setup_pool();
        pool.run(|| {
            let shape_name = Ok::<_, AllocError>("FixedArray".to_string())?;
            let shape = ArrayShapeDyn {
                meta: ArrayMetaDyn {
                    name: shape_name,
                    docs: None,
                },
                element_shape: &DEC32_SHAPE,
                len: 0,
            };
            let mut array = ArrayValueDyn::new(shape);

            let value1 = LpValueBox::from(Dec32::ZERO);
            let value2 = LpValueBox::from(Dec32::from_i32(42));

            array
                .push(value1)
                .map_err(|_| AllocError::SoftLimitExceeded)?;
            array
                .push(value2)
                .map_err(|_| AllocError::SoftLimitExceeded)?;

            assert_eq!(array.len(), 2);
            assert_eq!(
                ArrayValue::shape(&array).element_shape().kind(),
                LpKind::Dec32
            );

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_array_shape_meta() {
        let shape = ArrayShapeStatic {
            meta: crate::kind::array::array_meta::ArrayMetaStatic {
                name: "TestArray",
                docs: Some("Test array with documentation"),
            },
            element_shape: &INT32_SHAPE,
            len: 5,
        };

        assert_eq!(shape.meta().name(), "TestArray");
        assert_eq!(shape.meta().docs(), Some("Test array with documentation"));
    }
}
