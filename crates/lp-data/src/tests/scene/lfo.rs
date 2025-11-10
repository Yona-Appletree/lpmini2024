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

    fn get_field_by_index(&self, index: usize) -> Result<LpValueRef<'_>, RuntimeError> {
        match index {
            0 => Ok(LpValueRef::Fixed(&self.period as &dyn LpValue)),
            _ => Err(RuntimeError::IndexOutOfBounds {
                index,
                len: LFO_CONFIG_SHAPE.field_count(),
            }),
        }
    }

    fn get_field_by_index_mut(&mut self, index: usize) -> Result<LpValueRefMut<'_>, RuntimeError> {
        match index {
            0 => Ok(LpValueRefMut::Fixed(&mut self.period as &mut dyn LpValue)),
            _ => Err(RuntimeError::IndexOutOfBounds {
                index,
                len: LFO_CONFIG_SHAPE.field_count(),
            }),
        }
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

    fn get_field_by_index(&self, index: usize) -> Result<LpValueRef<'_>, RuntimeError> {
        match index {
            0 => Ok(LpValueRef::Record(&self.config as &dyn RecordValue)),
            1 => Ok(LpValueRef::Fixed(&self.output as &dyn LpValue)),
            _ => Err(RuntimeError::IndexOutOfBounds {
                index,
                len: LFO_NODE_SHAPE.field_count(),
            }),
        }
    }

    fn get_field_by_index_mut(&mut self, index: usize) -> Result<LpValueRefMut<'_>, RuntimeError> {
        match index {
            0 => Ok(LpValueRefMut::Record(
                &mut self.config as &mut dyn RecordValue,
            )),
            1 => Ok(LpValueRefMut::Fixed(&mut self.output as &mut dyn LpValue)),
            _ => Err(RuntimeError::IndexOutOfBounds {
                index,
                len: LFO_NODE_SHAPE.field_count(),
            }),
        }
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
