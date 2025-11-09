//! LFO (Low Frequency Oscillator) node.

extern crate alloc;

use crate::kind::{
    fixed::{fixed_meta::FixedMetaStatic, fixed_static::FixedShapeStatic},
    record::{
        record_meta::RecordFieldMetaStatic,
        record_static::{RecordFieldStatic, RecordShapeStatic},
    },
    shape::LpShape,
    value::{LpValue, LpValueBox, RecordValue},
};
use crate::value::RuntimeError;
use alloc::string::String;
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
    fn get_field(&self, name: &str) -> Result<&dyn LpValue, RuntimeError> {
        match name {
            "period" => Ok(&self.period as &dyn LpValue),
            _ => Err(RuntimeError::FieldNotFound {
                record_name: String::from("LfoConfig"),
                field_name: String::from(name),
            }),
        }
    }

    fn get_field_mut(&mut self, name: &str) -> Result<&mut dyn LpValue, RuntimeError> {
        match name {
            "period" => Ok(&mut self.period as &mut dyn LpValue),
            _ => Err(RuntimeError::FieldNotFound {
                record_name: String::from("LfoConfig"),
                field_name: String::from(name),
            }),
        }
    }

    fn set_field(&mut self, _name: &str, _value: &dyn LpValue) -> Result<(), RuntimeError> {
        Err(RuntimeError::TypeMismatch {
            expected: String::from("Fixed"),
            actual: String::from("unknown"),
        })
    }

    fn field_count(&self) -> usize {
        1
    }

    fn iter_fields(&self) -> crate::kind::value::VecIntoIter<(String, LpValueBox)> {
        let mut fields = Vec::new();
        fields.push((String::from("period"), LpValueBox::from(self.period)));
        fields.into_iter()
    }

    fn clone_box(&self) -> Box<dyn RecordValue> {
        Box::new(self.clone())
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
    fn get_field(&self, name: &str) -> Result<&dyn LpValue, RuntimeError> {
        match name {
            "config" => Ok(&self.config as &dyn LpValue),
            "output" => Ok(&self.output as &dyn LpValue),
            _ => Err(RuntimeError::FieldNotFound {
                record_name: String::from("LfoNode"),
                field_name: String::from(name),
            }),
        }
    }

    fn get_field_mut(&mut self, name: &str) -> Result<&mut dyn LpValue, RuntimeError> {
        match name {
            "config" => Ok(&mut self.config as &mut dyn LpValue),
            "output" => Ok(&mut self.output as &mut dyn LpValue),
            _ => Err(RuntimeError::FieldNotFound {
                record_name: String::from("LfoNode"),
                field_name: String::from(name),
            }),
        }
    }

    fn set_field(&mut self, _name: &str, _value: &dyn LpValue) -> Result<(), RuntimeError> {
        Err(RuntimeError::TypeMismatch {
            expected: String::from("set_field not implemented"),
            actual: String::from("unknown"),
        })
    }

    fn field_count(&self) -> usize {
        2 // config, output
    }

    fn iter_fields(&self) -> crate::kind::value::VecIntoIter<(String, LpValueBox)> {
        let mut fields = Vec::new();
        // Box the config as a RecordValue
        fields.push((
            String::from("config"),
            LpValueBox::from(Box::new(self.config.clone()) as Box<dyn RecordValue>),
        ));
        fields.push((String::from("output"), LpValueBox::from(self.output)));
        fields.into_iter()
    }

    fn clone_box(&self) -> Box<dyn RecordValue> {
        Box::new(self.clone())
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
