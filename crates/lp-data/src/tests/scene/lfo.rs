//! LFO (Low Frequency Oscillator) node.

#[cfg(feature = "alloc")]
extern crate alloc;

use crate::kind::{
    fixed::{fixed_meta::FixedMetaStatic, fixed_static::FixedShapeStatic},
    record::{
        record_meta::RecordFieldMetaStatic,
        record_shape::RecordShape,
        record_static::{RecordFieldStatic, RecordShapeStatic},
    },
    shape::LpShape,
    value::{LpValue, LpValueBox, LpValueRef, LpValueRefMut, RecordValue},
};
use crate::value::RuntimeError;
use lp_math::fixed::Fixed;

/// Configuration for an LFO node.
#[derive(Debug, Clone, PartialEq)]
pub struct LfoConfig {
    /// Oscillation period in seconds.
    pub period: Fixed,
}

/// Static shape for LfoConfig.
const LFO_CONFIG_SHAPE: RecordShapeStatic = RecordShapeStatic {
    name: "LFO Config",
    fields: &[RecordFieldStatic {
        name: "period",
        shape: &PERIOD_SHAPE,
        meta: RecordFieldMetaStatic {
            docs: Some("Oscillation period in seconds"),
        },
    }],
};

/// Shape for period field with unit metadata.
const PERIOD_SHAPE: FixedShapeStatic = FixedShapeStatic::with_meta(FixedMetaStatic {
    label: "Period",
    desc_md: Some("Oscillation period"),
    unit: Some("seconds"),
});

impl LpValue for LfoConfig {
    fn shape(&self) -> &dyn LpShape {
        &LFO_CONFIG_SHAPE
    }
}

impl RecordValue for LfoConfig {
    fn get_field(&self, name: &str) -> Result<LpValueRef, RuntimeError> {
        match name {
            "period" => Ok(LpValueRef::Fixed(&self.period as &dyn LpValue)),
            _ => Err(RuntimeError::FieldNotFound {
                #[cfg(feature = "alloc")]
                record_name: alloc::string::String::from("LfoConfig"),
                #[cfg(not(feature = "alloc"))]
                record_name: "LfoConfig",
                #[cfg(feature = "alloc")]
                field_name: alloc::string::String::from(name),
                #[cfg(not(feature = "alloc"))]
                field_name: name,
            }),
        }
    }

    fn get_field_mut(&mut self, name: &str) -> Result<LpValueRefMut, RuntimeError> {
        match name {
            "period" => Ok(LpValueRefMut::Fixed(&mut self.period as &mut dyn LpValue)),
            _ => Err(RuntimeError::FieldNotFound {
                #[cfg(feature = "alloc")]
                record_name: alloc::string::String::from("LfoConfig"),
                #[cfg(not(feature = "alloc"))]
                record_name: "LfoConfig",
                #[cfg(feature = "alloc")]
                field_name: alloc::string::String::from(name),
                #[cfg(not(feature = "alloc"))]
                field_name: name,
            }),
        }
    }

    fn set_field(&mut self, _name: &str, _value: &dyn LpValue) -> Result<(), RuntimeError> {
        Err(RuntimeError::TypeMismatch {
            #[cfg(feature = "alloc")]
            expected: alloc::string::String::from("Fixed"),
            #[cfg(not(feature = "alloc"))]
            expected: "Fixed",
            #[cfg(feature = "alloc")]
            actual: alloc::string::String::from("unknown"),
            #[cfg(not(feature = "alloc"))]
            actual: "unknown",
        })
    }

    fn field_count(&self) -> usize {
        1
    }

    fn get_field_by_index(&self, index: usize) -> Result<(&str, LpValueRef), RuntimeError> {
        // For LfoConfig, we know the shape is LFO_CONFIG_SHAPE
        let record_shape = &LFO_CONFIG_SHAPE;
        let field_shape =
            record_shape
                .get_field(index)
                .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                    index,
                    len: record_shape.field_count(),
                })?;

        let field_name = field_shape.name();
        let field_value = self.get_field(field_name)?;
        Ok((field_name, field_value))
    }
}

