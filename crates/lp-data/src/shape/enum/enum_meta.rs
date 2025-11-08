//! Enum metadata and UI hints.

use crate::shape::record::RecordField;
use crate::shape::shape_ref::ShapeRef;

/// Description of an enum variant.
#[derive(Debug)]
pub enum EnumVariant {
    Unit {
        name: &'static str,
    },
    Tuple {
        name: &'static str,
        fields: &'static [ShapeRef],
    },
    Struct {
        name: &'static str,
        fields: &'static [RecordField],
    },
}

impl EnumVariant {
    pub const fn unit(name: &'static str) -> Self {
        Self::Unit { name }
    }

    pub const fn tuple(name: &'static str, fields: &'static [ShapeRef]) -> Self {
        Self::Tuple { name, fields }
    }

    pub const fn struct_variant(name: &'static str, fields: &'static [RecordField]) -> Self {
        Self::Struct { name, fields }
    }
}

/// UI hints for enums.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumUi {
    Dropdown,
    SegmentedControl,
}
