//! LFO (Low Frequency Oscillator) node.

#[cfg(feature = "alloc")]
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

    fn get_field_mut(&mut self, name: &str) -> Result<&mut dyn LpValue, RuntimeError> {
        match name {
            "period" => Ok(&mut self.period as &mut dyn LpValue),
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

    #[cfg(feature = "alloc")]
    fn iter_fields(&self) -> alloc::vec::IntoIter<(alloc::string::String, LpValueBox)> {
        let mut fields = alloc::vec::Vec::new();
        fields.push((
            alloc::string::String::from("period"),
            LpValueBox::from(self.period),
        ));
        fields.into_iter()
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

    fn get_field_mut(&mut self, name: &str) -> Result<&mut dyn LpValue, RuntimeError> {
        match name {
            "config" => Ok(&mut self.config as &mut dyn LpValue),
            "output" => Ok(&mut self.output as &mut dyn LpValue),
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

    #[cfg(feature = "alloc")]
    fn iter_fields(&self) -> alloc::vec::IntoIter<(alloc::string::String, LpValueBox)> {
        let mut fields = alloc::vec::Vec::new();
        // Box the config as a RecordValue
        let config_ref: &dyn RecordValue = &self.config;
        #[allow(deprecated)]
        let config_boxed = lp_pool::LpBoxDyn::try_new_unsized(config_ref)
            .expect("Failed to allocate config in pool");
        fields.push((
            alloc::string::String::from("config"),
            LpValueBox::from(config_boxed),
        ));
        fields.push((
            alloc::string::String::from("output"),
            LpValueBox::from(self.output),
        ));
        fields.into_iter()
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
