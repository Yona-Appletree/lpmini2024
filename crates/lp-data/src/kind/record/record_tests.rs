//! Integration tests for Record shapes and values.

#[cfg(test)]
mod tests {
    use crate::kind::fixed::fixed_static::FIXED_SHAPE;
    use crate::kind::kind::LpKind;
    use crate::kind::record::record_meta::{RecordFieldMetaStatic, RecordMetaStatic};
    use crate::kind::record::record_static::{RecordFieldStatic, RecordShapeStatic};
    use crate::kind::record::{RecordFieldShape, RecordShape};
    use crate::kind::shape::LpShape;

    #[test]
    fn test_record_shape_static() {
        const FIELD: RecordFieldStatic = RecordFieldStatic {
            name: "period_ms",
            shape: &FIXED_SHAPE,
            meta: RecordFieldMetaStatic { docs: None },
        };

        const FIELDS: &[RecordFieldStatic] = &[FIELD];
        let shape = RecordShapeStatic {
            meta: RecordMetaStatic {
                name: "LfoConfig",
                docs: None,
            },
            fields: FIELDS,
        };

        assert_eq!(shape.kind(), LpKind::Record);
        assert_eq!(shape.field_count(), 1);
        assert!(shape.find_field("period_ms").is_some());
        assert!(shape.find_field("nonexistent").is_none());
    }

    #[test]
    fn test_record_field_access() {
        const FIELD: RecordFieldStatic = RecordFieldStatic {
            name: "period_ms",
            shape: &FIXED_SHAPE,
            meta: RecordFieldMetaStatic {
                docs: Some("Oscillation period in milliseconds"),
            },
        };

        let field: &dyn RecordFieldShape = &FIELD;
        assert_eq!(field.name(), "period_ms");
        assert_eq!(field.shape().kind(), LpKind::Fixed);
        assert_eq!(
            field.meta().docs(),
            Some("Oscillation period in milliseconds")
        );
    }

    #[test]
    fn test_record_with_all_primitive_types() {
        use crate::kind::bool::bool_static::BOOL_SHAPE;
        use crate::kind::int32::int32_static::INT32_SHAPE;
        use crate::kind::vec2::vec2_static::VEC2_SHAPE;
        use crate::kind::vec3::vec3_static::VEC3_SHAPE;
        use crate::kind::vec4::vec4_static::VEC4_SHAPE;

        const FIELDS: &[RecordFieldStatic] = &[
            RecordFieldStatic {
                name: "count",
                shape: &INT32_SHAPE,
                meta: RecordFieldMetaStatic { docs: None },
            },
            RecordFieldStatic {
                name: "enabled",
                shape: &BOOL_SHAPE,
                meta: RecordFieldMetaStatic { docs: None },
            },
            RecordFieldStatic {
                name: "position",
                shape: &VEC2_SHAPE,
                meta: RecordFieldMetaStatic { docs: None },
            },
            RecordFieldStatic {
                name: "rotation",
                shape: &VEC3_SHAPE,
                meta: RecordFieldMetaStatic { docs: None },
            },
            RecordFieldStatic {
                name: "color",
                shape: &VEC4_SHAPE,
                meta: RecordFieldMetaStatic { docs: None },
            },
            RecordFieldStatic {
                name: "frequency",
                shape: &FIXED_SHAPE,
                meta: RecordFieldMetaStatic { docs: None },
            },
        ];

        let shape = RecordShapeStatic {
            meta: RecordMetaStatic {
                name: "TestRecord",
                docs: None,
            },
            fields: FIELDS,
        };

        assert_eq!(shape.kind(), LpKind::Record);
        assert_eq!(shape.field_count(), 6);
        assert_eq!(
            shape.find_field("count").unwrap().shape().kind(),
            LpKind::Int32
        );
        assert_eq!(
            shape.find_field("enabled").unwrap().shape().kind(),
            LpKind::Bool
        );
        assert_eq!(
            shape.find_field("position").unwrap().shape().kind(),
            LpKind::Vec2
        );
        assert_eq!(
            shape.find_field("rotation").unwrap().shape().kind(),
            LpKind::Vec3
        );
        assert_eq!(
            shape.find_field("color").unwrap().shape().kind(),
            LpKind::Vec4
        );
        assert_eq!(
            shape.find_field("frequency").unwrap().shape().kind(),
            LpKind::Fixed
        );
    }
}
