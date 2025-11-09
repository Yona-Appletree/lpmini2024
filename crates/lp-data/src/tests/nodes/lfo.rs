use lp_math::fixed::Fixed;
use serde::{Deserialize, Serialize};

use crate as lp_data;
use crate::shape::int32::LpInt32;
use crate::shape::record::RecordValue;
use crate::shape::value::LpValueTrait;
use crate::value::{LpValue, RuntimeError};
use crate::LpSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, LpSchema)]
#[lp(schema(name = "LFO Config", docs = "Configuration for an LFO node."))]
pub struct LfoConfig {
    #[lp(field(
        ui(slider(min = 10, max = 60000, step = 1)),
        docs = "Oscillation period in milliseconds."
    ))]
    pub period_ms: LpInt32,
}

impl LpValueTrait for LfoConfig {
    fn shape(&self) -> &crate::shape::shape_ref::ShapeRef {
        <Self as lp_data::LpDescribe>::lp_schema()
    }
}

impl RecordValue for LfoConfig {
    fn get_field(&self, name: &str) -> Result<&dyn LpValueTrait, RuntimeError> {
        match name {
            "period_ms" => Ok(&self.period_ms as &dyn LpValueTrait),
            _ => Err(RuntimeError::FieldNotFound {
                record_name: "LfoConfig",
                field_name: name,
            }),
        }
    }

    fn get_field_mut(&mut self, name: &str) -> Result<&mut dyn LpValueTrait, RuntimeError> {
        match name {
            "period_ms" => Ok(&mut self.period_ms as &mut dyn LpValueTrait),
            _ => Err(RuntimeError::FieldNotFound {
                record_name: "LfoConfig",
                field_name: name,
            }),
        }
    }

    fn set_field(&mut self, name: &str, value: LpValue) -> Result<(), RuntimeError> {
        match name {
            "period_ms" => {
                if let LpValue::Int32(i) = value {
                    self.period_ms = LpInt32(i);
                    Ok(())
                } else {
                    Err(RuntimeError::TypeMismatch {
                        expected: "Int32",
                        actual: "other",
                    })
                }
            }
            _ => Err(RuntimeError::FieldNotFound {
                record_name: "LfoConfig",
                field_name: name,
            }),
        }
    }
}

/// Runtime structure for an LFO node.
///
/// This represents the runtime state of an LFO node with its config and output.
#[derive(Debug, Clone, PartialEq)]
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
