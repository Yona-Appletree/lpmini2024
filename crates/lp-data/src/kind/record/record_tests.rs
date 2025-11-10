//! Integration tests for Record shapes and values.

#[cfg(test)]
mod tests {
    use crate::kind::{
        fixed::fixed_static::FIXED_SHAPE,
        kind::LpKind,
        record::{
            record_meta::{RecordFieldMetaStatic, RecordMetaStatic},
            record_static::{RecordFieldStatic, RecordShapeStatic},
            RecordFieldShape, RecordShape,
        },
        shape::LpShape,
    };

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
}
