//! LFO (Low Frequency Oscillator) node.

use crate::kind::{
    fixed::{fixed_meta::FixedMetaStatic, fixed_static::FixedShapeStatic},
    record::{
        record_meta::{RecordFieldMetaStatic, RecordMetaStatic},
        record_shape::RecordShape,
        record_static::{RecordFieldStatic, RecordShapeStatic},
    },
    shape::LpShape,
    value::{LpValue, LpValueRef, LpValueRefMut},
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
    meta: RecordMetaStatic {
        name: "LfoConfig",
        docs: None,
    },
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
    fn shape(&self) -> &dyn RecordShape {
        &LFO_CONFIG_SHAPE
    }

    fn get_field(&self, name: &str) -> Result<LpValueRef<'_>, RuntimeError> {
        match name {
            "period" => Ok(LpValueRef::Fixed(&self.period as &dyn LpValue)),
            _ => Err(RuntimeError::field_not_found("LfoConfig", name)),
        }
    }

    fn get_field_mut(&mut self, name: &str) -> Result<LpValueRefMut<'_>, RuntimeError> {
        match name {
            "period" => Ok(LpValueRefMut::Fixed(&mut self.period as &mut dyn LpValue)),
            _ => Err(RuntimeError::field_not_found("LfoConfig", name)),
        }
    }

    fn set_field(&mut self, _name: &str, _value: &dyn LpValue) -> Result<(), RuntimeError> {
        Err(RuntimeError::type_mismatch("Fixed", "unknown"))
    }

    fn field_count(&self) -> usize {
        1
    }

    fn get_field_by_index(&self, index: usize) -> Result<(&str, LpValueRef<'_>), RuntimeError> {
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
    meta: RecordMetaStatic {
        name: "LfoNode",
        docs: None,
    },
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
use crate::kind::record::record_value::RecordValue;

impl LpValue for LfoNode {
    fn shape(&self) -> &dyn LpShape {
        &LFO_NODE_SHAPE
    }
}

impl RecordValue for LfoNode {
    fn shape(&self) -> &dyn RecordShape {
        &LFO_NODE_SHAPE
    }

    fn get_field(&self, name: &str) -> Result<LpValueRef<'_>, RuntimeError> {
        match name {
            "config" => Ok(LpValueRef::Record(&self.config as &dyn RecordValue)),
            "output" => Ok(LpValueRef::Fixed(&self.output as &dyn LpValue)),
            _ => Err(RuntimeError::field_not_found("LfoNode", name)),
        }
    }

    fn get_field_mut(&mut self, name: &str) -> Result<LpValueRefMut<'_>, RuntimeError> {
        match name {
            "config" => Ok(LpValueRefMut::Record(
                &mut self.config as &mut dyn RecordValue,
            )),
            "output" => Ok(LpValueRefMut::Fixed(&mut self.output as &mut dyn LpValue)),
            _ => Err(RuntimeError::field_not_found("LfoNode", name)),
        }
    }

    fn set_field(&mut self, _name: &str, _value: &dyn LpValue) -> Result<(), RuntimeError> {
        Err(RuntimeError::type_mismatch(
            "set_field not implemented",
            "unknown",
        ))
    }

    fn field_count(&self) -> usize {
        2 // config, output
    }

    fn get_field_by_index(&self, index: usize) -> Result<(&str, LpValueRef<'_>), RuntimeError> {
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