/// Runtime structure for an LFO node.
pub struct LfoNode {
    pub config: LfoConfig,
    pub output: Fixed,
}

impl LfoNode {
    pub fn new(config: LfoConfig) -> Self {
        Self {
            config,
            output: Fixed::ZERO,
        }
    }
}

/// Static shape for LfoNode.
const LFO_NODE_SHAPE: RecordShapeStatic = RecordShapeStatic {
    name: "LfoNode",
    fields: &[
        RecordFieldStatic {
            name: "config",
            shape: &LFO_CONFIG_SHAPE,
            meta: RecordFieldMetaStatic {
                docs: Some("LFO configuration"),
            },
        },
        RecordFieldStatic {
            name: "output",
            shape: &FIXED_SHAPE,
            meta: RecordFieldMetaStatic {
                docs: Some("LFO output value"),
            },
        },
    ],
};

use crate::kind::fixed::fixed_static::FIXED_SHAPE;

impl LpValue for LfoNode {
    fn shape(&self) -> &dyn LpShape {
        &LFO_NODE_SHAPE
    }
}

impl RecordValue for LfoNode {
    fn get_field(&self, name: &str) -> Result<LpValueRef, RuntimeError> {
        match name {
            "config" => Ok(LpValueRef::Record(&self.config as &dyn RecordValue)),
            "output" => Ok(LpValueRef::Fixed(&self.output as &dyn LpValue)),
            _ => Err(RuntimeError::FieldNotFound {
                #[cfg(feature = "alloc")]
                record_name: alloc::string::String::from("LfoNode"),
                #[cfg(not(feature = "alloc"))]
                record_name: "LfoNode",
                #[cfg(feature = "alloc")]
                field_name: alloc::string::String::from(name),
                #[cfg(not(feature = "alloc"))]
                field_name: name,
            }),
        }
    }

    fn get_field_mut(&mut self, name: &str) -> Result<LpValueRefMut, RuntimeError> {
        match name {
            "config" => Ok(LpValueRefMut::Record(
                &mut self.config as &mut dyn RecordValue,
            )),
            "output" => Ok(LpValueRefMut::Fixed(&mut self.output as &mut dyn LpValue)),
            _ => Err(RuntimeError::FieldNotFound {
                #[cfg(feature = "alloc")]
                record_name: alloc::string::String::from("LfoNode"),
                #[cfg(not(feature = "alloc"))]
                record_name: "LfoNode",
                #[cfg(feature = "alloc")]
                field_name: alloc::string::String::from(name),
                #[cfg(not(feature = "alloc"))]
                field_name: name,
            }),
        }
    }

    fn set_field(&mut self, _name: &str, _value: &dyn LpValue) -> Result<(), RuntimeError> {
        Err(RuntimeError::TypeMismatch {
            #[cfg(feature = "alloc")]
            expected: alloc::string::String::from("set_field not implemented"),
            #[cfg(not(feature = "alloc"))]
            expected: "set_field not implemented",
            #[cfg(feature = "alloc")]
            actual: alloc::string::String::from("unknown"),
            #[cfg(not(feature = "alloc"))]
            actual: "unknown",
        })
    }

    fn field_count(&self) -> usize {
        2 // config, output
    }

    fn get_field_by_index(&self, index: usize) -> Result<(&str, LpValueRef), RuntimeError> {
        // For LfoNode, we know the shape is LFO_NODE_SHAPE
        let record_shape = &LFO_NODE_SHAPE;
        let field_shape =
            record_shape
                .get_field(index)
                .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                    index,
                    len: record_shape.field_count(),
                })?;

        let field_name = field_shape.name();
        let field_value = self.get_field(field_name)?;
        Ok((field_name, field_value))
    }
}

impl Clone for LfoNode {
    fn clone(&self) -> Self {
        LfoNode {
            config: self.config.clone(),
            output: self.output,
        }
    }
}
